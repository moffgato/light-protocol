/**
* This file is auto-generated by the Light cli.
* DO NOT EDIT MANUALLY.
* THE FILE WILL BE OVERWRITTEN EVERY TIME THE LIGHT CLI BUILD IS RUN.
*/
pragma circom 2.1.4;

include "poseidon.circom";
include "merkleProof.circom";
include "keypair.circom";
include "gates.circom";
include "comparators.circom";


template streaming_payments( nAppUtxos, levels, nIns, nOuts, feeAsset, indexFeeAsset, indexPublicAsset, nAssets, nInAssets, nOutAssets) {


    assert( nIns * nAssets < 49);
    assert( nInAssets <= nAssets);
    assert( nOutAssets <= nAssets);

    signal input isAppInUtxo[nAppUtxos][nIns];
    signal input txIntegrityHash;
    signal input  inAmount[nIns][nInAssets];
    signal input  inPublicKey[nIns];
    signal input  inBlinding[nIns];
    signal input  inAppDataHash[nIns];
    signal  input inPoolType[nIns];
    signal  input inVerifierPubkey[nIns];
    signal  input inIndices[nIns][nInAssets][nAssets];

    // data for transaction outputsAccount
    signal  input outputCommitment[nOuts];
    signal  input outAmount[nOuts][nOutAssets];
    signal  input outPubkey[nOuts];
    signal  input outBlinding[nOuts];
    signal  input outAppDataHash[nOuts];
    signal  input outIndices[nOuts][nOutAssets][nAssets];
    signal  input outPoolType[nOuts];
    signal  input outVerifierPubkey[nOuts];

    signal  input assetPubkeys[nAssets];
    signal input transactionVersion;

    component inGetAsset[nIns][nInAssets][nAssets];

    component inCommitmentHasher[nIns];
    component inAmountsHasher[nIns];
    component inAssetsHasher[nIns];

    component sumIn[nIns][nInAssets][nAssets];
    component inAmountCheck[nIns][nInAssets];


    // enforce pooltypes of 0
    // add public input to distinguish between pool types
    inPoolType[0] === 0;
    inPoolType[0] === outPoolType[0];

    var sumIns[nAssets];
    for (var i = 0; i < nAssets; i++) {
    sumIns[i] = 0;
    }

    var assetsIns[nIns][nInAssets];
    for (var i = 0; i < nIns; i++) {
        for (var j = 0; j < nInAssets; j++) {
        assetsIns[i][j] = 0;
        }
    }

    // verify correctness of transaction s
    for (var tx = 0; tx < nIns; tx++) {

        // determine the asset type
        // and checks that the asset is included in assetPubkeys[nInAssets]
        // skips first asset since that is the feeAsset
        // iterates over remaining assets and adds the assetPubkey if index is 1
        // all other indices are zero
        inAssetsHasher[tx] = Poseidon(nInAssets);
        for (var a = 0; a < nInAssets; a++) {

            for (var i = 0; i < nAssets; i++) {
                inGetAsset[tx][a][i] = AND();
                inGetAsset[tx][a][i].a <== assetPubkeys[i];
                inGetAsset[tx][a][i].b <== inIndices[tx][a][i];
                assetsIns[tx][a] += inGetAsset[tx][a][i].out;
            }
            inAssetsHasher[tx].inputs[a] <== assetsIns[tx][a];
        }

        inAmountsHasher[tx] = Poseidon(nInAssets);
        var sumInAmount = 0;
        for (var a = 0; a < nInAssets; a++) {
            inAmountCheck[tx][a] = Num2Bits(64);
            inAmountCheck[tx][a].in <== inAmount[tx][a];
            inAmountsHasher[tx].inputs[a] <== inAmount[tx][a];
            sumInAmount += inAmount[tx][a];
        }

        inCommitmentHasher[tx] = Poseidon(8);
        inCommitmentHasher[tx].inputs[0] <== transactionVersion; // transaction version
        inCommitmentHasher[tx].inputs[1] <== inAmountsHasher[tx].out;
        inCommitmentHasher[tx].inputs[2] <== inPublicKey[tx];
        inCommitmentHasher[tx].inputs[3] <== inBlinding[tx];
        inCommitmentHasher[tx].inputs[4] <== inAssetsHasher[tx].out;
        inCommitmentHasher[tx].inputs[5] <== inAppDataHash[tx];
        inCommitmentHasher[tx].inputs[6] <== inPoolType[tx];
        inCommitmentHasher[tx].inputs[7] <== inVerifierPubkey[tx];




        // for (var i = 0; i < nInAssets; i++) {
        //     for (var j = 0; j < nAssets; j++) {
        //         sumIn[tx][i][j] = AND();
        //         sumIn[tx][i][j].a <== inAmount[tx][i];
        //         sumIn[tx][i][j].b <== inIndices[tx][i][j];
        //         sumIns[j] += sumIn[tx][i][j].out;
        //     }
        // }
    }

    component outGetAsset[nOuts][nOutAssets][nAssets];
    component outCommitmentHasher[nOuts];
    component outAmountCheck[nOuts][nOutAssets];
    component sumOut[nOuts][nOutAssets][nAssets];
    component outAmountsHasher[nOuts];
    component outAssetsHasher[nOuts];

    var sumOuts[nAssets];
    for (var i = 0; i < nAssets; i++) {
    sumOuts[i] = 0;
    }

    var assetsOuts[nOuts][nOutAssets];
    for (var i = 0; i < nOuts; i++) {
        for (var j = 0; j < nOutAssets; j++) {
        assetsOuts[i][j] = 0;
        }
    }

    // verify correctness of transaction outputs
    for (var tx = 0; tx < nOuts; tx++) {

        // for every asset for every tx only one index is 1 others are 0
        // select the asset corresponding to the index
        // and add it to the assetHasher
        outAssetsHasher[tx] = Poseidon(nOutAssets);

        for (var a = 0; a < nOutAssets; a++) {
            var asset = 0;
            for (var i = 0; i < nAssets; i++) {
                outGetAsset[tx][a][i] = AND();
                outGetAsset[tx][a][i].a <== assetPubkeys[i];
                outGetAsset[tx][a][i].b <== outIndices[tx][a][i];
                asset += outGetAsset[tx][a][i].out;
            }
            assetsOuts[tx][a] = asset;
            outAssetsHasher[tx].inputs[a] <== asset;
        }

        for (var i = 0; i < nOutAssets; i++) {
            // Check that amount fits into 64 bits to prevent overflow
            outAmountCheck[tx][i] = Num2Bits(64);
            outAmountCheck[tx][i].in <== outAmount[tx][i];
        }

        outAmountsHasher[tx] = Poseidon(nOutAssets);
        for (var i = 0; i < nOutAssets; i++) {
            outAmountsHasher[tx].inputs[i] <== outAmount[tx][i];
        }

        outCommitmentHasher[tx] = Poseidon(8);
        outCommitmentHasher[tx].inputs[0] <== transactionVersion; // transaction version
        outCommitmentHasher[tx].inputs[1] <== outAmountsHasher[tx].out;
        outCommitmentHasher[tx].inputs[2] <== outPubkey[tx];
        outCommitmentHasher[tx].inputs[3] <== outBlinding[tx];
        outCommitmentHasher[tx].inputs[4] <== outAssetsHasher[tx].out;
        outCommitmentHasher[tx].inputs[5] <== outAppDataHash[tx];
        outCommitmentHasher[tx].inputs[6] <== outPoolType[tx];
        outCommitmentHasher[tx].inputs[7] <== outVerifierPubkey[tx];
        outCommitmentHasher[tx].out === outputCommitment[tx];

        // ensure that all pool types are the same
        outPoolType[0] === outPoolType[tx];
    }

    // public inputs
    signal input publicAppVerifier;
    signal  input transactionHash;

    // generating input hash
    // hash commitment 
    component inputHasher = Poseidon(nIns);
    for (var i = 0; i < nIns; i++) {
        inputHasher.inputs[i] <== inCommitmentHasher[i].out;
    }

    component outputHasher = Poseidon(nOuts);
    for (var i = 0; i < nOuts; i++) {
        outputHasher.inputs[i] <== outCommitmentHasher[i].out;
    }

    component transactionHasher = Poseidon(3);

    transactionHasher.inputs[0] <== inputHasher.out;
    transactionHasher.inputs[1] <== outputHasher.out;
    transactionHasher.inputs[2] <== txIntegrityHash;


    transactionHash === transactionHasher.out;

signal input endSlot;
signal input rate;
component instructionHasher[nAppUtxos];

            component checkInstructionHash[nAppUtxos][nIns];
for (var appUtxoIndex = 0; appUtxoIndex < nAppUtxos; appUtxoIndex++) {
            	instructionHasher[appUtxoIndex] = Poseidon(2);
instructionHasher[appUtxoIndex].inputs[0] <== endSlot;
instructionHasher[appUtxoIndex].inputs[1] <== rate;
for (var inUtxoIndex = 0; inUtxoIndex < nIns; inUtxoIndex++) {
        checkInstructionHash[appUtxoIndex][inUtxoIndex] = ForceEqualIfEnabled();
        checkInstructionHash[appUtxoIndex][inUtxoIndex].in[0] <== inAppDataHash[inUtxoIndex];
        checkInstructionHash[appUtxoIndex][inUtxoIndex].in[1] <== instructionHasher[appUtxoIndex].out;
        checkInstructionHash[appUtxoIndex][inUtxoIndex].enabled <== isAppInUtxo[appUtxoIndex][inUtxoIndex];
   }

    }

/**
* -------------------------- Application starts here --------------------------
*/
signal input currentSlotPrivate;
signal input currentSlot;
signal input diff;
signal input remainingAmount;
signal input isOutUtxo[nOuts];

component rangeCheckDiff = Num2Bits(64);
rangeCheckDiff.in <== diff;
component rangeCheckSlotPrivate = Num2Bits(64);
rangeCheckSlotPrivate.in <== currentSlotPrivate;

currentSlotPrivate + diff === currentSlot;

(endSlot - currentSlotPrivate) * rate === remainingAmount;

component greaterThanZero = GreaterEqThan(64);
greaterThanZero.in[0] <== endSlot - currentSlotPrivate;
greaterThanZero.in[1] <== 0;
greaterThanZero.out === 1;

var standardProgramUtxoPubkey = 0; //Poseidon(0);
component checkRemainingAmount[nOuts][nOuts];
component checkOutInstructionHash[nOuts][nOuts];
component checkPublicAppVerifier[nOuts][nOuts];

for(var i=0; i < nOuts; i++) {
for(var j = 0 ; j < nOuts; j++) {
checkRemainingAmount[i][j] = ForceEqualIfEnabled();
checkRemainingAmount[i][j].in[0] <== remainingAmount;
checkRemainingAmount[i][j].in[1] <== outAmount[i][0];
checkRemainingAmount[i][j].enabled <== isOutUtxo[j];

checkOutInstructionHash[i][j] = ForceEqualIfEnabled();
checkOutInstructionHash[i][j].in[0] <== instructionHasher[0].out;
checkOutInstructionHash[i][j].in[1] <== outAppDataHash[i];
checkOutInstructionHash[i][j].enabled <== isOutUtxo[j];

checkPublicAppVerifier[i][j] = ForceEqualIfEnabled();
checkPublicAppVerifier[i][j].in[0] <== publicAppVerifier;
checkPublicAppVerifier[i][j].in[1] <== outVerifierPubkey[i];
checkPublicAppVerifier[i][j].enabled <== isOutUtxo[j];
}
}

component checkIndices = CheckIndices(nOuts);
checkIndices.indices <== isOutUtxo;
checkIndices.threshold <== 1;
checkIndices.enabled <== remainingAmount;
}

