package denimtesting

import (
	"os"
	"testing"

	"github.com/google/go-cmp/cmp"
)

// Represents one test case comprised of an input and desired output.
type GoldenTestCase struct {
	// Absolute path to the file containing desired test output.
	goldenFilePath string
	// Absolute path to the file containing input for the test.
	inputFilePath string
	// Name of this [GoldenTestCase].
	name string
}

// Returns true if this [GoldenTestCase] has all the state it needs to run.
func (goldenTestCase *GoldenTestCase) isComplete() bool {
	return len(goldenTestCase.goldenFilePath) > 0 && len(goldenTestCase.inputFilePath) > 0 && len(goldenTestCase.name) > 0
}

// Runs this [GoldenTestCase] using the specified [goldenTestFunc] to generate
// an output string.
func (goldenTestCase *GoldenTestCase) run(goldenTestFunc GoldenTestFunc, t *testing.T) {
	goldenFileData, err := os.ReadFile(goldenTestCase.goldenFilePath)
	if err != nil {
		t.Errorf("failed to read golden file \"%s\": %v", goldenTestCase.goldenFilePath, err)

		return
	}

	inputFileData, err := os.ReadFile(goldenTestCase.inputFilePath)
	if err != nil {
		t.Errorf("failed to read input file \"%s\": %v", goldenTestCase.inputFilePath, err)

		return
	}

	desiredTestOutput := string(goldenFileData)
	inputFileContents := string(inputFileData)

	actualTestOutput, err := goldenTestFunc(inputFileContents)
	if err != nil {
		t.Errorf("failed to invoke the test function: %v", err)

		return
	}

	if diff := cmp.Diff(desiredTestOutput, actualTestOutput); diff != "" {
		t.Errorf("golden mismatch (-want +got):\n%s", diff)
	}
}
