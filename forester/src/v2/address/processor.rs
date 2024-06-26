use crate::config::ForesterConfig;
use crate::errors::ForesterError;
use crate::v2::address::pipeline::{AddressPipelineStage, PipelineContext};
use crate::v2::BackpressureControl;
use account_compression::{AddressMerkleTreeAccount, QueueAccount};
use light_hash_set::HashSet;
use light_hasher::Poseidon;
use light_registry::sdk::{
    create_update_address_merkle_tree_instruction, UpdateAddressMerkleTreeInstructionInputs,
};
use light_test_utils::get_indexed_merkle_tree;
use light_test_utils::indexer::{Indexer, NewAddressProofWithContext};
use light_test_utils::rpc::rpc_connection::RpcConnection;
use log::{info, warn};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{mpsc, Mutex};

pub struct AddressProcessor<T: Indexer, R: RpcConnection> {
    pub input: mpsc::Receiver<AddressPipelineStage<T, R>>,
    pub output: mpsc::Sender<AddressPipelineStage<T, R>>,
    pub backpressure: BackpressureControl,
    pub shutdown: Arc<AtomicBool>,
    pub close_output: mpsc::Receiver<()>,
}

impl<T: Indexer, R: RpcConnection> AddressProcessor<T, R> {

    pub(crate) async fn process(&mut self) {
        info!("Starting AddressProcessor process");
        loop {
            tokio::select! {
            Some(item) = self.input.recv() => {
                info!("Received item in AddressProcessor");
                let _permit = self.backpressure.acquire().await;
                let result = match item {
                    AddressPipelineStage::FetchAddressQueueData(ref context) => {
                        self.fetch_address_queue_data(context).await
                    }
                    AddressPipelineStage::ProcessAddressQueue(ref context, ref queue_data) => {
                        self.process_address_queue(context.clone(), queue_data.clone()).await
                    }
                    AddressPipelineStage::UpdateAddressMerkleTree(ref context, ref account) => {
                        self.update_address_merkle_tree(context.clone(), account.clone()).await
                    }
                    AddressPipelineStage::UpdateIndexer(ref context, ref proof) => {
                            let proof = *(proof.clone());
                            self.update_indexer(context.clone(), proof).await
                    }
                };

                match result {
                    Ok(next_stages) => {
                        for next_stage in next_stages {
                            if let Err(e) = self.output.send(next_stage).await {
                                warn!("Failed to send next stage to output: {:?}", e);
                                // If we can't send, the receiver is probably closed. We should stop.
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error in AddressProcessor: {:?}", e);
                        // Instead of breaking, let's continue to the next item
                        let context = item.context();
                        if let Err(e) = self.output.send(AddressPipelineStage::FetchAddressQueueData(context)).await {
                            warn!("Failed to send FetchAddressQueueData stage after error: {:?}", e);
                            // If we can't send, the receiver is probably closed. We should stop.
                            return;
                        }
                    }
                }
            }
            _ = self.close_output.recv() => {
                info!("Received signal to close output channel");
                break;
            }
            else => break,
        }

            if self.shutdown.load(Ordering::Relaxed) {
                info!("Shutdown signal received, stopping AddressProcessor");
                break;
            }
        }
        info!("AddressProcessor process completed");
    }

    async fn fetch_address_queue_data(
        &self,
        context: &PipelineContext<T, R>,
    ) -> Result<Vec<AddressPipelineStage<T, R>>, ForesterError> {
        let config = &context.config;
        let rpc = &context.rpc;

        let queue_data = fetch_address_queue_data(config, rpc).await?;
        if queue_data.is_empty() {
            info!("Address queue is empty");
            Ok(vec![AddressPipelineStage::ProcessAddressQueue(
                context.clone(),
                vec![],
            )])
        } else {
            Ok(vec![AddressPipelineStage::ProcessAddressQueue(
                context.clone(),
                queue_data,
            )])
        }
    }
    
    async fn process_address_queue(
        &self,
        context: PipelineContext<T, R>,
        queue_data: Vec<crate::v2::address::Account>,
    ) -> Result<Vec<AddressPipelineStage<T, R>>, ForesterError> {
        let mut next_stages = Vec::new();
        for account in queue_data {
            next_stages.push(AddressPipelineStage::UpdateAddressMerkleTree(
                context.clone(),
                account,
            ));
        }
        Ok(next_stages)
    }

    async fn update_address_merkle_tree(
        &self,
        context: PipelineContext<T, R>,
        account: crate::v2::address::Account,
    ) -> Result<Vec<AddressPipelineStage<T, R>>, ForesterError> {
        let config = &context.config;
        let rpc = &context.rpc;
        let indexer = &context.indexer;

        let address = account.hash;
        let address_hashset_index = account.index;

        let proof_result = indexer
            .lock()
            .await
            .get_multiple_new_address_proofs(config.address_merkle_tree_pubkey.to_bytes(), address)
            .await;

        match proof_result {
            Ok(proof) => {
                let mut retry_count = 0;
                let max_retries = 3;

                while retry_count < max_retries {
                    match update_merkle_tree(
                        rpc,
                        &config.payer_keypair,
                        config.address_merkle_tree_queue_pubkey,
                        config.address_merkle_tree_pubkey,
                        address_hashset_index as u16,
                        proof.low_address_index,
                        proof.low_address_value,
                        proof.low_address_next_index,
                        proof.low_address_next_value,
                        proof.low_address_proof,
                    )
                        .await {
                        Ok(true) => {
                            info!("Successfully updated merkle tree for address: {:?}", address);
                            return Ok(vec![AddressPipelineStage::FetchAddressQueueData(context)]);
                        }
                        Ok(false) => {
                            warn!("Failed to update merkle tree for address: {:?}", address);
                            retry_count += 1;
                        }
                        Err(e) => {
                            warn!("Error updating merkle tree for address {:?}: {:?}", address, e);
                            retry_count += 1;
                        }
                    }

                    if retry_count < max_retries {
                        info!("Retrying update for address: {:?} (Attempt {} of {})", address, retry_count + 1, max_retries);
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }
                }
                warn!("Max retries reached for address: {:?}. Moving to next address.", address);
            }
            Err(e) => {
                warn!("Failed to get address tree proof for address {:?}: {:?}", address, e);
            }
        }
        Ok(vec![AddressPipelineStage::FetchAddressQueueData(context)])
    }

    async fn update_indexer(
        &self,
        context: PipelineContext<T, R>,
        proof: NewAddressProofWithContext,
    ) -> Result<Vec<AddressPipelineStage<T, R>>, ForesterError> {
        let config = &context.config;
        let indexer = &context.indexer;

        indexer
            .lock()
            .await
            .address_tree_updated(config.address_merkle_tree_pubkey.to_bytes(), proof);

        Ok(vec![AddressPipelineStage::FetchAddressQueueData(context)])
    }
}

async fn fetch_address_queue_data<R: RpcConnection>(
    config: &Arc<ForesterConfig>,
    rpc: &Arc<Mutex<R>>,
) -> Result<Vec<crate::v2::address::Account>, ForesterError> {
    let address_queue_pubkey = config.address_merkle_tree_queue_pubkey;

    let mut account = (*rpc.lock().await)
        .get_account(address_queue_pubkey)
        .await?
        .unwrap();
    let address_queue: HashSet = unsafe {
        HashSet::from_bytes_copy(&mut account.data[8 + mem::size_of::<QueueAccount>()..])?
    };
    let mut address_queue_vec = Vec::new();

    for i in 0..address_queue.capacity {
        let bucket = address_queue.get_bucket(i).unwrap();
        if let Some(bucket) = bucket {
            if bucket.sequence_number.is_none() {
                address_queue_vec.push(crate::v2::address::Account {
                    hash: bucket.value_bytes(),
                    index: i,
                });
            }
        }
    }

    Ok(address_queue_vec)
}

#[allow(clippy::too_many_arguments)]
pub async fn update_merkle_tree<R: RpcConnection>(
    rpc: &Arc<Mutex<R>>,
    payer: &Keypair,
    address_queue_pubkey: Pubkey,
    address_merkle_tree_pubkey: Pubkey,
    value: u16,
    low_address_index: u64,
    low_address_value: [u8; 32],
    low_address_next_index: u64,
    low_address_next_value: [u8; 32],
    low_address_proof: [[u8; 32]; 16],
) -> Result<bool, ForesterError> {
    info!("update_merkle_tree");
    let (changelog_index, indexed_changelog_index) =
        get_changelog_indices(&address_merkle_tree_pubkey, &mut *rpc.lock().await)
            .await
            .unwrap();
    info!("changelog_index: {:?}", changelog_index);

    let update_ix =
        create_update_address_merkle_tree_instruction(UpdateAddressMerkleTreeInstructionInputs {
            authority: payer.pubkey(),
            address_merkle_tree: address_merkle_tree_pubkey,
            address_queue: address_queue_pubkey,
            value,
            low_address_index,
            low_address_value,
            low_address_next_index,
            low_address_next_value,
            low_address_proof,
            changelog_index: changelog_index as u16,
            indexed_changelog_index: indexed_changelog_index as u16,
        });
    info!("sending transaction...");

    let rpc = &mut *rpc.lock().await;
    let transaction = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&payer.pubkey()),
        &[&payer],
        rpc.get_latest_blockhash().await.unwrap(),
    );

    let signature = rpc.process_transaction(transaction).await?;
    info!("signature: {:?}", signature);
    let confirmed = rpc.confirm_transaction(signature).await?;
    info!("confirmed: {:?}", confirmed);
    Ok(confirmed)
}

pub async fn get_changelog_indices<R: RpcConnection>(
    merkle_tree_pubkey: &Pubkey,
    client: &mut R,
) -> Result<(usize, usize), ForesterError> {
    let merkle_tree = get_indexed_merkle_tree::<AddressMerkleTreeAccount, R, Poseidon, usize, 26>(
        client,
        *merkle_tree_pubkey,
    )
        .await;
    let changelog_index = merkle_tree.changelog_index();
    let indexed_changelog_index = merkle_tree.indexed_changelog_index();
    Ok((changelog_index, indexed_changelog_index))
}
