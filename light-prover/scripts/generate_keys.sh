#!/usr/bin/env bash

DEPTH="26"

gnark() {
    local args=("$@")
    ./light-prover "${args[@]}"
}

generate() {
    local UPDATE_BATCH_SIZE=$1
    local INSERTION_BATCH_SIZE=$2
    local INCLUSION_COMPRESSED_ACCOUNTS=$3
    local NON_INCLUSION_COMPRESSED_ACCOUNTS=$4
    local CIRCUIT_TYPE=$5
    mkdir -p circuits
    if [ "$CIRCUIT_TYPE" == "insertion" ]; then
            COMPRESSED_ACCOUNTS=$INSERTION_BATCH_SIZE
            CIRCUIT_TYPE_RS="insertion"
    elif [ "$CIRCUIT_TYPE" == "update" ]; then
            COMPRESSED_ACCOUNTS=$UPDATE_BATCH_SIZE
            CIRCUIT_TYPE_RS="update"
    elif [ "$CIRCUIT_TYPE" == "inclusion" ]; then
        COMPRESSED_ACCOUNTS=$INCLUSION_COMPRESSED_ACCOUNTS
        CIRCUIT_TYPE_RS="inclusion"
    elif [ "$CIRCUIT_TYPE" == "non-inclusion" ]; then
        COMPRESSED_ACCOUNTS=$NON_INCLUSION_COMPRESSED_ACCOUNTS
        # rust file names cannot include dashes
        CIRCUIT_TYPE_RS="non_inclusion"
    else
        COMPRESSED_ACCOUNTS="${INCLUSION_COMPRESSED_ACCOUNTS}_${NON_INCLUSION_COMPRESSED_ACCOUNTS}"
        CIRCUIT_TYPE_RS="combined"
    fi
    CIRCUIT_FILE="./proving-keys/${CIRCUIT_TYPE}_${DEPTH}_${COMPRESSED_ACCOUNTS}.key"
    CIRCUIT_VKEY_FILE="./proving-keys/${CIRCUIT_TYPE}_${DEPTH}_${COMPRESSED_ACCOUNTS}.vkey"
    CIRCUIT_VKEY_RS_FILE="../circuit-lib/verifier/src/verifying_keys/${CIRCUIT_TYPE_RS}_${DEPTH}_${COMPRESSED_ACCOUNTS}.rs"

    echo "Generating ${CIRCUIT_TYPE} circuit for ${COMPRESSED_ACCOUNTS} COMPRESSED_ACCOUNTS..."
    echo "go run . setup \
      --circuit ${CIRCUIT_TYPE} \
      --inclusion-compressed-accounts ${INCLUSION_COMPRESSED_ACCOUNTS} \
      --non-inclusion-compressed-accounts ${NON_INCLUSION_COMPRESSED_ACCOUNTS} \
      --inclusion-tree-depth ${DEPTH} \
      --non-inclusion-tree-depth ${DEPTH} \
      --insertion-batch-size ${INSERTION_BATCH_SIZE} \
      --insertion-tree-depth ${DEPTH} \
      --update-batch-size ${UPDATE_BATCH_SIZE} \
      --update-tree-depth ${DEPTH} \
      --output ${CIRCUIT_FILE} \
      --output-vkey ${CIRCUIT_VKEY_FILE}"

    gnark setup \
      --circuit "${CIRCUIT_TYPE}" \
      --inclusion-compressed-accounts "$INCLUSION_COMPRESSED_ACCOUNTS" \
      --non-inclusion-compressed-accounts "$NON_INCLUSION_COMPRESSED_ACCOUNTS" \
      --inclusion-tree-depth "$DEPTH" \
      --non-inclusion-tree-depth "$DEPTH" \
      --insertion-batch-size "$INSERTION_BATCH_SIZE" \
      --insertion-tree-depth "$DEPTH" \
      --update-batch-size "$UPDATE_BATCH_SIZE" \
      --update-tree-depth "$DEPTH" \
      --output "${CIRCUIT_FILE}" \
      --output-vkey "${CIRCUIT_VKEY_FILE}"
    cargo xtask generate-vkey-rs --input-path "${CIRCUIT_VKEY_FILE}" --output-path "${CIRCUIT_VKEY_RS_FILE}"
}

declare -a update_batch_sizes_arr=("1" "2" "4" "8")
for batch_size in "${update_batch_sizes_arr[@]}"
do
    generate "$batch_size" "0" "0" "0" "update"
done

declare -a insertion_batch_sizes_arr=("1" "2" "4" "8")
for batch_size in "${insertion_batch_sizes_arr[@]}"
do
    generate "0" "$batch_size" "0" "0" "insertion"
done

declare -a inclusion_compressed_accounts_arr=("1" "2" "3" "4" "8")

for compressed_accounts in "${inclusion_compressed_accounts_arr[@]}"
do
   generate "0" "0" "$compressed_accounts" "0" "inclusion"
done

declare -a non_inclusion_compressed_accounts_arr=("1" "2")

for compressed_accounts in "${non_inclusion_compressed_accounts_arr[@]}"
do
   generate "0" "0" "0" "$compressed_accounts" "non-inclusion"
done

declare -a combined_inclusion_compressed_accounts_arr=("1" "2" "3" "4")
declare -a combined_non_inclusion_compressed_accounts_arr=("1" "2")

for i_compressed_accounts in "${combined_inclusion_compressed_accounts_arr[@]}"
do
 for ni_compressed_accounts in "${combined_non_inclusion_compressed_accounts_arr[@]}"
 do
   generate "0" "0" "$i_compressed_accounts" "$ni_compressed_accounts" "combined"
 done
done
