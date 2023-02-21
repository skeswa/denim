//go:generate go run github.com/abice/go-enum --names --noprefix

package syntaxkind

// Enumerates every kind of syntax node e.g. `Ident`, `KeywordUse`, etc.
/*
ENUM(
comment
whitespace
)
*/
type SyntaxKind uint16
