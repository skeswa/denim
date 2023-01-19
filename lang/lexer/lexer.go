package lexer

import (
	"strings"
	"unicode/utf8"

	"github.com/skeswa/denim/lang/logger"
)

// Converts a source file to a stream of tokens.
//
// Unlike many compilers, Denim does not run the lexer to completion before
// the parser is started. Instead, the lexer is called repeatedly by the parser
// as the parser parses the file. This is because many tokens are
// context-sensitive and need high-level information from the parser. For
// example, consider regular expression literals.
//
// For efficiency, the text associated with textual tokens is stored in two
// separate ways depending on the token. Identifiers use UTF-8 encoding which
// allows them to be slices of the input file without allocating extra memory.
// Strings use UTF-16 encoding so they can represent unicode surrogates
// accurately.
type Lexer struct {
	// ???
	AllComments []logger.Range
	// ???
	CommentsBeforeToken []logger.Range
	// ???
	Identifier MaybeSubstring
	// ???
	LegalCommentsBeforeToken []logger.Range
	// ???
	log logger.Log
	// ???
	source logger.Source
	// ???
	SourceMappingURL logger.Span

	// Escape sequences in string literals are decoded lazily because they are
	// not interpreted inside tagged templates, and tagged templates can contain
	// invalid escape sequences. If the decoded array is nil, the encoded value
	// should be passed to "tryToDecodeEscapeSequences" first.
	decodedStringLiteralOrNil []uint16
	// ???
	encodedStringLiteralText string

	// ???
	errorSuffix string
	// Keeps track of where the lexer currently is.
	tracker logger.LineColumnTracker

	// ???
	encodedStringLiteralStart int

	// ???
	Number float64
	// ???
	current int
	// ???
	start int
	// ???
	end int
	// ???
	ApproximateNewlineCount int
	// ???
	AwaitKeywordLoc logger.Loc
	// ???
	FnOrArrowStartLoc logger.Loc
	// ???
	PreviousBackslashQuoteInJSX logger.Range
	// ???
	LegacyHTMLCommentRange logger.Range
	// ???
	codePoint rune
	// ???
	prevErrorLoc logger.Loc
	// ???
	json JSONFlavor
	// ???
	Token T
	// ???
	HasNewlineBefore bool
	// ???
	HasPureCommentBefore bool
	// ???
	PrevTokenWasAwaitKeyword bool
	// ???
	rescanCloseBraceAsTemplateToken bool
	// ???
	forGlobalName bool

	// The log is disabled during speculative scans that may backtrack
	IsLogDisabled bool
}

// Returns a brand new `Lexer`.
func NewLexer(log logger.Log, source logger.Source) Lexer {
	lexer := Lexer{
		log:               log,
		source:            source,
		tracker:           logger.MakeLineColumnTracker(&source),
		prevErrorLoc:      logger.Loc{Start: -1},
		FnOrArrowStartLoc: logger.Loc{Start: -1},
		json:              NotJSON,
	}
	lexer.step()
	lexer.Next()
	return lexer
}

// Advances the lexer exactly one UTF-8 character.
func (lexer *Lexer) step() {
	codePoint, width := utf8.DecodeRuneInString(lexer.source.Contents[lexer.current:])

	// Use -1 to indicate the end of the file
	if width == 0 {
		codePoint = -1
	}

	// Track the approximate number of newlines in the file so we can preallocate
	// the line offset table in the printer for source maps. The line offset table
	// is the #1 highest allocation in the heap profile, so this is worth doing.
	// This count is approximate because it handles "\n" and "\r\n" (the common
	// cases) but not "\r" or "\u2028" or "\u2029". Getting this wrong is harmless
	// because it's only a preallocation. The array will just grow if it's too small.
	if codePoint == '\n' {
		lexer.ApproximateNewlineCount++
	}

	lexer.codePoint = codePoint
	lexer.end = lexer.current
	lexer.current += width
}

