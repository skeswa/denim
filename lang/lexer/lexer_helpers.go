package lexer

import (
	"strconv"
	"strings"

	"github.com/skeswa/denim/lang/identifier"
	"github.com/skeswa/denim/lang/optimizations"
	"github.com/skeswa/denim/lang/text"
)

// Returns the position currently being lexed.
func (lexer *Lexer) Loc() text.Loc {
	return text.Loc{Start: int32(lexer.start)}
}

// Returns the `Range` currently being lexed.
func (lexer *Lexer) Range() text.Range {
	return text.Range{Loc: text.Loc{Start: int32(lexer.start)}, Len: int32(lexer.end - lexer.start)}
}

// Returns the stretch of text currently being lexed.
func (lexer *Lexer) Raw() string {
	return lexer.source.Contents[lexer.start:lexer.end]
}

// Lexes an entire number or a single isolated dot that starts with the current
// position.
func (lexer *Lexer) parseNumericLiteralOrDot() {
	// Number or dot
	first := lexer.codePoint
	lexer.step()

	// Dot without a digit after it
	if first == '.' && (lexer.codePoint < '0' || lexer.codePoint > '9') {
		// ".."
		if lexer.codePoint == '.' {
			// "..."
			if lexer.current < len(lexer.source.Contents) &&
				lexer.source.Contents[lexer.current] == '.' {
				lexer.step()
				lexer.step()
				lexer.Token = TDotDotDot
				return
			}

			lexer.step()
			lexer.Token = TDotDot
			return
		}

		// "."
		lexer.Token = TDot
		return
	}

	underscoreCount := 0
	lastUnderscoreEnd := 0
	hasDot := first == '.'
	hasExponent := false
	base := 0.0

	// Assume this is an `int`, but potentially change to a `double` later
	lexer.Token = TIntLiteral

	// Check for binary, octal, or hexadecimal literal
	if first == '0' {
		switch lexer.codePoint {
		case 'b', 'B':
			base = 2

		case 'o', 'O':
			base = 8

		case 'x', 'X':
			base = 16

		default:
			// Leading zero must be followed by "x", "o", or "b"
			lexer.SyntaxError()
		}
	}

	if base != 0 {
		// Integer literal
		isFirst := true
		lexer.Number = 0
		lexer.step()

	integerLiteral:
		for {
			switch lexer.codePoint {
			case '_':
				// Cannot have multiple underscores in a row
				if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
					lexer.SyntaxError()
				}

				// The first digit must exist
				if isFirst {
					lexer.SyntaxError()
				}

				lastUnderscoreEnd = lexer.end
				underscoreCount++

			case '0', '1':
				lexer.Number = lexer.Number*base + float64(lexer.codePoint-'0')

			case '2', '3', '4', '5', '6', '7':
				if base == 2 {
					lexer.SyntaxError()
				}
				lexer.Number = lexer.Number*base + float64(lexer.codePoint-'0')

			case '8', '9':
				if base < 10 {
					lexer.SyntaxError()
				}
				lexer.Number = lexer.Number*base + float64(lexer.codePoint-'0')

			case 'A', 'B', 'C', 'D', 'E', 'F':
				if base != 16 {
					lexer.SyntaxError()
				}
				lexer.Number = lexer.Number*base + float64(lexer.codePoint+10-'A')

			case 'a', 'b', 'c', 'd', 'e', 'f':
				if base != 16 {
					lexer.SyntaxError()
				}
				lexer.Number = lexer.Number*base + float64(lexer.codePoint+10-'a')

			default:
				// The first digit must exist
				if isFirst {
					lexer.SyntaxError()
				}

				break integerLiteral
			}

			lexer.step()
			isFirst = false
		}
	} else {
		// Literal could be an int or double

		// Initial digits
		for {
			if lexer.codePoint < '0' || lexer.codePoint > '9' {
				if lexer.codePoint != '_' {
					break
				}

				// Cannot have multiple underscores in a row
				if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
					lexer.SyntaxError()
				}

				lastUnderscoreEnd = lexer.end
				underscoreCount++
			}
			lexer.step()
		}

		// Fractional digits
		if first != '.' && lexer.codePoint == '.' {
			// An underscore must not come last
			if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
				lexer.end--
				lexer.SyntaxError()
			}

			hasDot = true
			lexer.step()
			if lexer.codePoint == '_' {
				lexer.SyntaxError()
			}
			for lexer.codePoint < '0' || lexer.codePoint > '9' {
				if lexer.codePoint != '_' {
					break
				}

				// Cannot have multiple underscores in a row
				if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
					lexer.SyntaxError()
				}

				lastUnderscoreEnd = lexer.end
				underscoreCount++
			}
			lexer.step()
		}

		// Exponent
		if lexer.codePoint == 'e' || lexer.codePoint == 'E' {
			// An underscore must not come last
			if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
				lexer.end--
				lexer.SyntaxError()
			}

			hasExponent = true
			lexer.step()
			if lexer.codePoint == '+' || lexer.codePoint == '-' {
				lexer.step()
			}
			if lexer.codePoint < '0' || lexer.codePoint > '9' {
				lexer.SyntaxError()
			}
			for {
				if lexer.codePoint < '0' || lexer.codePoint > '9' {
					if lexer.codePoint != '_' {
						break
					}

					// Cannot have multiple underscores in a row
					if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
						lexer.SyntaxError()
					}

					lastUnderscoreEnd = lexer.end
					underscoreCount++
				}
				lexer.step()
			}
		}

		// Take a slice of the text to parse
		text := lexer.rawIdentifier()

		// Filter out underscores
		if underscoreCount > 0 {
			bytes := make([]byte, 0, len(text.String)-underscoreCount)
			for i := 0; i < len(text.String); i++ {
				c := text.String[i]
				if c != '_' {
					bytes = append(bytes, c)
				}
			}
			text = optimizations.MaybeSubstring{String: string(bytes)}
		}

		// TODO(skeswa): we can be smarter about knowing "1.215e2" is a `double`,
		// whereas "1.215e3" is an `int` - for now we always assume that the
		// presence of dots or exponents implies a `double`.

		if !hasDot && !hasExponent && lexer.end-lexer.start < 19 {
			// Parse a 32-bit integer (very fast path)
			var number uint64 = 0
			for _, c := range text.String {
				number = number*10 + uint64(c-'0')
			}
			lexer.Number = float64(number)
			lexer.Token = TIntLiteral
		} else {
			// Parse a double-precision floating-point number
			value, _ := strconv.ParseFloat(text.String, 64)
			lexer.Number = value
			lexer.Token = TDoubleLiteral
		}
	}

	// An underscore must not come last
	if lastUnderscoreEnd > 0 && lexer.end == lastUnderscoreEnd+1 {
		lexer.end--
		lexer.SyntaxError()
	}

	// // TODO(skeswa): `bigint` support could be cool in the future.

	// // Handle bigint literals after the underscore-at-end check above
	// if lexer.codePoint == 'n' && !hasDotOrExponent {
	// 	lexer.Token = TBigIntegerLiteral
	// 	lexer.step()
	// }

	// Identifiers can't occur immediately after numbers
	if identifier.IsStart(lexer.codePoint) {
		lexer.SyntaxError()
	}
}

