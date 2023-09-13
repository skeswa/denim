use crate::token_kind::TokenKind;

/// Parsed token.
///
/// It doesn't contain information about data that has been parsed, only the
/// type of the token and its size.
///
/// For reference, see
/// [rustc-lexer's `Token`](https://github.com/rust-lang/rust/blob/d9c8274fb7e2c9087c27a87bf4d85bf1d78cd1e0/compiler/rustc_lexer/src/lib.rs#L43).
#[derive(Debug)]
pub struct Token {
    /// Determines the type of this `Token`.
    pub kind: TokenKind,
    /// How many unicode code points are represented by this `Token`.
    pub len: u32,
}
