use tokenizer::{Mode, StringLiteralEnding};

use crate::{
    error_util::{error_to_diagnostic_message, unescape_string_error_message},
    tokenized_str::TokenizedStr,
    tokenizer_error::TokenizerError,
    SyntaxKind::{self, *},
    T,
};

pub(crate) struct TokenConverter<'a> {
    pub(crate) res: TokenizedStr<'a>,
    pub(crate) offset: usize,
}

impl<'a> TokenConverter<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        Self {
            res: TokenizedStr { text, kind: Vec::new(), start: Vec::new(), error: Vec::new() },
            offset: 0,
        }
    }

    pub(crate) fn finalize_with_eof(mut self) -> TokenizedStr<'a> {
        self.res.push(EOF, self.offset);
        self.res
    }

    pub(crate) fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<&str>) {
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some(err) = err {
            let token = self.res.len() as u32;
            let msg = err.to_owned();
            self.res.error.push(TokenizerError { msg, token });
        }
    }

    pub(crate) fn extend_token(&mut self, kind: &tokenizer::TokenKind, token_text: &str) {
        // A note on an intended tradeoff:
        // We drop some useful information here (see patterns with double dots `..`)
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let mut err = "";

        let syntax_kind = {
            match kind {
                tokenizer::TokenKind::LineComment { is_doc_comment: _ } => COMMENT,
                tokenizer::TokenKind::BlockComment { is_terminated } => {
                    if !is_terminated {
                        err = "Missing trailing `*/` symbols to terminate the block comment";
                    }
                    COMMENT
                }

                tokenizer::TokenKind::Whitespace => WHITESPACE,

                tokenizer::TokenKind::Ident if token_text == "_" => UNDERSCORE,
                tokenizer::TokenKind::Ident => {
                    SyntaxKind::from_keyword(token_text).unwrap_or(IDENT)
                }
                tokenizer::TokenKind::InvalidIdent => {
                    err = "Ident contains invalid characters";
                    IDENT
                }

                tokenizer::TokenKind::Literal { kind, .. } => {
                    self.extend_literal(token_text.len(), kind);
                    return;
                }

                tokenizer::TokenKind::Semi => T![;],
                tokenizer::TokenKind::Comma => T![,],
                tokenizer::TokenKind::Dot => T![.],
                tokenizer::TokenKind::DotDot => T![..],
                tokenizer::TokenKind::Dotro => T![::],
                tokenizer::TokenKind::OpenParen => T!['('],
                tokenizer::TokenKind::CloseParen => T![')'],
                tokenizer::TokenKind::OpenBrace => T!['{'],
                tokenizer::TokenKind::CloseBrace => T!['}'],
                tokenizer::TokenKind::OpenBracket => T!['['],
                tokenizer::TokenKind::CloseBracket => T![']'],
                tokenizer::TokenKind::At => T![@],
                tokenizer::TokenKind::Pound => T![#],
                tokenizer::TokenKind::Tilde => T![~],
                tokenizer::TokenKind::Question => T![?],
                tokenizer::TokenKind::Colon => T![:],
                tokenizer::TokenKind::Dollar => T![$],
                tokenizer::TokenKind::Eq => T![=],
                tokenizer::TokenKind::EqEq => T![==],
                tokenizer::TokenKind::EqEqEq => T![===],
                tokenizer::TokenKind::NEq => T![!=],
                tokenizer::TokenKind::NEqEq => T![!==],
                tokenizer::TokenKind::Bang => T![!],
                tokenizer::TokenKind::Lt => T![<],
                tokenizer::TokenKind::Gt => T![>],
                tokenizer::TokenKind::Minus => T![-],
                tokenizer::TokenKind::Amp => T![&],
                tokenizer::TokenKind::Pipe => T![|],
                tokenizer::TokenKind::Plus => T![+],
                tokenizer::TokenKind::SectionSeparator => T![---],
                tokenizer::TokenKind::Star => T![*],
                tokenizer::TokenKind::Slash => T![/],
                tokenizer::TokenKind::Percent => T![%],
                tokenizer::TokenKind::Unknown => ERROR,
                tokenizer::TokenKind::UnknownPrefix if token_text == "builtin" => IDENT,
                tokenizer::TokenKind::UnknownPrefix => {
                    err = "unknown literal prefix";
                    IDENT
                }
                tokenizer::TokenKind::End => EOF,
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, token_text.len(), err);
    }

    pub(crate) fn extend_literal(&mut self, len: usize, kind: &tokenizer::LiteralKind) {
        let mut err = "";

        let syntax_kind = match *kind {
            tokenizer::LiteralKind::Int { is_empty, base: _ } => {
                if is_empty {
                    err = "Missing digits after the integer base prefix";
                }
                INT_NUMBER
            }
            tokenizer::LiteralKind::Float { is_empty, base: _ } => {
                if is_empty {
                    err = "Missing digits after the exponent symbol";
                }
                FLOAT_NUMBER
            }
            tokenizer::LiteralKind::Char { is_terminated } => {
                if !is_terminated {
                    err = "Missing trailing `'` symbol to terminate the character literal";
                } else {
                    let text = &self.res.text[self.offset + 1..][..len - 1];
                    let i = text.rfind('\'').unwrap();
                    let text = &text[..i];
                    if let Err(e) = tokenizer::unescape::unescape_char(text) {
                        err = error_to_diagnostic_message(e, Mode::Char);
                    }
                }
                CHAR
            }
            tokenizer::LiteralKind::Str { ending } => {
                match ending {
                    StringLiteralEnding::TerminatedString { is_multiline: _ } => {
                        let text = &self.res.text[self.offset + 1..][..len - 1];
                        let i = text.rfind('"').unwrap();
                        let text = &text[..i];
                        err = unescape_string_error_message(text, Mode::Str);
                    }
                    StringLiteralEnding::UnterminatedString => {
                        err = "Missing trailing `\"` symbol to terminate the string literal";
                    }
                }
                STRING
            }
            tokenizer::LiteralKind::RawStr { pound_count } => {
                if pound_count.is_none() {
                    err = "Invalid raw string literal";
                }
                STRING
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, len, err);
    }
}
