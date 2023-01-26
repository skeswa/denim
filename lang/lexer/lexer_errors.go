package lexer

import (
	"fmt"
	"strings"
	"unicode/utf8"

	"github.com/skeswa/denim/lang/logger"
	"github.com/skeswa/denim/lang/text"
)

// `struct` that gets emitted when the `Lexer` panics.
type LexerPanic struct{}

// Logs an error message pertaining to the source at the specified range `r`
// along with additional details.
func (lexer *Lexer) AddRangeErrorWithNotes(r text.Range, text string, notes []logger.MsgData) {
	// Don't report multiple errors in the same spot
	if r.Loc == lexer.prevErrorLoc {
		return
	}
	lexer.prevErrorLoc = r.Loc

	if !lexer.IsLogDisabled {
		lexer.log.AddErrorWithNotes(&lexer.tracker, r, text, notes)
	}
}

// Logs an error message indicating that Denim expected to see `token` instead
// of what it found.
func (lexer *Lexer) Expected(token T) {
	if text, ok := tokenToString[token]; ok {
		lexer.ExpectedString(text)
	} else {
		lexer.Unexpected()
	}
}

// Logs an error message indicating that Denim expected to see something
// described by the provided `text` instead of what it found.
func (lexer *Lexer) ExpectedString(text string) {
	found := fmt.Sprintf("%q", lexer.Raw())
	if lexer.start == len(lexer.source.Contents) {
		found = "end of file"
	}

	suggestion := ""
	if strings.HasPrefix(text, "\"") && strings.HasSuffix(text, "\"") {
		suggestion = text[1 : len(text)-1]
	}

	lexer.addRangeErrorWithSuggestion(lexer.Range(), fmt.Sprintf("Expected %s%s but found %s", text, lexer.errorSuffix, found), suggestion)
	panic(LexerPanic{})
}

// Logs an error message to do with erroneous syntax before panicking.
func (lexer *Lexer) SyntaxError() {
	loc := text.Loc{Start: int32(lexer.end)}
	message := "Unexpected end of file"
	if lexer.end < len(lexer.source.Contents) {
		c, _ := utf8.DecodeRuneInString(lexer.source.Contents[lexer.end:])
		if c < 0x20 {
			message = fmt.Sprintf("Syntax error \"\\x%02X\"", c)
		} else if c >= 0x80 {
			message = fmt.Sprintf("Syntax error \"\\u{%x}\"", c)
		} else if c != '"' {
			message = fmt.Sprintf("Syntax error \"%c\"", c)
		} else {
			message = "Syntax error '\"'"
		}
	}
	lexer.addRangeError(text.Range{Loc: loc}, message)
	panic(LexerPanic{})
}

// Logs an error message indicating that Denim expected to see something other
// than what it found.
func (lexer *Lexer) Unexpected() {
	found := fmt.Sprintf("%q", lexer.Raw())
	if lexer.start == len(lexer.source.Contents) {
		found = "end of file"
	}
	lexer.addRangeError(lexer.Range(), fmt.Sprintf("Unexpected %s%s", found, lexer.errorSuffix))
	panic(LexerPanic{})
}

// Logs an error at the specified range `r` and `text`.
func (lexer *Lexer) addRangeError(r text.Range, text string) {
	// Don't report multiple errors in the same spot
	if r.Loc == lexer.prevErrorLoc {
		return
	}
	lexer.prevErrorLoc = r.Loc

	if !lexer.IsLogDisabled {
		lexer.log.AddError(&lexer.tracker, r, text)
	}
}

// Logs an error at the specified range `r` and `text` with an accompanying
// `suggestion` that user might find helpful.
func (lexer *Lexer) addRangeErrorWithSuggestion(r text.Range, text string, suggestion string) {
	// Don't report multiple errors in the same spot
	if r.Loc == lexer.prevErrorLoc {
		return
	}
	lexer.prevErrorLoc = r.Loc

	if !lexer.IsLogDisabled {
		data := lexer.tracker.MsgData(r, text)
		data.Location.Suggestion = suggestion
		lexer.log.AddMsg(logger.Msg{Kind: logger.Error, Data: data})
	}
}
