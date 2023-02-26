package lexer

import (
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

// Advances the [Lexer] forward one [token.Token] in the source string,
// returning it.
func (lexer *Lexer) NextToken() token.Token {
	firstRune, firstRuneIndex := lexer.bump()

	if firstRune == endOfSourceRune {
		return token.Token{
			Index:  firstRuneIndex,
			Kind:   token.Unknown,
			Length: 0,
		}
	}

	panic("askdjh")
}

const (
	// Rune used to signify the end of the source string.
	endOfSourceRune rune = -1
)

// Advances the [Lexer] forward one `rune` in the source string, returning it
// followed by its index.
//
// Note that this method does not advance to the next token - for that, see
// [NextToken].
func (lexer *Lexer) bump() (rune, int) {
	nextRune, nextRuneWidth := utf8.DecodeRuneInString(lexer.source[lexer.nextIndex:])

	// A width of `0` indicates the end of the `string`.
	if nextRuneWidth == 0 {
		nextRune = endOfSourceRune
	}

	lexer.previousRune = lexer.currentRune

	lexer.currentIndex = lexer.nextIndex
	lexer.currentRune = nextRune
	lexer.nextIndex += nextRuneWidth

	return nextRune, lexer.currentIndex
}

// Returns true if this [Lexer] has more source to lex.
func (lexer *Lexer) hasNext() bool {
	return lexer.currentRune != endOfSourceRune
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
