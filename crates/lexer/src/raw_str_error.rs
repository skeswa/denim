/// Enumerates everything that can go wrong when interpreting a string token.
///
/// `RawStrError` is a fork of [rustc-lexer's version](https://github.com/rust-lang/rust/blob/d9c8274fb7e2c9087c27a87bf4d85bf1d78cd1e0/compiler/rustc_lexer/src/lib.rs#L202C1-L212C2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawStrError {
    /// Non `#` characters exist between `r` and `"`, e.g. `r##~"abcde"##`
    InvalidStarter { bad_char: char },
    /// The string was not terminated, e.g. `r###"abcde"##`.
    /// `possible_terminator_offset` is the number of characters after `r` or
    /// `br` where they may have intended to terminate it.
    NoTerminator {
        expected: u32,
        found: u32,
        possible_terminator_offset: Option<u32>,
    },
    /// More than 255 `#`s exist.
    TooManyDelimiters { found: u32 },
}
