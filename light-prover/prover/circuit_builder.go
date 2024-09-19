package prover

import (
	"encoding/json"
	"fmt"
)

type CircuitType string

const (
	Combined     CircuitType = "combined"
	Inclusion    CircuitType = "inclusion"
	NonInclusion CircuitType = "non-inclusion"
	Insertion    CircuitType = "insertion"
)

func SetupCircuit(
	circuit CircuitType,
	inclusionTreeDepth uint32,
	inclusionNumberOfCompressedAccounts uint32,
	nonInclusionTreeDepth uint32,
	nonInclusionNumberOfCompressedAccounts uint32,
	insertionTreeDepth uint32,
	insertionBatchSize uint32,
) (*ProvingSystem, error) {
	switch circuit {
	case Inclusion:
		return SetupInclusion(inclusionTreeDepth, inclusionNumberOfCompressedAccounts)
	case NonInclusion:
		return SetupNonInclusion(nonInclusionTreeDepth, nonInclusionNumberOfCompressedAccounts)
	case Combined:
		return SetupCombined(inclusionTreeDepth, inclusionNumberOfCompressedAccounts, nonInclusionTreeDepth, nonInclusionNumberOfCompressedAccounts)
	case Insertion:
		return SetupInsertion(insertionTreeDepth, insertionBatchSize)
	default:
		return nil, fmt.Errorf("invalid circuit: %s", circuit)
	}
}

func ParseCircuitType(data []byte) (CircuitType, error) {
	var inputs map[string]*json.RawMessage
	err := json.Unmarshal(data, &inputs)
	if err != nil {
		return "", err
	}

	_, hasInputCompressedAccounts := inputs["input-compressed-accounts"]
	_, hasNewAddresses := inputs["new-addresses"]
	_, hasInsertionInputs := inputs["insertion-inputs"]

	if hasInputCompressedAccounts && hasNewAddresses {
		return Combined, nil
	} else if hasInputCompressedAccounts {
		return Inclusion, nil
	} else if hasNewAddresses {
		return NonInclusion, nil
	} else if hasInsertionInputs {
		return Insertion, nil
	}
	return "", fmt.Errorf("unknown schema")
}

func IsCircuitEnabled(s []CircuitType, e CircuitType) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}
