package lexer

// Returns true if the specified unicode [codePoint] is classified as whitespace
// by the Denim langauge.
//
// This code is lifted from a combination of esbuild's `js_lexer.IsWhitespace()`
// and the Rust language lexer's `rustc_lexer::is_whitespace()`.
func isRuneWhitespace(codePoint rune) bool {
	switch codePoint {
	case
		// Usual ASCII suspects:
		'\u000A', // newline
		'\u0009', // character tabulation
		'\u000B', // line tabulation
		'\u000C', // form feed
		'\u0020', // space
		'\u00A0', // no-break space

		// Unicode "Space_Separator" code points
		'\u1680', // ogham space mark
		'\u2000', // en quad
		'\u2001', // em quad
		'\u2002', // en space
		'\u2003', // em space
		'\u2004', // three-per-em space
		'\u2005', // four-per-em space
		'\u2006', // six-per-em space
		'\u2007', // figure space
		'\u2008', // punctuation space
		'\u2009', // thin space
		'\u200A', // hair space
		'\u202F', // narrow no-break space
		'\u205F', // medium mathematical space
		'\u3000', // ideographic space

		'\uFEFF': // zero width non-breaking space
		return true

	default:
		return false
	}
}
