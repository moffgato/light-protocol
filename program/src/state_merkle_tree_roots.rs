use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    log::sol_log_compute_units,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use std::convert::TryInto;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
pub const MERKLE_TREE_ACC_BYTES: [u8; 32] = [
    222, 66, 10, 195, 58, 162, 229, 40, 247, 92, 17, 93, 85, 233, 85, 138, 197, 136, 2, 65, 208,
    158, 38, 39, 155, 208, 117, 251, 244, 33, 72, 213,
];
#[derive(Clone, Debug)]
pub struct MerkleTreeRoots {
    pub is_initialized: bool,
    pub roots: Vec<u8>,
    pub ROOT_HISTORY_SIZE: u64,
}

impl Sealed for MerkleTreeRoots {}
impl IsInitialized for MerkleTreeRoots {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MerkleTreeRoots {
    const LEN: usize = 16657;

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, MerkleTreeRoots::LEN];

        let (
            is_initialized,
            levels,
            filledSubtrees,
            currentRootIndex,
            nextIndex,
            ROOT_HISTORY_SIZE,
            //609
            roots,
            //18137
            unused_remainder
        ) = array_refs![input, 1, 8, 576, 8, 8, 8, 16000, 48];

        if is_initialized[0] != 1u8 {
            msg!("Merkle Tree is not initialized");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(MerkleTreeRoots {
            is_initialized: true,
            roots: roots.to_vec(),
            ROOT_HISTORY_SIZE: u64::from_le_bytes(*ROOT_HISTORY_SIZE),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        //is not meant to be called since this structs purpose is to solely unpack roots
        //to check for the existence of one root
    }
}

pub fn check_root_hash_exists(
    account_main: &AccountInfo,
    root_bytes: &Vec<u8>,
    //found_root: &mut u8,
) -> Result<u8,ProgramError>{
    let mut account_main_data = MerkleTreeRoots::unpack(&account_main.data.borrow()).unwrap();
    msg!("merkletree acc key: {:?}", *account_main.key);
    msg!(
        "key to check: {:?}",
        solana_program::pubkey::Pubkey::new(&MERKLE_TREE_ACC_BYTES[..])
    );
    // assert_eq!(
    //     *account_main.key,
    //     solana_program::pubkey::Pubkey::new(&MERKLE_TREE_ACC_BYTES[..])
    // );


    if *account_main.key != solana_program::pubkey::Pubkey::new(&MERKLE_TREE_ACC_BYTES[..]) {
        msg!("merkle tree account is incorrect");
        return Err(ProgramError::IllegalOwner);
    }
    msg!("did not crash {}", account_main_data.ROOT_HISTORY_SIZE);
    // assert!(
    //     account_main_data.ROOT_HISTORY_SIZE < 593,
    //     "root history size too large"
    // );
    if account_main_data.ROOT_HISTORY_SIZE > 593 {
        msg!("root history size too large");
        return Err(ProgramError::InvalidAccountData);
    }
    msg!("looking for root {:?}", *root_bytes);
    let mut found_root = 0;
    let mut i = 0;
    let mut counter = 0;
    loop {
        //sol_log_compute_units();
        if account_main_data.roots[i..i + 32] == *root_bytes {
            msg!("found root hash index {}", counter);
            found_root = 1u8;
            break;
        }

        if counter % 10 == 0 {
            msg!("{}", counter);
        }
        i += 32;
        counter += 1;
        if counter == account_main_data.ROOT_HISTORY_SIZE {
            msg!("did not find root should panic here but is disabled for testing");
            //panic!("did not find root");
            //return Err(ProgramError::InvalidAccountData);
            break;
        }
    }
    Ok(found_root)
}
