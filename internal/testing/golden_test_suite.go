package denimtesting

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"
)

// A related collection of golden test cases.
type GoldenTestSuite struct {
	// All of the test cases in this test suite.
	goldenTestCases []GoldenTestCase
	// Function used to generate golden test output.
	goldenTestFunc GoldenTestFunc
	// Name of this [GoldenTestSuite].
	name string
	// Test object used by the surrounding Go test.
	t *testing.T
}

// Arguments `struct` for [NewGoldenTestSuite].
type NewGoldenTestSuiteArgs struct {
	// File extension used by the golden files for this test suite.
	GoldenFileExtension string
	// Function used to generate golden test output.
	GoldenTestFunc GoldenTestFunc
	// File extension used by the input files for this test suite.
	InputFileExtension string
	// Relative path within the repo to the directory containing the goldens.
	RepoRelativeGoldensDirectoryPath string
	// Test object used by the surrounding Go test.
	T *testing.T
}

// Creates a new [GoldenTestSuite] out of a directory of goldens and inputs.
func NewGoldenTestSuite(args NewGoldenTestSuiteArgs) (*GoldenTestSuite, error) {
	goldensDirectoryPath, err := RepoRelativePathToAbsolute(args.RepoRelativeGoldensDirectoryPath)
	if err != nil {
		return nil, errors.Join(errors.New("failed to find goldens directory"), err)
	}

	goldensDirectoryName := filepath.Base(goldensDirectoryPath)

	goldenFileExtension := formatFileExtension(args.GoldenFileExtension)
	inputFileExtension := formatFileExtension(args.InputFileExtension)

	var (
		goldenTestCase       *GoldenTestCase
		goldenTestCaseByName = map[string]*GoldenTestCase{}
		ok                   bool
	)

	filesAndDirectories, err := os.ReadDir(goldensDirectoryPath)
	if err != nil {
		return nil, errors.Join(errors.New("failed to list files within goldens directory"), err)
	}

	for _, filesOrDirectory := range filesAndDirectories {
		if filesOrDirectory.IsDir() {
			continue
		}

		fileNameAndExtension := filesOrDirectory.Name()

		fileExtension := strings.ToLower(filepath.Ext(fileNameAndExtension))
		if fileExtension != goldenFileExtension && fileExtension != inputFileExtension {
			continue
		}

		fileName := strings.TrimSuffix(fileNameAndExtension, fileExtension)
		if goldenTestCase, ok = goldenTestCaseByName[fileName]; !ok {
			goldenTestCase = &GoldenTestCase{name: fileName}
			goldenTestCaseByName[fileName] = goldenTestCase
		}

		filePath := filepath.Join(goldensDirectoryPath, fileNameAndExtension)

		switch fileExtension {
		case goldenFileExtension:
			goldenTestCase.goldenFilePath = filePath
		case inputFileExtension:
			goldenTestCase.inputFilePath = filePath
		}
	}

	goldenTestCases := []GoldenTestCase{}
	for _, goldenTestCase := range goldenTestCaseByName {
		if !goldenTestCase.isComplete() {
			return nil, fmt.Errorf("test case \"%s\" is not complete", goldenTestCase.name)
		}

		goldenTestCases = append(goldenTestCases, *goldenTestCase)
	}

	sort.Sort(byGoldenTestCaseName(goldenTestCases))

	goldenTestSuite := GoldenTestSuite{
		goldenTestCases: goldenTestCases,
		goldenTestFunc:  args.GoldenTestFunc,
		name:            goldensDirectoryName,
		t:               args.T,
	}

	return &goldenTestSuite, nil
}

// Runs every test in this [GoldenTestSuite].
func (goldenTestSuite *GoldenTestSuite) Run() {
	goldenTestSuite.t.Run(goldenTestSuite.name, func(t *testing.T) {
		for _, goldenTestCase := range goldenTestSuite.goldenTestCases {
			t.Run(goldenTestCase.name, func(t *testing.T) {
				goldenTestCase.run(goldenTestSuite.goldenTestFunc, t)
			})
		}
	})
}

// Returns a formatted version of [fileExtension].
func formatFileExtension(fileExtension string) string {
	fileExtension = strings.ToLower(strings.TrimSpace(fileExtension))

	if strings.IndexRune(fileExtension, '.') == 0 {
		return fileExtension
	}

	fileExtension = "." + fileExtension

	return fileExtension
}
