package merkle_tree

import (
	"encoding/json"
	"fmt"
	"light/light-prover/prover"
	"math/big"
	"os"
	"testing"
)

func TestInclusionParameters_TestTree(t *testing.T) {
	file, err := os.OpenFile("../test-data/inclusion.csv", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			t.Errorf("Error closing file: %v", err)
		}
	}(file)

	var testTreeDepth = []int{26}
	var testCompressedAccountCount = []int{1, 2, 3, 4}

	for i := 0; i < len(testTreeDepth); i++ {
		for j := 0; j < len(testCompressedAccountCount); j++ {
			trees := MakeTestIncludedTrees(testTreeDepth[i], testCompressedAccountCount[j])
			for _, tree := range trees {
				var json, err = tree.Tree.MarshalJSON()
				if err != nil {
					t.Errorf("Error marshalling JSON: %v", err)
					return
				}

				_, err = fmt.Fprintf(file, "%d;%s\n", flag(tree.Valid), json)
				if err != nil {
					t.Errorf("Error writing to file: %v", err)
					return
				}
			}
		}
	}
}

func TestNonInclusionParameters_TestTree(t *testing.T) {
	file, err := os.OpenFile("../test-data/non_inclusion.csv", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		t.Errorf("Error opening file: %v", err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			t.Errorf("Error closing file: %v", err)
		}
	}(file)

	var testTreeDepth = []int{26}
	var testCompressedAccountCount = []int{1, 2, 3, 4}

	for i := 0; i < len(testTreeDepth); i++ {
		for j := 0; j < len(testCompressedAccountCount); j++ {
			trees := MakeTestNonInclusionTrees(testTreeDepth[i], testCompressedAccountCount[j])
			for _, tree := range trees {
				var json, err = tree.Tree.MarshalJSON()
				if err != nil {
					t.Errorf("Error marshalling JSON: %v", err)
					return
				}
				_, err = fmt.Fprintf(file, "%d;%s\n", flag(tree.Valid), json)
				if err != nil {
					t.Errorf("Error writing to file: %v", err)
					return
				}
			}
		}
	}
}

func TestCombined(t *testing.T) {
	file, err := os.OpenFile("../test-data/combined.csv", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		t.Errorf("Error opening file: %v", err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			t.Errorf("Error closing file: %v", err)
		}
	}(file)

	var testTreeDepth = []int{26}
	var testCompressedAccountCount = []int{1, 2, 3, 4}

	for i := 0; i < len(testTreeDepth); i++ {
		for j := 0; j < len(testCompressedAccountCount); j++ {
			trees1 := MakeTestIncludedTrees(testTreeDepth[i], testCompressedAccountCount[j])
			trees2 := MakeTestNonInclusionTrees(testTreeDepth[i], testCompressedAccountCount[j])
			for k, tree1 := range trees1 {
				for l, tree2 := range trees2 {
					var combinedParams = prover.CombinedParameters{
						InclusionParameters:    tree1.Tree,
						NonInclusionParameters: tree2.Tree,
					}
					var json, err = combinedParams.MarshalJSON()
					if err != nil {
						t.Errorf("Error marshalling JSON: %v", err)
						return
					}

					valid := tree1.Valid && tree2.Valid
					_, err = fmt.Fprintf(file, "%d;%s\n", flag(valid), json)
					if err != nil {
						t.Errorf("Error writing to file: %v", err)
						return
					}
					fmt.Printf("Test %d: %d, %d\n", i, k, l)
				}
			}
		}
	}
}

func flag(valid bool) int {
	if valid {
		return 1
	}
	return 0
}

type InclusionTreeValidPair struct {
	Tree  prover.InclusionParameters
	Valid bool
}

