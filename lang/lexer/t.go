package lexer

// Here, `T` is short for "token".
//
// This parlance is stolen from esbuild's `js_lexer`.
type T uint8

// If you add a new token, remember to add it to "tokenToString" too
const (
	TEndOfFile T = iota
	TSyntaxError

	// Literals
	TDoubleLiteral // Contents are in lexer.Double (float64)
	TIntLiteral    // Contents are in lexer.Int (int64)
	TRuneLiteral   // Contents are in lexer.RuneLiteral (uint16)
	TStringLiteral // Contents are in lexer.StringLiteral ([]uint16)

	// Pseudo-literals
	TStringInterpolationHead   // Contents are in lexer.StringLiteral ([]uint16)
	TStringInterpolationMiddle // Contents are in lexer.StringLiteral ([]uint16)
	TStringInterpolationTail   // Contents are in lexer.StringLiteral ([]uint16)

	// Punctuation
	TAmpersand
	TAmpersandAmpersand
	TAsterisk
	TAsteriskAsterisk
	TAt
	TBar
	TBarBar
	TCloseBrace
	TCloseBracket
	TCloseParen
	TColon
	TComma
	TDot
	TDotDot
	TDotDotDot
	TEquals
	TEqualsEquals
	TEqualsGreaterThan
	TExclamation
	TExclamationEquals
	TGreaterThan
	TGreaterThanEquals
	TLessThan
	TLessThanEquals
	TMinus
	TMinusGreaterThan
	TOpenBrace
	TOpenBracket
	TOpenParen
	TPercent
	TPlus
	TQuestionDot
	TQuestionQuestion
	TSemicolon
	TSlash
	TTildeSlash

	// Identifiers
	TIdentifier     // Contents are in lexer.Identifier (string)
	TEscapedKeyword // A keyword that has been escaped as an identifer

	// Reserved words
	TAs
	TAsync
	TAwait
	TBreak
	TContinue
	TElse
	TEnum
	TExtern
	TFalse
	TFn
	TFor
	TFork
	TFrom
	TIf
	TImpl
	TIn
	TIs
	TLet
	TLoop
	TMatch
	TMod
	TPub
	TReturn
	TSelf
	TSelfType
	TShow
	TStruct
	TTandem
	TTrait
	TTrue
	TTry
	TType
	TUnknown
	TUse
	TVoid
	TWhere
	TWhile
)
