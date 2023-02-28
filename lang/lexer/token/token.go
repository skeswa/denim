package token

// Atomic unit of Denim source code.
//
// A "token" is best thought of as a sequence of one or more characters that
// represents a single idea e.g. "block comment", or "question mark".
type Token struct {
	// Byte-wise offset of this [Token] within its surrounding snippet of source
	// code.
	Index int
	// Categorizes this [Token].
	Kind TokenKind
	// Number of bytes taken up by this [Token] within its surrounding snippet of
	// source code.
	Length int
	// Additional metadata optionally attached to this [Token] to add more
	// context.
	//
	// Most kinds of tokens will not need metadata, and so this field will be
	// `nil`.
	Metadata *TokenMetadata
}
