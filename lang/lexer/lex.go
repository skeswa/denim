package lexer

import (
	"github.com/skeswa/denim/lang/lexer/token"
)

// Lexes whichever token begins at the current position of the [Lexer],
// returning the kind of that token.
//
// This code is ported almost exactly from `rustc_lexer::Cursor::advance_token`.
func (lexer *Lexer) lex() (token.TokenKind, *token.TokenMetadata) {
	currentRune := lexer.currentRune

	if currentRune == endOfSourceRune {
		return token.End, nil
	}

	if currentRune == '/' {
		switch lexer.peek(1) {
		case '/':
			return lexer.lexLineComment()
		case '*':
			return lexer.lexBlockComment()
		default:
			return token.Slash, nil
		}
	}

	if isRuneWhitespace(currentRune) {
		return lexer.lexWhitespace()
	}

	return token.Unknown, nil
}

// Lexes a block comment token at the current position of the [Lexer].
func (lexer *Lexer) lexBlockComment() (token.TokenKind, *token.TokenMetadata) {
	lexer.expectRune('/')
	lexer.expectRuneNext('*')

	lexer.bump()

	panic("todo")
}

// Lexes a line comment token at the current position of the [Lexer].
func (lexer *Lexer) lexLineComment() (token.TokenKind, *token.TokenMetadata) {
	lexer.expectRune('/')
	lexer.expectRuneNext('/')

	lexer.bump()

	isDocComment := false

	// Check for a triple slash; triple slash is a "doc comment".
	if nextRune := lexer.peek(1); nextRune == '/' {
		isDocComment = true
	}

	for lexer.hasNext() && lexer.peek(1) != '\n' {
		lexer.bump()
	}

	var metadata *token.TokenMetadata
	if isDocComment {
		metadata = &token.TokenMetadata{LineCommentIsDocComment: isDocComment}
	}

	return token.LineComment, metadata
}

// Lexes a whitespace token at the current position of the [Lexer].
func (lexer *Lexer) lexWhitespace() (token.TokenKind, *token.TokenMetadata) {
	lexer.expectRune(' ')

	// Keep going until we hit a rune that is not whitespace.
	for lexer.hasNext() && isRuneWhitespace(lexer.peek(1)) {
		lexer.bump()
	}

	return token.Whitespace, nil
}
