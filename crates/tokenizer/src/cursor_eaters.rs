use crate::{
    constants::EOF_CHAR,
    cursor::Cursor,
    raw_str_error::RawStrError,
    string_literal_ending::StringLiteralEnding::{self, *},
};

impl<'a> Cursor<'a> {
    /// Eats unicode codepoints until they no longer fit the definition of a
    /// decimal number, returning `true` if the eaten decimal included any
    /// digits (e.g. `_` does not include digits).
    ///
    /// NOTE: succeeds on `_`, which isn't a valid number.
    pub(crate) fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;

                    self.bump();
                }
                _ => break,
            }
        }

        has_digits
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// double-quoted string, returning a [DoubleQuotedStringEnding] that
    /// details how the string literal ends, if at all.
    pub(crate) fn eat_double_quoted_string(&mut self) -> StringLiteralEnding {
        debug_assert!(self.prev() == '"');

        // Look for a multi-line string.
        if self.first() == '"' && self.second() == '"' {
            self.bump();
            self.bump();

            return self.eat_multiline_double_quoted_string();
        }

        while let Some(c) = self.bump() {
            match c {
                '"' => {
                    return TerminatedString {
                        is_multiline: false,
                    };
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }

        // End of file reached.
        UnterminatedString
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// float exponent, returning `true` if the eaten float exponent included
    /// any digits (e.g. `_` does not include digits).
    pub(crate) fn eat_float_exponent(&mut self) -> bool {
        debug_assert!(self.prev() == 'e' || self.prev() == 'E');

        if self.first() == '-' || self.first() == '+' {
            self.bump();
        }

        self.eat_decimal_digits()
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// hexadecimal number, returning `true` if the eaten hexadecimal included
    /// any digits (e.g. `_` does not include digits).
    ///
    /// NOTE: succeeds on `_`, which isn't a valid number.
    pub(crate) fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    has_digits = true;

                    self.bump();
                }
                _ => break,
            }
        }

        has_digits
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// multi-line string, returning a [DoubleQuotedStringEnding] that details
    /// how the string literal ends, if at all.
    pub(crate) fn eat_multiline_double_quoted_string(&mut self) -> StringLiteralEnding {
        debug_assert!(self.prev() == '"');

        while let Some(c) = self.bump() {
            match c {
                // Check to see if this is the `"""` end of a multi-line
                // string.
                '"' if self.first() == '"' && self.second() == '"' => {
                    // Eat the multi-line string ending.
                    self.bump();
                    self.bump();

                    return TerminatedString { is_multiline: true };
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }

        // End of file reached.
        UnterminatedString
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// double-quoted raw string, returning the number of pound characters
    /// following the initial `r` if and only if the raw string is correctly defined.
    ///
    /// * `prefix_len` is the number of unicode codepoints that account for the
    ///    beginning of the string literal (e.g. a string that begins `r##"` has
    ///    a `prefix_len` of `4`).
    pub(crate) fn eat_raw_double_quoted_string(
        &mut self,
        prefix_len: u32,
    ) -> Result<u8, RawStrError> {
        // Wrap the actual function to handle the error with too many pound
        // characters. This way, it eats the whole raw string.
        let pound_count = self.eat_raw_double_quoted_string_unvalidated(prefix_len)?;

        // Only up to 255 `#`s are allowed in raw strings
        match u8::try_from(pound_count) {
            Ok(num) => Ok(num),
            Err(_) => Err(RawStrError::TooManyDelimiters { found: pound_count }),
        }
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// single-quoted string, returning `true` if the single-quoted string
    /// terminates as expected.
    pub(crate) fn eat_single_quoted_string(&mut self) -> bool {
        debug_assert!(self.prev() == '\'');

        // Check if it's a one-symbol literal.
        if self.second() == '\'' && self.first() != '\\' {
            self.bump();
            self.bump();

            return true;
        }

        // Literal has more than one symbol.

        // Parse until either quotes are terminated or error is detected.
        loop {
            match self.first() {
                // Quotes are terminated, finish parsing.
                '\'' => {
                    self.bump();

                    return true;
                }
                // Probably beginning of the comment, which we don't want to include
                // to the error report.
                '/' => break,
                // Newline without following '\'' means unclosed quote, stop parsing.
                '\n' if self.second() != '\'' => break,
                // End of file, stop parsing.
                EOF_CHAR if self.is_eof() => break,
                // Escaped slash is considered one character, so bump twice.
                '\\' => {
                    self.bump();
                    self.bump();
                }
                // Skip the character.
                _ => {
                    self.bump();
                }
            }
        }

        // Character was not terminated.
        false
    }

    /// Eats unicode codepoints until they no longer fit the definition of a
    /// double-quoted raw string, returning the number of pound characters
    /// following the initial `r`.
    ///
    /// * `prefix_len` is the number of unicode codepoints that account for the
    ///    beginning of the string literal (e.g. a string that begins `r##"` has
    ///    a `prefix_len` of `4`).
    fn eat_raw_double_quoted_string_unvalidated(
        &mut self,
        prefix_len: u32,
    ) -> Result<u32, RawStrError> {
        debug_assert!(self.prev() == 'r');

        let start_pos = self.pos_within_token();
        let mut possible_terminator_offset = None;
        let mut max_pound_count = 0;

        // Count opening '#' symbols.
        let mut eaten_pound_count = 0;
        while self.first() == '#' {
            eaten_pound_count += 1;
            self.bump();
        }
        let opening_pound_count = eaten_pound_count;

        // Check that string is started.
        match self.bump() {
            Some('"') => (),
            c => {
                let c = c.unwrap_or(EOF_CHAR);

                return Err(RawStrError::InvalidStarter { bad_char: c });
            }
        }

        // Skip the string contents and on each '#' character met, check if this is
        // a raw string termination.
        loop {
            self.eat_while(|c| c != '"');

            if self.is_eof() {
                return Err(RawStrError::NoTerminator {
                    expected: opening_pound_count,
                    found: max_pound_count,
                    possible_terminator_offset,
                });
            }

            // Eat closing double quote.
            self.bump();

            // Check that amount of closing '#' symbols is equal to the amount
            // of opening ones.
            //
            // NOTE: this will not consume extra trailing `#` characters:
            // `r###"abcde"####` is lexed as a `RawStr { pound_count: 3 }`
            // followed by a `#` token.
            let mut closing_pound_count = 0;
            while self.first() == '#' && closing_pound_count < opening_pound_count {
                closing_pound_count += 1;
                self.bump();
            }

            if closing_pound_count == opening_pound_count {
                return Ok(opening_pound_count);
            } else if closing_pound_count > max_pound_count {
                // Keep track of possible terminators to give a hint about
                // where there might be a missing terminator
                possible_terminator_offset =
                    Some(self.pos_within_token() - start_pos - closing_pound_count + prefix_len);
                max_pound_count = closing_pound_count;
            }
        }
    }
}