// Reads the current token `T`.
func (lexer *Lexer) Next() {
	lexer.HasNewlineBefore = lexer.end == 0
	lexer.HasPureCommentBefore = false
	lexer.PrevTokenWasAwaitKeyword = false
	lexer.LegalCommentsBeforeToken = lexer.LegalCommentsBeforeToken[:0]
	lexer.CommentsBeforeToken = lexer.CommentsBeforeToken[:0]

	for {
		lexer.start = lexer.end
		lexer.Token = 0

		switch lexer.codePoint {
		case -1: // This indicates the end of the file
			lexer.Token = TEndOfFile

		case '#':
			if lexer.start == 0 && strings.HasPrefix(lexer.source.Contents, "#!") {
				// "#!/usr/bin/env node"
				lexer.Token = THashbang
			hashbang:
				for {
					lexer.step()
					switch lexer.codePoint {
					case '\r', '\n', '\u2028', '\u2029':
						break hashbang

					case -1: // This indicates the end of the file
						break hashbang
					}
				}
				lexer.Identifier = lexer.rawIdentifier()
			} else {
				// "#foo"
				lexer.step()
				if lexer.codePoint == '\\' {
					lexer.Identifier, _ = lexer.scanIdentifierWithEscapes(privateIdentifier)
				} else {
					if !js_ast.IsIdentifierStart(lexer.codePoint) {
						lexer.SyntaxError()
					}
					lexer.step()
					for js_ast.IsIdentifierContinue(lexer.codePoint) {
						lexer.step()
					}
					if lexer.codePoint == '\\' {
						lexer.Identifier, _ = lexer.scanIdentifierWithEscapes(privateIdentifier)
					} else {
						lexer.Identifier = lexer.rawIdentifier()
					}
				}
				lexer.Token = TPrivateIdentifier
			}

		case '\r', '\n', '\u2028', '\u2029':
			lexer.step()
			lexer.HasNewlineBefore = true
			continue

		case '\t', ' ':
			lexer.step()
			continue

		case '(':
			lexer.step()
			lexer.Token = TOpenParen

		case ')':
			lexer.step()
			lexer.Token = TCloseParen

		case '[':
			lexer.step()
			lexer.Token = TOpenBracket

		case ']':
			lexer.step()
			lexer.Token = TCloseBracket

		case '{':
			lexer.step()
			lexer.Token = TOpenBrace

		case '}':
			lexer.step()
			lexer.Token = TCloseBrace

		case ',':
			lexer.step()
			lexer.Token = TComma

		case ':':
			lexer.step()
			lexer.Token = TColon

		case ';':
			lexer.step()
			lexer.Token = TSemicolon

		case '@':
			lexer.step()
			lexer.Token = TAt

		case '~':
			lexer.step()
			lexer.Token = TTilde

		case '?':
			// '?' or '?.' or '??' or '??='
			lexer.step()
			switch lexer.codePoint {
			case '?':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TQuestionQuestionEquals
				default:
					lexer.Token = TQuestionQuestion
				}
			case '.':
				lexer.Token = TQuestion
				current := lexer.current
				contents := lexer.source.Contents

				// Lookahead to disambiguate with 'a?.1:b'
				if current < len(contents) {
					c := contents[current]
					if c < '0' || c > '9' {
						lexer.step()
						lexer.Token = TQuestionDot
					}
				}
			default:
				lexer.Token = TQuestion
			}

		case '%':
			// '%' or '%='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TPercentEquals
			default:
				lexer.Token = TPercent
			}

		case '&':
			// '&' or '&=' or '&&' or '&&='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TAmpersandEquals
			case '&':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TAmpersandAmpersandEquals
				default:
					lexer.Token = TAmpersandAmpersand
				}
			default:
				lexer.Token = TAmpersand
			}

		case '|':
			// '|' or '|=' or '||' or '||='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TBarEquals
			case '|':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TBarBarEquals
				default:
					lexer.Token = TBarBar
				}
			default:
				lexer.Token = TBar
			}

		case '^':
			// '^' or '^='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TCaretEquals
			default:
				lexer.Token = TCaret
			}

		case '+':
			// '+' or '+=' or '++'
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TPlusEquals
			case '+':
				lexer.step()
				lexer.Token = TPlusPlus
			default:
				lexer.Token = TPlus
			}

		case '-':
			// '-' or '-=' or '--' or '-->'
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TMinusEquals
			case '-':
				lexer.step()

				// Handle legacy HTML-style comments
				if lexer.codePoint == '>' && lexer.HasNewlineBefore {
					lexer.step()
					lexer.LegacyHTMLCommentRange = lexer.Range()
					lexer.log.AddID(logger.MsgID_JS_HTMLCommentInJS, logger.Warning, &lexer.tracker, lexer.Range(),
						"Treating \"-->\" as the start of a legacy HTML single-line comment")
				singleLineHTMLCloseComment:
					for {
						switch lexer.codePoint {
						case '\r', '\n', '\u2028', '\u2029':
							break singleLineHTMLCloseComment

						case -1: // This indicates the end of the file
							break singleLineHTMLCloseComment
						}
						lexer.step()
					}
					continue
				}

				lexer.Token = TMinusMinus
			default:
				lexer.Token = TMinus
				if lexer.json == JSON && lexer.codePoint != '.' && (lexer.codePoint < '0' || lexer.codePoint > '9') {
					lexer.Unexpected()
				}
			}

		case '*':
			// '*' or '*=' or '**' or '**='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TAsteriskEquals

			case '*':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TAsteriskAsteriskEquals

				default:
					lexer.Token = TAsteriskAsterisk
				}

			default:
				lexer.Token = TAsterisk
			}

		case '/':
			// '/' or '/=' or '//' or '/* ... */'
			lexer.step()
			if lexer.forGlobalName {
				lexer.Token = TSlash
				break
			}
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TSlashEquals

			case '/':
			singleLineComment:
				for {
					lexer.step()
					switch lexer.codePoint {
					case '\r', '\n', '\u2028', '\u2029':
						break singleLineComment

					case -1: // This indicates the end of the file
						break singleLineComment
					}
				}
				if lexer.json == JSON {
					lexer.addRangeError(lexer.Range(), "JSON does not support comments")
				}
				lexer.scanCommentText()
				continue

			case '*':
				lexer.step()
				startRange := lexer.Range()
			multiLineComment:
				for {
					switch lexer.codePoint {
					case '*':
						lexer.step()
						if lexer.codePoint == '/' {
							lexer.step()
							break multiLineComment
						}

					case '\r', '\n', '\u2028', '\u2029':
						lexer.step()
						lexer.HasNewlineBefore = true

					case -1: // This indicates the end of the file
						lexer.start = lexer.end
						lexer.AddRangeErrorWithNotes(logger.Range{Loc: lexer.Loc()}, "Expected \"*/\" to terminate multi-line comment",
							[]logger.MsgData{lexer.tracker.MsgData(startRange, "The multi-line comment starts here:")})
						panic(LexerPanic{})

					default:
						lexer.step()
					}
				}
				if lexer.json == JSON {
					lexer.addRangeError(lexer.Range(), "JSON does not support comments")
				}
				lexer.scanCommentText()
				continue

			default:
				lexer.Token = TSlash
			}

		case '=':
			// '=' or '=>' or '==' or '==='
			lexer.step()
			switch lexer.codePoint {
			case '>':
				lexer.step()
				lexer.Token = TEqualsGreaterThan
			case '=':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TEqualsEqualsEquals
				default:
					lexer.Token = TEqualsEquals
				}
			default:
				lexer.Token = TEquals
			}

		case '<':
			// '<' or '<<' or '<=' or '<<=' or '<!--'
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TLessThanEquals
			case '<':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TLessThanLessThanEquals
				default:
					lexer.Token = TLessThanLessThan
				}

				// Handle legacy HTML-style comments
			case '!':
				if strings.HasPrefix(lexer.source.Contents[lexer.start:], "<!--") {
					lexer.step()
					lexer.step()
					lexer.step()
					lexer.LegacyHTMLCommentRange = lexer.Range()
					lexer.log.AddID(logger.MsgID_JS_HTMLCommentInJS, logger.Warning, &lexer.tracker, lexer.Range(),
						"Treating \"<!--\" as the start of a legacy HTML single-line comment")
				singleLineHTMLOpenComment:
					for {
						switch lexer.codePoint {
						case '\r', '\n', '\u2028', '\u2029':
							break singleLineHTMLOpenComment

						case -1: // This indicates the end of the file
							break singleLineHTMLOpenComment
						}
						lexer.step()
					}
					continue
				}

				lexer.Token = TLessThan

			default:
				lexer.Token = TLessThan
			}

		case '>':
			// '>' or '>>' or '>>>' or '>=' or '>>=' or '>>>='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				lexer.Token = TGreaterThanEquals
			case '>':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TGreaterThanGreaterThanEquals
				case '>':
					lexer.step()
					switch lexer.codePoint {
					case '=':
						lexer.step()
						lexer.Token = TGreaterThanGreaterThanGreaterThanEquals
					default:
						lexer.Token = TGreaterThanGreaterThanGreaterThan
					}
				default:
					lexer.Token = TGreaterThanGreaterThan
				}
			default:
				lexer.Token = TGreaterThan
			}

		case '!':
			// '!' or '!=' or '!=='
			lexer.step()
			switch lexer.codePoint {
			case '=':
				lexer.step()
				switch lexer.codePoint {
				case '=':
					lexer.step()
					lexer.Token = TExclamationEqualsEquals
				default:
					lexer.Token = TExclamationEquals
				}
			default:
				lexer.Token = TExclamation
			}

		case '\'', '"', '`':
			quote := lexer.codePoint
			needsSlowPath := false
			suffixLen := 1

			if quote != '`' {
				lexer.Token = TStringLiteral
			} else if lexer.rescanCloseBraceAsTemplateToken {
				lexer.Token = TTemplateTail
			} else {
				lexer.Token = TNoSubstitutionTemplateLiteral
			}
			lexer.step()

		stringLiteral:
			for {
				switch lexer.codePoint {
				case '\\':
					needsSlowPath = true
					lexer.step()

					// Handle Windows CRLF
					if lexer.codePoint == '\r' && lexer.json != JSON {
						lexer.step()
						if lexer.codePoint == '\n' {
							lexer.step()
						}
						continue
					}

				case -1: // This indicates the end of the file
					lexer.addRangeError(logger.Range{Loc: logger.Loc{Start: int32(lexer.end)}}, "Unterminated string literal")
					panic(LexerPanic{})

				case '\r':
					if quote != '`' {
						lexer.addRangeError(logger.Range{Loc: logger.Loc{Start: int32(lexer.end)}}, "Unterminated string literal")
						panic(LexerPanic{})
					}

					// Template literals require newline normalization
					needsSlowPath = true

				case '\n':
					if quote != '`' {
						lexer.addRangeError(logger.Range{Loc: logger.Loc{Start: int32(lexer.end)}}, "Unterminated string literal")
						panic(LexerPanic{})
					}

				case '$':
					if quote == '`' {
						lexer.step()
						if lexer.codePoint == '{' {
							suffixLen = 2
							lexer.step()
							if lexer.rescanCloseBraceAsTemplateToken {
								lexer.Token = TTemplateMiddle
							} else {
								lexer.Token = TTemplateHead
							}
							break stringLiteral
						}
						continue stringLiteral
					}

				case quote:
					lexer.step()
					break stringLiteral

				default:
					// Non-ASCII strings need the slow path
					if lexer.codePoint >= 0x80 {
						needsSlowPath = true
					} else if lexer.json == JSON && lexer.codePoint < 0x20 {
						lexer.SyntaxError()
					}
				}
				lexer.step()
			}

			text := lexer.source.Contents[lexer.start+1 : lexer.end-suffixLen]

			if needsSlowPath {
				// Slow path
				lexer.decodedStringLiteralOrNil = nil
				lexer.encodedStringLiteralStart = lexer.start + 1
				lexer.encodedStringLiteralText = text
			} else {
				// Fast path
				n := len(text)
				copy := make([]uint16, n)
				for i := 0; i < n; i++ {
					copy[i] = uint16(text[i])
				}
				lexer.decodedStringLiteralOrNil = copy
			}

			if quote == '\'' && (lexer.json == JSON || lexer.json == TSConfigJSON) {
				lexer.addRangeError(lexer.Range(), "JSON strings must use double quotes")
			}

		// Note: This case is hot in profiles
		case '_', '$',
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
			'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
			'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z':
			// This is a fast path for long ASCII identifiers. Doing this in a loop
			// first instead of doing "step()" and "js_ast.IsIdentifierContinue()" like we
			// do after this is noticeably faster in the common case of ASCII-only
			// text. For example, doing this sped up end-to-end consuming of a large
			// TypeScript type declaration file from 97ms to 79ms (around 20% faster).
			contents := lexer.source.Contents
			n := len(contents)
			i := lexer.current
			for i < n {
				c := contents[i]
				if (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') && (c < '0' || c > '9') && c != '_' && c != '$' {
					break
				}
				i++
			}
			lexer.current = i

			// Now do the slow path for any remaining non-ASCII identifier characters
			lexer.step()
			if lexer.codePoint >= 0x80 {
				for js_ast.IsIdentifierContinue(lexer.codePoint) {
					lexer.step()
				}
			}

			// If there's a slash, then we're in the extra-slow (and extra-rare) case
			// where the identifier has embedded escapes
			if lexer.codePoint == '\\' {
				lexer.Identifier, lexer.Token = lexer.scanIdentifierWithEscapes(normalIdentifier)
				break
			}

			// Otherwise (if there was no escape) we can slice the code verbatim
			lexer.Identifier = lexer.rawIdentifier()
			lexer.Token = Keywords[lexer.Raw()]
			if lexer.Token == 0 {
				lexer.Token = TIdentifier
			}

		case '\\':
			lexer.Identifier, lexer.Token = lexer.scanIdentifierWithEscapes(normalIdentifier)

		case '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			lexer.parseNumericLiteralOrDot()

		default:
			// Check for unusual whitespace characters
			if js_ast.IsWhitespace(lexer.codePoint) {
				lexer.step()
				continue
			}

			if js_ast.IsIdentifierStart(lexer.codePoint) {
				lexer.step()
				for js_ast.IsIdentifierContinue(lexer.codePoint) {
					lexer.step()
				}
				if lexer.codePoint == '\\' {
					lexer.Identifier, lexer.Token = lexer.scanIdentifierWithEscapes(normalIdentifier)
				} else {
					lexer.Token = TIdentifier
					lexer.Identifier = lexer.rawIdentifier()
				}
				break
			}

			lexer.end = lexer.current
			lexer.Token = TSyntaxError
		}

		return
	}
}