// Returns the stretch of text currently being lexed as an identifier.
func (lexer *Lexer) rawIdentifier() optimizations.MaybeSubstring {
	return optimizations.MaybeSubstring{String: lexer.Raw(), Start: optimizations.MakeIndex32(uint32(lexer.start))}
}

// Lexes an entire stretch of commentary that starts with the current position.
func (lexer *Lexer) scanCommentText() {
	text := lexer.source.Contents[lexer.start:lexer.end]
	hasLegalAnnotation := len(text) > 2 && text[2] == '!'
	isMultiLineComment := text[1] == '*'
	omitFromGeneralCommentPreservation := false

	// Save the original comment text so we can subtract comments from the
	// character frequency analysis used by symbol minification
	lexer.AllComments = append(lexer.AllComments, lexer.Range())

	// Omit the trailing "*/" from the checks below
	endOfCommentText := len(text)
	if isMultiLineComment {
		endOfCommentText -= 2
	}

	for i, n := 0, len(text); i < n; i++ {
		switch text[i] {
		case '#', '@':
			rest := text[i+1 : endOfCommentText]
			if i == 2 && strings.HasPrefix(rest, " sourceMappingURL=") {
				if arg, ok := scanForPragmaArg(pragmaNoSpaceFirst, lexer.start+i+1, " sourceMappingURL=", rest); ok {
					omitFromGeneralCommentPreservation = true
					lexer.SourceMappingURL = arg
				}
			}
		}
	}

	if hasLegalAnnotation {
		lexer.LegalCommentsBeforeToken = append(lexer.LegalCommentsBeforeToken, lexer.Range())
	}

	if !omitFromGeneralCommentPreservation {
		lexer.CommentsBeforeToken = append(lexer.CommentsBeforeToken, lexer.Range())
	}
}
