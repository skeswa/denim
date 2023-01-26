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

// Maps each kind of `T` to a short, human-friendly label.
var tokenToString = map[T]string{
	TEndOfFile:   "end of file",
	TSyntaxError: "syntax error",

	// Literals
	TDoubleLiteral: "double",
	TIntLiteral:    "int",
	TRuneLiteral:   "rune",
	TStringLiteral: "string",

	// Punctuation
	TAmpersand:          "\"&\"",
	TAmpersandAmpersand: "\"&&\"",
	TAsterisk:           "\"*\"",
	TAsteriskAsterisk:   "\"**\"",
	TAt:                 "\"@\"",
	TBar:                "\"|\"",
	TBarBar:             "\"||\"",
	TCloseBrace:         "\"}\"",
	TCloseBracket:       "\"]\"",
	TCloseParen:         "\")\"",
	TColon:              "\":\"",
	TComma:              "\",\"",
	TDot:                "\".\"",
	TDotDot:             "\"..\"",
	TDotDotDot:          "\"...\"",
	TEquals:             "\"=\"",
	TEqualsEquals:       "\"==\"",
	TEqualsGreaterThan:  "\"=>\"",
	TExclamation:        "\"!\"",
	TExclamationEquals:  "\"!=\"",
	TGreaterThan:        "\">\"",
	TGreaterThanEquals:  "\">=\"",
	TLessThan:           "\"<\"",
	TLessThanEquals:     "\"<=\"",
	TMinus:              "\"-\"",
	TMinusGreaterThan:   "\"->\"",
	TOpenBrace:          "\"{\"",
	TOpenBracket:        "\"[\"",
	TOpenParen:          "\"(\"",
	TPercent:            "\"%\"",
	TPlus:               "\"+\"",
	TQuestionDot:        "\"?.\"",
	TQuestionQuestion:   "\"??\"",
	TSemicolon:          "\";\"",
	TSlash:              "\"/\"",
	TTildeSlash:         "\"~\"",

	// Reserved words
	TAs:       "\"as\"",
	TAsync:    "\"async\"",
	TAwait:    "\"await\"",
	TBreak:    "\"break\"",
	TContinue: "\"continue\"",
	TElse:     "\"else\"",
	TEnum:     "\"enum\"",
	TExtern:   "\"extern\"",
	TFalse:    "\"false\"",
	TFn:       "\"fn\"",
	TFor:      "\"for\"",
	TFork:     "\"fork\"",
	TFrom:     "\"from\"",
	TIf:       "\"if\"",
	TImpl:     "\"impl\"",
	TIn:       "\"in\"",
	TIs:       "\"is\"",
	TLet:      "\"let\"",
	TLoop:     "\"loop\"",
	TMatch:    "\"match\"",
	TMod:      "\"mod\"",
	TPub:      "\"pub\"",
	TReturn:   "\"return\"",
	TSelf:     "\"self\"",
	TSelfType: "\"selftype\"",
	TShow:     "\"show\"",
	TStruct:   "\"struct\"",
	TTandem:   "\"tandem\"",
	TTrait:    "\"trait\"",
	TTrue:     "\"true\"",
	TTry:      "\"try\"",
	TType:     "\"type\"",
	TUnknown:  "\"unknown\"",
	TUse:      "\"use\"",
	TVoid:     "\"void\"",
	TWhere:    "\"where\"",
	TWhile:    "\"while\"",
}
