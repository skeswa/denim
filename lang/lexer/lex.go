package lexer

import "fmt"

// Lexes the token begun by the current rune of the [Lexer] as whitespace.
func (lexer *Lexer) lexWhitespace() {
	if lexer.isDebug && !isRuneWhitespace(lexer.previousRune) {
		panic(fmt.Sprintf("Expected whitespace, but got \"%c\"", lexer.previousRune))
	}

	// Keep going until we hit a rune that is not whitespace.
	for lexer.hasNext() && isRuneWhitespace(lexer.peek(1)) {
		lexer.bump()
	}

	// TODO(skeswa): formToken(TokenKind)
}
