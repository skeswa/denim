package text

// Returns `true` if `codePoint` is whitespace.
//
// See the "White Space Code Points" table in the ECMAScript standard.
func IsWhitespace(codePoint rune) bool {
	switch codePoint {
	case
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
