//go:generate go run github.com/abice/go-enum --names --nocomments --noprefix

package literal

// Enumerates every kind of literal e.g. `Int`, `String`, etc.
/*
ENUM(
Float 			// Describes a floating point literal (e.g. `-12.80001`, `1.2e-12`).
Int					// Describes an integer literal (e.g. `11`, `-0x120`, `1_000_000`).
RawString		// Describes a "raw" text literal (e.g. `r"abc"#`, `r#"xyz"#`).
Rune				// Describes a character literal (e.g. `'a'`, `'👖'`).
String			// Describes a single or multi-line text literal (e.g. `"abc"`, `"""xyz"""`).
)
*/
type LiteralKind uint16
