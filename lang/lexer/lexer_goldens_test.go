package lexer_test

import (
	"strings"
	"testing"

	denimtesting "github.com/skeswa/denim/internal/testing"
	"github.com/skeswa/denim/lang/lexer"
)

func TestLexerGoldens(t *testing.T) {
	if testSuite, err := denimtesting.NewGoldenTestSuite(denimtesting.NewGoldenTestSuiteArgs{
		GoldenFileExtension:              "dlex",
		GoldenTestFunc:                   lexerGoldenTestFunc,
		InputFileExtension:               "denim",
		RepoRelativeGoldensDirectoryPath: "lang/lexer/test_data/goldens/ok",
		T:                                t,
	}); err != nil {
		t.Errorf("failed to create test suite: %v", err)
	} else {
		testSuite.Run()
	}
}

// Executes a golden test for the [lexer.Lexer].
func lexerGoldenTestFunc(input string) (string, error) {
	lexer := lexer.NewLexer(input)

	var stringBuilder strings.Builder
	for !lexer.IsTerminated() {
		token := lexer.NextToken()

		stringBuilder.WriteString(token.String())
		stringBuilder.WriteRune('\n')
	}

	return stringBuilder.String(), nil
}
