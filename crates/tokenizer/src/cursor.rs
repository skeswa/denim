use std::str::Chars;

use crate::constants::EOF_CHAR;

/// Peekable iterator over a char sequence.
///
/// Next characters can be peeked via `first` method,
/// and position can be shifted forward via `bump` method.
///
/// For reference, see
/// [rustc-lexer's `Cursor`](https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs),
pub struct Cursor<'a> {
    /// Iterator over chars.
    ///
    /// Slightly faster than a `&str`.
    chars: Chars<'a>,
    // Bytes left in the `chars` Iterator.
    len_remaining: usize,
    /// Previously visited unicode code point.
    ///
    /// Notably, `prev` refers to the semantic idea of a character rather than
    /// a traditional ASCII 8-bit `char`.
    #[cfg(debug_assertions)]
    prev: char,
}

impl<'a> Cursor<'a> {
    /// Creates and returns a new `Cursor` that will iterate over the specified
    /// `input`.
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            len_remaining: input.len(),
            #[cfg(debug_assertions)]
            prev: EOF_CHAR,
        }
    }

    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }

        Some(c)
    }

    /// Eats symbols while predicate returns true or until the end of file is
    /// reached.
    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        // It was tried making optimized version of this for eg. line comments, but
        // LLVM can inline all of this and compile it down to fast iteration over bytes.
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    /// Peeks the next symbol from the input stream without consuming it.
    ///
    /// If requested position doesn't exist, `EOF_CHAR` is returned. However,
    /// getting `EOF_CHAR` doesn't always mean actual end of file, it should be
    /// checked with `is_eof` method.
    pub(crate) fn first(&self) -> char {
        // `.next()` optimizes better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Returns `true` if there is nothing more to consume.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Returns amount of already consumed symbols.
    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    /// Returns the last eaten symbol (or `'\0'` in release builds).
    ///
    /// (For debug assertions only.)
    pub(crate) fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }

        #[cfg(not(debug_assertions))]
        {
            EOF_CHAR
        }
    }

    /// Resets the number of bytes consumed to 0.
    pub(crate) fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Peeks the second symbol from the input stream without consuming it.
    pub(crate) fn second(&self) -> char {
        // `.next()` optimizes better than `.nth(1)`
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }
}
