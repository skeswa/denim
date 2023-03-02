package lexer

import (
	"fmt"
	"unicode/utf8"

	"github.com/skeswa/denim/lang/lexer/token"
)

// Low-level Denim lexer.
//
// [Lexer] interprets Denim source code as a sequence of tokens.
type Lexer struct {
	// Position of the [currentRune] within the [source] string.
	currentIndex int
	// Unicode code point that terminates at [currentIndex] within the [source]
	// string.
	//
	// Values less than `0` indicate the end of the [source] string.
	currentRune rune
	// True if this [Lexer] is running in debug mode.
	isDebug bool
	// Position of the next `rune` within the [source] string.
	nextIndex int
	// Previous value of [currentRune], or `0` if there has not yet been at least
	// two runes.
	previousRune rune
	// Source code text being lexed by this [Lexer].
	source string
}

// Instantiates a brand new [Lexer] for the specified snippet of source code
// text.
func NewLexer(source string) Lexer {
	return Lexer{source: source}
}

// Returns true if this [Lexer] has no source to lex.
func (lexer *Lexer) IsTerminated() bool {
	return lexer.currentRune == endOfSourceRune
}

// Advances the [Lexer] forward one [token.Token] in the source string,
// returning it.
func (lexer *Lexer) NextToken() token.Token {
	lexer.bump()

	initialIndex := lexer.currentIndex
	tokenKind, tokenMetadata := lexer.lex()

	tokenLength := lexer.currentIndex - initialIndex
	if tokenKind != token.End {
		// Because we only move the current index to the next token on the next call
		// to this method, length will always be one too small. So we correct that
		// in all cases except the one where this method should not be called again.
		tokenLength += 1
	}

	return token.Token{
		Index:    initialIndex,
		Kind:     tokenKind,
		Length:   tokenLength,
		Metadata: tokenMetadata,
	}
}

const (
	// Rune used to signify the end of the source string.
	endOfSourceRune rune = -1
)

// Advances the [Lexer] forward one `rune` in the source string, returning that
// `rune`.
//
// Note that this method does not advance to the next token - for that, see
// [NextToken].
func (lexer *Lexer) bump() rune {
	nextRune, nextRuneWidth := utf8.DecodeRuneInString(lexer.source[lexer.nextIndex:])

	// A width of `0` indicates the end of the `string`.
	if nextRuneWidth == 0 {
		nextRune = endOfSourceRune
	}

	lexer.previousRune = lexer.currentRune

	lexer.currentIndex = lexer.nextIndex
	lexer.currentRune = nextRune
	lexer.nextIndex += nextRuneWidth

	return nextRune
}

// Asserts that the rune that is [offset] runes ahead of the current one is
// [expectedRune], panicking otherwise.
func (lexer *Lexer) expectRuneAhead(expectedRune rune, offset int) {
	if !lexer.isDebug {
		return
	}

	currentRune := lexer.peek(offset)
	if currentRune == expectedRune {
		return
	}

	panic(fmt.Sprintf("Expected '%c', but found '%c'", expectedRune, currentRune))
}

// Asserts that the current rune of this [Lexer] is [expectedRune], panicking
// otherwise.
func (lexer *Lexer) expectRune(expectedRune rune) {
	lexer.expectRuneAhead(expectedRune, 0)
}

// Asserts that the next rune of this [Lexer] is [expectedRune], panicking
// otherwise.
func (lexer *Lexer) expectRuneNext(expectedRune rune) {
	lexer.expectRuneAhead(expectedRune, 1)
}

// Looks ahead of the current rune by the specified offset, returning the rune
// at that position.
func (lexer *Lexer) peek(offset int) rune {
	// TODO(skeswa): it might make sense to optimize this a bit by caching the
	// lookahead runes.

	var (
		currentIndex  = lexer.nextIndex
		nextRune      = lexer.currentRune
		nextRuneWidth int
	)

	for i := 0; i < offset; i += 1 {
		nextRune, nextRuneWidth = utf8.DecodeRuneInString(lexer.source[currentIndex:])

		if nextRuneWidth == 0 {
			nextRune = endOfSourceRune

			break
		}

		currentIndex += nextRuneWidth
		i += 1
	}

	return nextRune
}
