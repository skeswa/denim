use crate::literal_kind::LiteralKind;

/// Enum representing common lexeme types.
///
/// For reference, see
/// [rustc-lexer's `TokenKind`](https://github.com/rust-lang/rust/blob/d9c8274fb7e2c9087c27a87bf4d85bf1d78cd1e0/compiler/rustc_lexer/src/lib.rs#L56).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    ///////////////////////////////// GENERIC //////////////////////////////////
    //
    /// Token that represents the end of input, usually represented by the EOF
    /// character.
    End,
    /// Unknown token, not expected by the lexer, e.g. "№".
    Unknown,

    ///////////////////////////// MULTI-CHARACTER //////////////////////////////
    //
    /// `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment {
        /// `true` if this block comment ends correctly with a `*/`.
        is_terminated: bool,
    },
    /// Any identifier (e.g. a variable name or a keyword).
    ///
    /// At this step, keywords are also considered identifiers.
    Ident,
    /// Like `Ident`, but containing invalid unicode codepoints.
    InvalidIdent,
    /// `// comment`.
    LineComment {
        /// `true` if this `LineComment` documents a particular symbol.
        is_doc_comment: bool,
    },
    /// Syntax that defines actual, usable data directly in source code.
    ///
    /// Examples: `12u8`, `1.0e-40`, `"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [LiteralKind] for more details.
    Literal {
        /// Specifies the general type of `Literal`.
        kind: LiteralKind,
    },
    /// Unrecognized prefix, like `foo#`, `foo'`, `foo"`.
    ///
    /// NOTE: only the prefix (`foo`) is included in the token, not the
    /// separator (which is lexed as its own distinct token).
    UnknownPrefix,
    /// Any whitespace character sequence.
    Whitespace,

    //////////////////////////// SINGLE-CHARACTER //////////////////////////////
    //
    /// "&".
    Amp,
    /// "@".
    At,
    /// "!".
    Bang,
    /// "^".
    Caret,
    /// "}".
    CloseBrace,
    /// "]".
    CloseBracket,
    /// ")".
    CloseParen,
    /// ":".
    Colon,
    /// ",".
    Comma,
    /// "$".
    Dollar,
    /// ".".
    Dot,
    /// "=".
    Eq,
    /// ">".
    Gt,
    /// "<".
    Lt,
    /// "-".
    Minus,
    /// "{".
    OpenBrace,
    /// "[".
    OpenBracket,
    /// "(".
    OpenParen,
    /// "%".
    Percent,
    /// "|".
    Pipe,
    /// "+".
    Plus,
    /// "#".
    Pound,
    /// "?".
    Question,
    /// ";".
    Semi,
    /// "/".
    Slash,
    /// "*".
    Star,
    /// "~".
    Tilde,
}
