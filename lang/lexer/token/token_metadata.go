package token

import "strings"

// Additional metadata attached to some kinds of [Token].
type TokenMetadata struct {
	// True if the surrounding [Token] is of kind [BlockComment], and is has more
	// comment "openings" than "closings".
	//
	// Is only set to a non-zero value if the surrounding [Token] is of kind
	// [BlockComment].
	BlockCommentIsUnterminated bool
	// True if the surrounding [Token] is of kind [LineComment], and is classified
	// as a "doc comment".
	//
	// Is only set to a non-zero value if the surrounding [Token] is of kind
	// [LineComment].
	LineCommentIsDocComment bool
}

// Represets [tokenMetadata] as a string.
func (tokenMetadata *TokenMetadata) String() string {
	var stringBuilder strings.Builder

	stringBuilder.WriteRune('{')

	isFirst := true

	if tokenMetadata.BlockCommentIsUnterminated {
		if isFirst {
			isFirst = false
		}

		stringBuilder.WriteString(" BlockComment: { IsUnterminated: true } ")
	}
	if tokenMetadata.LineCommentIsDocComment {
		if isFirst {
			isFirst = false
		} else {
			stringBuilder.WriteRune(',')
		}

		stringBuilder.WriteString(" LineComment: { IsDocComment: true } ")
	}

	stringBuilder.WriteRune('}')

	return stringBuilder.String()
}