// Function
//
// `MakeTestIncludedTrees`
//
// ```go
// func MakeTestIncludedTrees(depth int, numberOfCompressedAccounts int) []InclusionTreeValidPair
// ```
//
// # Description
//
// The `MakeTestIncludedTrees` function creates an array of InclusionTreeValidPair instances for testing.
// The variation between valid and invalid trees helps simulate real-world scenarios and assists in better
// testing for robustness and error-handling.
//
// Parameters:
//
//   - `depth (int)`: Defines the depth of each included tree.
//   - `numberOfCompressedAccounts (int)`: Number of unspent transaction outputs (CompressedAccounts) to include in each tree.
//
// Returns:
// - `[]InclusionTreeValidPair`: An array of `InclusionTreeValidPair` instances, each containing
// an `InclusionParameters` instance and a boolean value indicating whether the tree is valid.
//
// Pairs Explanation:
//
// - `validPair`: A valid tree constructed with input parameters. The Valid field is set to `true`.
// - `invalidRootPair`: A valid tree but the root value is invalidated by setting it to an integer 999. The Valid field is set to `false`.
// - `invalidLeafPair`: A valid tree where a leaf value is invalidated by setting it to an integer 999. The Valid field wis set to `false`.
// - `invalidInPathIndicesPair`: A valid tree but the InPathIndices value is invalidated by adding 1 to the index. The Valid field is set to `false`.
// - `invalidInPathIndicesPair`: A valid tree but the InPathIndices value is invalidated by subtracting 1 from the index. The Valid field is set to `false`.
// - `invalidInPathElementsPair`: A valid tree where the InPathElements is invalidated by setting a value to an integer 999. The Valid field is set to `false`.
//
// Example usage:
//
// ```go
// trees := MakeTestIncludedTrees(4, 2)
//
//	for _, tree := range trees {
//	    // perform operations on tree
//	}
//
// ```
func MakeTestIncludedTrees(depth int, numberOfCompressedAccounts int) []InclusionTreeValidPair {
	var trees []InclusionTreeValidPair

	validTree := BuildTestTree(depth, numberOfCompressedAccounts, false)
	validPair := InclusionTreeValidPair{Tree: validTree, Valid: true}

	invalidRootTree := BuildTestTree(depth, numberOfCompressedAccounts, true)
	invalidRootTree.Inputs[0].Root = *big.NewInt(999)
	invalidRootPair := InclusionTreeValidPair{Tree: invalidRootTree, Valid: false}

	invalidLeafTree := BuildTestTree(depth, numberOfCompressedAccounts, true)
	invalidLeafTree.Inputs[0].Leaf = *big.NewInt(999)
	invalidLeafPair := InclusionTreeValidPair{Tree: invalidLeafTree, Valid: false}

	invalidInPathIndicesTreeAddOne := BuildTestTree(depth, numberOfCompressedAccounts, true)
	invalidInPathIndicesTreeAddOne.Inputs[0].PathIndex = invalidInPathIndicesTreeAddOne.Inputs[0].PathIndex + 1
	invalidInPathIndicesPairAddOne := InclusionTreeValidPair{Tree: invalidInPathIndicesTreeAddOne, Valid: false}

	invalidInPathIndicesTreeSubOne := BuildTestTree(depth, numberOfCompressedAccounts, true)
	invalidInPathIndicesTreeSubOne.Inputs[0].PathIndex = invalidInPathIndicesTreeSubOne.Inputs[0].PathIndex - 1
	invalidInPathIndicesPairSubOne := InclusionTreeValidPair{Tree: invalidInPathIndicesTreeSubOne, Valid: false}

	invalidInPathElementsTree := BuildTestTree(depth, numberOfCompressedAccounts, true)
	invalidInPathElementsTree.Inputs[0].PathElements[0] = *big.NewInt(999)
	invalidInPathElementsPair := InclusionTreeValidPair{Tree: invalidInPathElementsTree, Valid: false}

	trees = append(trees, validPair)
	trees = append(trees, invalidRootPair)
	trees = append(trees, invalidLeafPair)
	trees = append(trees, invalidInPathIndicesPairAddOne)
	trees = append(trees, invalidInPathIndicesPairSubOne)
	trees = append(trees, invalidInPathElementsPair)
	return trees
}

type NonInclusionTreeValidPair struct {
	Tree  prover.NonInclusionParameters
	Valid bool
}

