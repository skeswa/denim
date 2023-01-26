package lexer

import (
	"unicode/utf8"

	"github.com/skeswa/denim/lang/text"
)

// Enumerates every variety of Denim pragma.
type pragmaArg uint8

const (
	// Pragma kind that does not start with a space.
	pragmaNoSpaceFirst pragmaArg = iota
	// Pragma kind that starts with a space.
	pragmaSkipSpaceFirst
)

// Extracts the location of the given `pragma“ from the specified `txt`.
func scanForPragmaArg(kind pragmaArg, start int, pragma string, txt string) (text.Span, bool) {
	txt = txt[len(pragma):]
	start += len(pragma)

	if txt == "" {
		return text.Span{}, false
	}

	// One or more whitespace characters
	c, width := utf8.DecodeRuneInString(txt)
	if kind == pragmaSkipSpaceFirst {
		if !text.IsWhitespace(c) {
			return text.Span{}, false
		}
		for text.IsWhitespace(c) {
			txt = txt[width:]
			start += width
			if txt == "" {
				return text.Span{}, false
			}
			c, width = utf8.DecodeRuneInString(txt)
		}
	}

	// One or more non-whitespace characters
	i := 0
	for !text.IsWhitespace(c) {
		i += width
		if i >= len(txt) {
			break
		}
		c, width = utf8.DecodeRuneInString(txt[i:])
		if text.IsWhitespace(c) {
			break
		}
	}

	return text.Span{
		Text: txt[:i],
		Range: text.Range{
			Loc: text.Loc{Start: int32(start)},
			Len: int32(i),
		},
	}, true
}
