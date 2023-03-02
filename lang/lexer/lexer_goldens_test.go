package lexer

import (
	"testing"

	denimtesting "github.com/skeswa/denim/internal/testing"
)

func TestLexerGoldens(t *testing.T) {
	if testSuite, err := denimtesting.NewGoldenTestSuite(denimtesting.NewGoldenTestSuiteArgs{
		GoldenFileExtension: "dlex",
		GoldenTestFunc: func(input string) (string, error) {
			return "ayy lmao", nil
		},
		InputFileExtension:               "denim",
		RepoRelativeGoldensDirectoryPath: "lang/lexer/test_data/goldens/ok",
		T:                                t,
	}); err != nil {
		t.Errorf("failed to create test suite: %v", err)
	} else {
		testSuite.Run()
	}
}
