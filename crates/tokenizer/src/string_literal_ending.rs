/// Enumerates every way that a double-quoted string literal can end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringLiteralEnding {
    /// Represents a string literal that ends.
    TerminatedString {
        /// `true` if the terminated string literal is a multi-line string
        /// literal.
        is_multiline: bool,
    },
    /// Represents a string literal that doesn't end.
    UnterminatedString,
}