template CheckIndices(n) {
signal input indices[n];
signal input threshold;
signal input enabled;
var varSumIndices = 0;
for (var j = 0; j < n; j++) {
varSumIndices += indices[j];
indices[j] * (1 - indices[j]) === 0;
}
component checkIfEnabled = ForceEqualIfEnabled();
checkIfEnabled.in[0] <== threshold;
checkIfEnabled.in[1] <== varSumIndices;
checkIfEnabled.enabled <== enabled;
}

/*
* Environment Constants:
*   levels = 18
*   nIns = 4
*   nOuts = 4
*   feeAsset = TruncatedKeccak256(0)
*   indexFeeAsset = 0
*   indexPublicAsset = 1
*   nAssets = 3
*   nInAssets = 2
*   nOutAssets = 2
* Environment variables:
*   txIntegrityHash;
*   transactionVersion;
*   publicAppVerifier;
*   transactionHash;
*   instructionHasher.out;
*  InUtxos:
*   inAmount[nIns][nInAssets];
*   inPublicKey[nIns];
*   inBlinding[nIns];
*   inAppDataHash[nIns];
*   inPoolType[nIns];
*   inVerifierPubkey[nIns];
*   inIndices[nIns][nInAssets][nAssets];
* OutUtxos:
*   outputCommitment[nOuts];
*   outAmount[nOuts][nOutAssets];
*   outPubkey[nOuts];
*   outBlinding[nOuts];
*   outAppDataHash[nOuts];
*   outIndices[nOuts][nOutAssets][nAssets];
*   outPoolType[nOuts];
*   outVerifierPubkey[nOuts];
*/