// Function
//
// `MakeTestNonInclusionTrees`
//
// ```go
// func MakeTestNonInclusionTrees(depth int, numberOfCompressedAccounts int) []NonInclusionTreeValidPair
// ```
//
// # Description
//
// The `MakeTestNonInclusionTrees` function creates an array of `NonInclusionTreeValidPair` instances for testing. These instances include various valid and invalid cases to simulate diverse scenarios and strengthen code robustness and error handling. This function helps in creating a testing environment that closely mimics a variety of real-world scenarios.
//
// # Parameters
//
// - `depth (int)`: Defines the depth of each included tree.
// - `numberOfCompressedAccounts (int)`: Number of unspent transaction outputs (CompressedAccounts) to include in each tree.
//
// # Returns
//
// - `[]NonInclusionTreeValidPair`: An array of `NonInclusionTreeValidPair` instances, each containing an `InclusionParameters` instance and a boolean value indicating whether the tree is valid.
//
// # Pairs Explanation
//
// - `validPair`: A tree constructed with input parameters. The `Valid` field is set to `true`.
//
// - `invalidRootPair`: A valid tree but the root value is invalidated by setting it to an integer 999. The `Valid` field is set to `false`.
//
// - `invalidLowValuePair`: An invalid tree with a low value. The `Valid` field is set to `false`.
//
// - `invalidHighValuePair`: An invalid tree with a high value. The `Valid` field is set to `false`.
//
// - `invalidInPathIndicesPair`: A valid tree but the `InPathIndices` value is invalidated by setting it to an integer 999. The `Valid` field is set to `false`.
//
// - `invalidInPathElementsPair`: A valid tree where the `InPathElements` are invalidated by an integer 999. The `Valid` field is set to `false`.
//
// # Example Usage
//
// ```go
// trees := MakeTestNonInclusionTrees(4, 2)
//
//	for _, tree := range trees {
//	    // perform operations on tree
//	}
//
// ```
func MakeTestNonInclusionTrees(depth int, numberOfCompressedAccounts int) []NonInclusionTreeValidPair {
	var trees []NonInclusionTreeValidPair

	validTree := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	validPair := NonInclusionTreeValidPair{Tree: validTree, Valid: true}

	invalidRootTree := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	invalidRootTree.Inputs[0].Root = *big.NewInt(999)
	invalidRootPair := NonInclusionTreeValidPair{Tree: invalidRootTree, Valid: false}

	invalidNextIndex := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	invalidNextIndex.Inputs[0].NextIndex = 999
	invalidNextIndexPair := NonInclusionTreeValidPair{Tree: invalidRootTree, Valid: false}

	invalidLowValueTree := BuildTestNonInclusionTree(depth, numberOfCompressedAccounts, true, false, true)
	invalidLowValuePair := NonInclusionTreeValidPair{Tree: invalidLowValueTree, Valid: false}

	invalidHighValueTree := BuildTestNonInclusionTree(depth, numberOfCompressedAccounts, true, false, false)
	invalidHighValuePair := NonInclusionTreeValidPair{Tree: invalidHighValueTree, Valid: false}

	invalidInPathIndicesTreeAddOne := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	invalidInPathIndicesTreeAddOne.Inputs[0].PathIndex += 1
	invalidInPathIndicesPairAddOne := NonInclusionTreeValidPair{Tree: invalidInPathIndicesTreeAddOne, Valid: false}

	invalidInPathIndicesTreeSubOne := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	invalidInPathIndicesTreeSubOne.Inputs[0].PathIndex -= 1
	invalidInPathIndicesPairSubOne := NonInclusionTreeValidPair{Tree: invalidInPathIndicesTreeSubOne, Valid: false}

	invalidInPathElementsTree := BuildValidTestNonInclusionTree(depth, numberOfCompressedAccounts, true)
	invalidInPathElementsTree.Inputs[0].PathElements[0] = *big.NewInt(999)
	invalidInPathElementsPair := NonInclusionTreeValidPair{Tree: invalidInPathElementsTree, Valid: false}

	trees = append(trees, validPair)
	trees = append(trees, invalidRootPair)
	trees = append(trees, invalidNextIndexPair)
	trees = append(trees, invalidLowValuePair)
	trees = append(trees, invalidHighValuePair)
	trees = append(trees, invalidInPathIndicesPairAddOne)
	trees = append(trees, invalidInPathIndicesPairSubOne)
	trees = append(trees, invalidInPathElementsPair)
	return trees
}

func TestBatchUpdateParameters_TestTree(t *testing.T) {
	file, err := os.OpenFile("../test-data/batch_update.csv", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		t.Errorf("Error opening file: %v", err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			t.Errorf("Error closing file: %v", err)
		}
	}(file)

	var testTreeDepth = []int{26}
	var testBatchSizes = []int{1, 2, 4, 8}

	for _, depth := range testTreeDepth {
		for _, batchSize := range testBatchSizes {
			batchUpdateParams := BuildTestBatchUpdateTree(depth, batchSize)

			json, err := json.Marshal(batchUpdateParams)
			if err != nil {
				t.Errorf("Error marshalling JSON: %v", err)
				return
			}

			_, err = fmt.Fprintf(file, "1;%s\n", json)
			if err != nil {
				t.Errorf("Error writing to file: %v", err)
				return
			}

			// Generate an invalid case (e.g., wrong pre-root)
			invalidParams := *batchUpdateParams
			invalidParams.PreRoot = *big.NewInt(999) // Set an invalid pre-root

			invalidJSON, err := json.Marshal(&invalidParams)
			if err != nil {
				t.Errorf("Error marshalling invalid JSON: %v", err)
				return
			}

			_, err = fmt.Fprintf(file, "0;%s\n", invalidJSON)
			if err != nil {
				t.Errorf("Error writing invalid case to file: %v", err)
				return
			}
		}
	}
}
