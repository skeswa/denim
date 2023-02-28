//go:generate go run github.com/abice/go-enum --names --nocomments --noprefix

package token

// Enumerates every kind of token e.g. `Semi`, `Dot`, etc.
/*
ENUM(
BlockComment 	// Describes a `/ * block comment * /`.
End						// Marks the end of any source snippet.
LineComment 	// Describes a `// line comment`.
Unknown				// Describes a token not expected by the lexer, e.g. "№".
Slash					// Describes a `/` (typically used in division).
Whitespace		// Describes any kind of whitespace (e.g. \n, \t, etc.).
)
*/
type TokenKind uint16
