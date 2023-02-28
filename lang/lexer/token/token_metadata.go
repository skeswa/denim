package token

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
