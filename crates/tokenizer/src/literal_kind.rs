use crate::{numeric_base::NumericBase, string_literal_ending::StringLiteralEnding};

/// Enum representing the literal types supported by the tokenizer.
///
/// Note that the suffix is *not* considered when deciding the `LiteralKind` in
/// this type. This means that float literals like `1f32` are classified by this
/// type as `Int`. (Compare against `rustc_ast::token::LitKind` and
/// `rustc_ast::ast::LitKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// "'a'", "'\\'", "'''", "';"
    Char {
        /// `true` if this literal ends correctly with a `'`.
        is_terminated: bool,
    },
    /// "12.34f32", "1e3", but not "1f32".
    Float {
        /// Base of this float literal.
        base: NumericBase,
        /// `true` if this float literal has no digits following its exponential
        /// prefix (e.g. `1e`).
        is_empty: bool,
    },
    /// "12_u8", "0o100", "0b120i99", "1f32".
    Int {
        /// Base of this int literal.
        base: NumericBase,
        /// `true` if this int literal has no digits following its base prefix
        /// (e.g. `0x` or `0b`).
        is_empty: bool,
    },
    /// "r"abc"", "r#"abc"#", "r####"ab"###"c"####", "r#"a".
    RawStr {
        /// Counts the number of `#` characters used to define the raw string.
        ///
        /// [None] indicates an invalid literal.
        pound_count: Option<u8>,
    },
    /// ""abc"", ""abc"
    Str {
        /// Describes how this string literal ends.
        ending: StringLiteralEnding,
    },
}
