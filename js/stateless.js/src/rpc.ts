import {
    Connection,
    ConnectionConfig,
    SolanaJSONRPCError,
    PublicKey,
} from '@solana/web3.js';
import {
    BalanceResult,
    CompressedAccountResult,
    CompressedAccountsByOwnerResult,
    CompressedProofWithContext,
    CompressedTokenAccountsByOwnerOrDelegateResult,
    CompressionApiInterface,
    GetCompressedTokenAccountsByOwnerOrDelegateOptions,
    HealthResult,
    MerkeProofResult,
    MultipleCompressedAccountsResult,
    ParsedTokenAccount,
    SlotResult,
    jsonRpcResult,
    jsonRpcResultAndContext,
} from './rpc-interface';
import {
    MerkleContextWithMerkleProof,
    BN254,
    bn,
    CompressedAccountWithMerkleContext,
    createBN254,
    encodeBN254toBase58,
    createCompressedAccountWithMerkleContext,
    createMerkleContext,
    TokenData,
} from './state';
import { array, create, nullable } from 'superstruct';
import { toCamelCase } from './utils/conversion';
import { defaultTestStateTreeAccounts } from './constants';
import { BN } from '@coral-xyz/anchor';
/// FIXME: this is a circular dependency:
/// implement getValidityProof directly in RPC
import { getTestRpc } from './test-utils/test-rpc';
import { Buffer } from 'buffer';

export function createRpc(
    endpointOrWeb3JsConnection: string | Connection = 'http://127.0.0.1:8899',
    compressionApiEndpoint: string = 'http://localhost:8784',
    config?: ConnectionConfig,
): Rpc {
    if (typeof endpointOrWeb3JsConnection === 'string') {
        return new Rpc(
            endpointOrWeb3JsConnection,
            compressionApiEndpoint,
            undefined,
            config,
        );
    }
    return new Rpc(
        endpointOrWeb3JsConnection.rpcEndpoint,
        compressionApiEndpoint,
        undefined,
        config,
    );
}

const rpcRequest = async (
    rpcEndpoint: string,
    method: string,
    params: any = [], // TODO: array?
    convertToCamelCase = true,
): Promise<any> => {
    const body = JSON.stringify({
        jsonrpc: '2.0',
        id: 'test-account',
        method: method,
        params: params,
    });

    const response = await fetch(rpcEndpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: body,
    });

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    if (convertToCamelCase) {
        const res = await response.json();
        return toCamelCase(res);
    }
    return await response.json();
};

const mockNullifierQueue = defaultTestStateTreeAccounts().nullifierQueue;

export class Rpc extends Connection implements CompressionApiInterface {
    /// TODO: can photon expose the default methods as well?
    compressionApiEndpoint: string;
    constructor(
        endpoint: string,
        compressionApiEndpoint: string,
        // TODO: implement
        proverEndpoint?: string,
        config?: ConnectionConfig,
    ) {
        super(endpoint, config || 'confirmed');
        this.compressionApiEndpoint = compressionApiEndpoint;
    }

