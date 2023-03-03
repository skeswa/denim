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

	depth := 1

loop:
	for ; !lexer.IsExhausted(); lexer.bump() {
		switch lexer.currentRune {
		case '/':
			if lexer.peek(1) == '*' {
				lexer.bump()

				// We found another `/*`, so let's bump the depth.
				depth += 1
			}
		case '*':
			if lexer.peek(1) == '/' {
				lexer.bump()

				// We found a `*/`, so let's drop the depth.
				depth -= 1

				// If the depth is zero, that means we have captured `/* ??? */`. That
				// means we should get outta here.
				if depth == 0 {
					break loop
				}
			}
		}
	}

	var metadata *token.TokenMetadata
	if isUnterminated := depth != 0; isUnterminated {
		metadata = &token.TokenMetadata{BlockCommentIsUnterminated: isUnterminated}
	}

	return token.BlockComment, metadata
}

// Lexes a line comment token at the current position of the [Lexer].
func (lexer *Lexer) lexLineComment() (token.TokenKind, *token.TokenMetadata) {
	lexer.expectRune('/')
	lexer.expectRuneNext('/')

	lexer.bump()

	isDocComment := false

	// Check for a triple slash; triple slash is a "doc comment". Ignore
	// quadrouple slash+ because it could just be ASCII art
	// e.g. `///// CONSTANTS /////`.
	if lexer.peek(1) == '/' && lexer.peek(2) != '/' {
		isDocComment = true
	}

	for !lexer.IsExhausted() && lexer.peek(1) != '\n' {
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
	for !lexer.IsExhausted() && isRuneWhitespace(lexer.peek(1)) {
		lexer.bump()
	}

	return token.Whitespace, nil
}

// Tries to lex a shebang token at the current position of the [Lexer], doing
// nothing and returning `false` if it does not find anything.
func (lexer *Lexer) maybeLexShebang() (token.TokenKind, *token.TokenMetadata, bool) {
	if lexer.peek(0) != '#' || lexer.peek(1) != '!' {
		return token.Shebang, nil, false
	}

	lexer.bump()

	for !lexer.IsExhausted() && isRuneWhitespace(lexer.peek(1)) {
		lexer.bump()
	}

	lexer.bump()

	interpreterIndex := lexer.currentIndex

	for !lexer.IsExhausted() && lexer.peek(1) != '\n' {
		lexer.bump()
	}

	return token.Shebang, &token.TokenMetadata{ShebangInterpreterIndex: interpreterIndex}, true
}
