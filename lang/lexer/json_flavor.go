package lexer

// Enumerates every variety of JSON supported by the Denim lexer.
type JSONFlavor uint8

const (
	// Specification: https://json.org/
	JSON JSONFlavor = iota

	// TypeScript's JSON superset is not documented but appears to allow:
	// - Comments: https://github.com/microsoft/TypeScript/issues/4987
	// - Trailing commas
	// - Full JS number syntax
	TSConfigJSON

	// This is used by the JavaScript lexer
	NotJSON
)
