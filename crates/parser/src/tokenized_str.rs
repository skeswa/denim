//! Tokenizing `&str` into a sequence of Denim tokens.
//!
//! Note that strictly speaking the parser in this crate is not required to work
//! on tokens which originated from text. Macros, eg, can synthesize tokens out
//! of thin air. So, ideally, tokenizer should be an orthogonal crate. It is
//! however convenient to include a text-based tokenizer here!
//!
//! Note that these tokens, unlike the tokens we feed into the parser, do
//! include info about comments and whitespace.

use std::ops;

use crate::{token_converter::TokenConverter, tokenizer_error::TokenizerError, SyntaxKind};

pub struct TokenizedStr<'a> {
    pub(crate) error: Vec<TokenizerError>,
    pub(crate) kind: Vec<SyntaxKind>,
    pub(crate) start: Vec<u32>,
    pub(crate) text: &'a str,
}

impl<'a> TokenizedStr<'a> {
    pub fn new(text: &'a str) -> TokenizedStr<'a> {
        let _p = tracing::info_span!("TokenizedStr::new").entered();
        let mut conv = TokenConverter::new(text);

        for token in tokenizer::tokenize(&text[conv.offset..]) {
            let token_text = &text[conv.offset..][..token.len as usize];

            conv.extend_token(&token.kind, token_text);
        }

        conv.finalize_with_eof()
    }

    pub fn single_token(text: &'a str) -> Option<(SyntaxKind, Option<String>)> {
        if text.is_empty() {
            return None;
        }

        let token = tokenizer::tokenize(text).next()?;
        if token.len as usize != text.len() {
            return None;
        }

        let mut conv = TokenConverter::new(text);
        conv.extend_token(&token.kind, text);
        match &*conv.res.kind {
            [kind] => Some((*kind, conv.res.error.pop().map(|it| it.msg))),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
    }
    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        r.end - r.start
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self.error.binary_search_by_key(&(i as u32), |i| i.token).ok()?;
        Some(self.error[err].msg.as_str())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token as usize, it.msg.as_str()))
    }

    pub(crate) fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}
