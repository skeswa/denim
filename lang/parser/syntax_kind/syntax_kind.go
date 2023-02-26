//go:generate go run github.com/abice/go-enum --names --nocomments --noprefix

package syntaxkind

// Enumerates every kind of syntax node e.g. `Ident`, `KeywordUse`, etc.
/*
ENUM(
Comment 			// Describes any kind of non-doc comment
SourceFile		// Root token for any file
Whitespace		// Describes any kind of whitespace (e.g. \n, \t, etc.)
)
*/
type SyntaxKind uint16
