package lexer

import (
	"unicode/utf8"

	"github.com/skeswa/denim/lang/ast"
	"github.com/skeswa/denim/lang/logger"
)

// Enumerates every variety of Denim pragma.
type pragmaArg uint8

const (
	// Pragma kind that does not start with a space.
	pragmaNoSpaceFirst pragmaArg = iota
	// Pragma kind that starts with a space.
	pragmaSkipSpaceFirst
)

// Extracts the location of the given `pragma“ from the specified `text`.
func scanForPragmaArg(kind pragmaArg, start int, pragma string, text string) (logger.Span, bool) {
	text = text[len(pragma):]
	start += len(pragma)

	if text == "" {
		return logger.Span{}, false
	}

	// One or more whitespace characters
	c, width := utf8.DecodeRuneInString(text)
	if kind == pragmaSkipSpaceFirst {
		if !ast.IsWhitespace(c) {
			return logger.Span{}, false
		}
		for ast.IsWhitespace(c) {
			text = text[width:]
			start += width
			if text == "" {
				return logger.Span{}, false
			}
			c, width = utf8.DecodeRuneInString(text)
		}
	}

	// One or more non-whitespace characters
	i := 0
	for !ast.IsWhitespace(c) {
		i += width
		if i >= len(text) {
			break
		}
		c, width = utf8.DecodeRuneInString(text[i:])
		if ast.IsWhitespace(c) {
			break
		}
	}

	return logger.Span{
		Text: text[:i],
		Range: logger.Range{
			Loc: logger.Loc{Start: int32(start)},
			Len: int32(i),
		},
	}, true
}