    async getCompressedAccount(
        hash: BN254,
    ): Promise<CompressedAccountWithMerkleContext | null> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getCompressedAccount',
            { hash: encodeBN254toBase58(hash) },
        );
        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(nullable(CompressedAccountResult)),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get info for compressed account ${hash.toString()}`,
            );
        }
        if (res.result.value === null) {
            return null;
        }
        const item = res.result.value;
        const account = createCompressedAccountWithMerkleContext(
            createMerkleContext(
                item.tree!,
                mockNullifierQueue,
                item.hash.toArray(),
                item.leafIndex,
            ),
            item.owner,
            bn(item.lamports),
            // TODO: fix. add typesafety to the rest
            item.data
                ? {
                      discriminator: item.discriminator.toArray('le'),
                      data: Buffer.from(item.data, 'base64'),
                      dataHash: item.dataHash!.toArray('le'), //FIXME: need to calculate the hash or return from server
                  }
                : undefined,

            item.address || undefined,
        );
        return account;
    }

    async getCompressedBalance(hash: BN254): Promise<BN | null> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getCompressedBalance',
            { hash: encodeBN254toBase58(hash) },
        );
        const res = create(unsafeRes, jsonRpcResultAndContext(BalanceResult));
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get balance for compressed account ${hash.toString()}`,
            );
        }
        if (res.result.value === null) {
            return null;
        }

        return bn(res.result.value);
    }

    /** Retrieve the merkle proof for a compressed account */
    async getCompressedAccountProof(
        hash: BN254,
    ): Promise<MerkleContextWithMerkleProof> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getCompressedAccountProof',
            encodeBN254toBase58(hash),
        );
        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(MerkeProofResult),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get proof for compressed account ${hash.toString()}`,
            );
        }
        if (res.result.value === null) {
            throw new Error(
                `failed to get proof for compressed account ${hash.toString()}`,
            );
        }

        const value: MerkleContextWithMerkleProof = {
            hash: res.result.value.hash.toArray(),
            merkleTree: res.result.value.merkleTree,
            leafIndex: res.result.value.leafIndex,
            merkleProof: res.result.value.proof,
            nullifierQueue: mockNullifierQueue,
            rootIndex: 0, // TODO: add root index
        };
        return value;
    }

    async getMultipleCompressedAccounts(
        hashes: BN254[],
    ): Promise<CompressedAccountWithMerkleContext[] | null> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getMultipleCompressedAccounts',
            hashes.map(hash => encodeBN254toBase58(hash)),
        );
        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(MultipleCompressedAccountsResult),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get info for compressed accounts ${hashes.map(hash => encodeBN254toBase58(hash)).join(', ')}`,
            );
        }
        if (res.result.value === null) {
            return null;
        }
        const accounts: CompressedAccountWithMerkleContext[] = [];
        res.result.value.items.map((item: any) => {
            const account = createCompressedAccountWithMerkleContext(
                createMerkleContext(
                    item.tree!,
                    mockNullifierQueue,
                    item.hash.toArray(),
                    item.leafIndex,
                ),
                item.owner,
                bn(item.lamports),
                item.data && {
                    /// TODO: validate whether we need to convert to 'le' here
                    discriminator: item.discriminator.toArray('le'),
                    data: Buffer.from(item.data, 'base64'),
                    dataHash: item.dataHash.toArray('le'), //FIXME: need to calculate the hash or return from server
                },
                item.address,
            );
            accounts.push(account);
        });

        return accounts;
    }

    /** Retrieve the merkle proof for a compressed account */
    async getMultipleCompressedAccountProofs(
        hashes: BN254[],
    ): Promise<MerkleContextWithMerkleProof[] | null> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getMultipleCompressedAccountProofs',
            hashes.map(hash => encodeBN254toBase58(hash)),
        );
        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(array(MerkeProofResult)),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get proofs for compressed accounts ${hashes.map(hash => encodeBN254toBase58(hash)).join(', ')}`,
            );
        }
        if (res.result.value === null) {
            return null;
        }

        const merkleProofs: MerkleContextWithMerkleProof[] = [];

        res.result.value.map((proof: any) => {
            const value: MerkleContextWithMerkleProof = {
                hash: proof.hash.toArray(), // FIXME
                merkleTree: proof.merkleTree,
                leafIndex: proof.leafIndex,
                merkleProof: proof.proof.map((proof: any) =>
                    createBN254(proof),
                ),
                nullifierQueue: mockNullifierQueue,
                rootIndex: 0, // TODO: add root index
            };
            merkleProofs.push(value);
        });

        return merkleProofs;
    }

    async getCompressedAccountsByOwner(
        owner: PublicKey,
    ): Promise<CompressedAccountWithMerkleContext[]> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getCompressedAccountsByOwner',
            { owner: owner.toBase58() },
        );

        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(CompressedAccountsByOwnerResult),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get info for compressed accounts owned by ${owner.toBase58()}`,
            );
        }
        if (res.result.value === null) {
            return [];
        }
        const accounts: CompressedAccountWithMerkleContext[] = [];
        /// TODO: clean up. Make typesafe
        res.result.value.items.map((item: any) => {
            const account = createCompressedAccountWithMerkleContext(
                createMerkleContext(
                    item.tree!,
                    mockNullifierQueue,
                    item.hash.toArray(),
                    item.leafIndex,
                ),
                item.owner,
                bn(item.lamports),
                item.data && {
                    discriminator: item.discriminator.toArray('le'),
                    data: Buffer.from(item.data, 'base64'),
                    dataHash: item.dataHash.toArray('le'), //FIXME: need to calculate the hash or return from server
                },
                item.address,
            );

            accounts.push(account);
        });

        return accounts;
    }

    /// TODO: Implement self
    async getValidityProof(
        hashes: BN254[],
    ): Promise<CompressedProofWithContext> {
        const rpc = await getTestRpc();
        const proof = await rpc.getValidityProof(hashes);
        return proof;
    }

    async getHealth(): Promise<string> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getHealth',
        );
        const res = create(unsafeRes, jsonRpcResult(HealthResult));
        if ('error' in res) {
            throw new SolanaJSONRPCError(res.error, 'failed to get health');
        }
        return res.result;
    }

    /** TODO: use from Connection */
    async getSlot(): Promise<number> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getSlot',
        );
        const res = create(unsafeRes, jsonRpcResult(SlotResult));
        if ('error' in res) {
            throw new SolanaJSONRPCError(res.error, 'failed to get slot');
        }
        return res.result;
    }

    async getCompressedTokenAccountsByOwner(
        owner: PublicKey,
        options?: GetCompressedTokenAccountsByOwnerOrDelegateOptions,
    ): Promise<ParsedTokenAccount[]> {
        const unsafeRes = await rpcRequest(
            this.compressionApiEndpoint,
            'getCompressedTokenAccountsByOwner',
            { owner: owner.toBase58(), mint: options?.mint?.toBase58() },
        );
        const res = create(
            unsafeRes,
            jsonRpcResultAndContext(
                CompressedTokenAccountsByOwnerOrDelegateResult,
            ),
        );
        if ('error' in res) {
            throw new SolanaJSONRPCError(
                res.error,
                `failed to get info for compressed accounts owned by ${owner.toBase58()}`,
            );
        }
        if (res.result.value === null) {
            throw new Error('not implemented. NULL result');
        }
        const accounts: ParsedTokenAccount[] = [];
        /// TODO: clean up. Make typesafe
        res.result.value.items.map((item: any) => {
            const account = createCompressedAccountWithMerkleContext(
                createMerkleContext(
                    item.tree!,
                    mockNullifierQueue,
                    item.hash.toArray(),
                    item.leafIndex,
                ),
                new PublicKey('9sixVEthz2kMSKfeApZXHwuboT6DZuT6crAYJTciUCqE'), // TODO: photon should return programOwner
                bn(item.lamports),
                item.data && {
                    discriminator: item.discriminator.toArray('le'),
                    data: Buffer.from(item.data, 'base64'),
                    dataHash: item.dataHash.toArray('le'), //FIXME: need to calculate the hash or return from server
                },
                item.address,
            );

            const tokenData: TokenData = {
                mint: item.mint,
                owner: item.owner,
                amount: item.amount,
                delegate: item.delegate,
                state: 1, // TODO: dynamic
                isNative: null, // TODO: dynamic
                delegatedAmount: bn(0), // TODO: dynamic
            };

            accounts.push({
                compressedAccount: account,
                parsed: tokenData,
            });
        });

        /// TODO: consider custom sort. we're returning most recent first
        /// because thats how our tests expect it currently
        return accounts.sort(
            (a, b) =>
                b.compressedAccount.leafIndex - a.compressedAccount.leafIndex,
        );
    }

    /// TODO: implement delegate
    async getCompressedTokenAccountsByDelegate(
        delegate: PublicKey,
        options?: GetCompressedTokenAccountsByOwnerOrDelegateOptions,
    ): Promise<ParsedTokenAccount[]> {
        throw new Error('Method not implemented.');
    }
    async getCompressedTokenAccountBalance(
        hash: BN254,
    ): Promise<{ amount: BN }> {
        throw new Error('Method not implemented.');
    }
}
