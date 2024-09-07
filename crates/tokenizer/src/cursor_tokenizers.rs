use crate::{
    cursor::Cursor,
    literal_kind::LiteralKind::{self, *},
    numeric_base::NumericBase::*,
    special_char::SpecialChar,
    token::Token,
    token_kind::TokenKind,
    token_kind::TokenKind::*,
};
use unicode_properties::UnicodeEmoji;

impl<'a> Cursor<'a> {
    /// Parses and returns the next [Token] from this [Cursor]'s char sequence,
    /// advancing past it thereafter.
    ///
    /// When there are no more tokens left to parse, a [TokenKind::End] token is
    /// returned.
    pub(crate) fn tokenize_next(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token { kind: TokenKind::End, len: 0 },
        };

        let kind = match first_char {
            // Slash, comment or block comment.
            '/' => match self.first() {
                '/' => self.tokenize_line_comment(),
                '*' => self.tokenize_block_comment(),
                _ => Slash,
            },

            // Whitespace sequence.
            c if c.is_whitespace() => self.tokenize_whitespace(),

            // Raw identifier, raw string literal or identifier.
            'r' => match self.first() {
                '#' | '"' => {
                    let maybe_pound_count = self.eat_raw_double_quoted_string(1);

                    let kind = RawStr { pound_count: maybe_pound_count.ok() };

                    Literal { kind }
                }
                _ => self.tokenize_ident_or_unknown_prefix(),
            },

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if c.is_ident_start() => self.tokenize_ident_or_unknown_prefix(),

            // Numeric literal.
            c @ '0'..='9' => {
                let literal_kind = self.tokenize_numeric_literal(c);

                // TODO: add support for type suffixes e.g. `123i32`.

                Literal { kind: literal_kind }
            }

            // Either a section separator, `---`, or a normal minus, `-`.
            '-' => match (self.first(), self.second()) {
                ('-', '-') => {
                    // Advance past the remaining section separator characters.
                    self.bump();
                    self.bump();

                    SectionSeparator
                }
                _ => Minus,
            },

            // Either a dotro, `::`, or a normal colon, `:`.
            ':' => match self.first() {
                ':' => {
                    // Advance past the remaining dotro character.
                    self.bump();

                    Dotro
                }
                _ => Colon,
            },

            // Either a `..` or `.`.
            '.' => match self.first() {
                '.' => {
                    // Advance past the remaining dotdot character.
                    self.bump();

                    DotDot
                }
                _ => Dot,
            },

            // Either a `===`, `==`, or `=`.
            '=' => match (self.first(), self.second()) {
                ('=', '=') => {
                    // Advance past the remaining eqeqeq characters.
                    self.bump();
                    self.bump();

                    EqEqEq
                }
                ('=', _) => {
                    // Advance past the remaining eqeq characters.
                    self.bump();

                    EqEq
                }
                _ => Eq,
            },

            // Either a `!==`, `!=`, or `!`.
            '!' => match (self.first(), self.second()) {
                ('=', '=') => {
                    // Advance past the remaining neqeq characters.
                    self.bump();
                    self.bump();

                    NEqEq
                }
                ('=', _) => {
                    // Advance past the remaining neqeq characters.
                    self.bump();

                    NEq
                }
                _ => Bang,
            },

            // One-symbol tokens.
            ';' => Semi,
            ',' => Comma,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '?' => Question,
            '$' => Dollar,
            '<' => Lt,
            '>' => Gt,
            '&' => Amp,
            '|' => Pipe,
            '+' => Plus,
            '*' => Star,
            '%' => Percent,

            // Character literal.
            '\'' => self.tokenize_char_literal(),

            // String literal.
            '"' => {
                let ending = self.eat_double_quoted_string();

                Literal { kind: Str { ending } }
            }

            // Identifier starting with an emoji. Only lexed for graceful error recovery.
            c if !c.is_ascii() && c.is_emoji_char() => self.tokenize_fake_ident_or_unknown_prefix(),

            // 🤷🏾‍♂️.
            _ => Unknown,
        };

        let token = Token { kind, len: self.pos_within_token() };

        self.reset_pos_within_token();

        token
    }

    /// Attempts to recognize the next token a character literal, only advancing
    /// the cursor and returning it if this attempt is successful.
    fn tokenize_char_literal(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '\'');

        let is_terminated = self.eat_single_quoted_string();

        return Literal { kind: Char { is_terminated } };
    }

    /// Attempts to recognize the next token as either an unknown prefix or an
    /// invalid identifier, only advancing the cursor and returning it if this
    /// attempt is successful.
    fn tokenize_fake_ident_or_unknown_prefix(&mut self) -> TokenKind {
        // Start is already eaten, eat the rest of identifier.
        self.eat_while(|c| {
            unicode_xid::UnicodeXID::is_xid_continue(c)
                || (!c.is_ascii() && c.is_emoji_char())
                || c == '\u{200d}'
        });

        // Known prefixes must have been handled earlier. So if
        // we see a prefix here, it is definitely an unknown prefix.
        match self.first() {
            '#' | '"' | '\'' => UnknownPrefix,
            _ => InvalidIdent,
        }
    }

    /// Attempts to recognize the next token as an identifier, only advancing
    /// the cursor and returning it if this attempt is successful.
    fn tokenize_ident_or_unknown_prefix(&mut self) -> TokenKind {
        debug_assert!(self.prev().is_ident_start());

        // Front is already eaten, eat the rest of identifier.
        self.eat_while(|c| c.is_after_ident_start());

        // Known prefixes must have been handled earlier. So if
        // we see a prefix here, it is definitely an unknown prefix.
        match self.first() {
            '#' | '"' | '\'' => UnknownPrefix,
            c if !c.is_ascii() && c.is_emoji_char() => self.tokenize_fake_ident_or_unknown_prefix(),
            _ => Ident,
        }
    }

    /// Attempts to recognize the next token as a block comment, only advancing
    /// the cursor and returning it if this attempt is successful.
    fn tokenize_block_comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '/' && self.first() == '*');

        self.bump();

        let mut depth = 1usize;
        while let Some(c) = self.bump() {
            match c {
                '/' if self.first() == '*' => {
                    self.bump();

                    depth += 1;
                }
                '*' if self.first() == '/' => {
                    self.bump();

                    depth -= 1;

                    if depth == 0 {
                        // This block comment is closed, so for a construction like "/* */ */"
                        // there will be a successfully parsed block comment "/* */"
                        // and " */" will be processed separately.
                        break;
                    }
                }
                _ => (),
            }
        }

        BlockComment { is_terminated: depth == 0 }
    }

    /// Attempts to recognize the next token as a line comment, only advancing
    /// the cursor and returning it if this attempt is successful.
    fn tokenize_line_comment(&mut self) -> TokenKind {
        debug_assert!(self.prev() == '/' && self.first() == '/');

        self.bump();

        // `///` is a doc comment.
        // `////` is _not_ a doc comment.
        let is_doc_comment = self.first() == '/' && self.second() != '/';

        // Line comment is terminated by the end of the current line.
        self.eat_while(|c| c != '\n');

        LineComment { is_doc_comment }
    }

    /// Attempts to recognize the next token as a numeric literal, only
    /// advancing the cursor and returning it if this attempt is successful.
    ///
    /// * `first_digit` is the first character of the numeric literal
    fn tokenize_numeric_literal(&mut self, first_digit: char) -> LiteralKind {
        debug_assert!('0' <= self.prev() && self.prev() <= '9');

        let mut base = Decimal;
        if first_digit == '0' {
            // Attempt to parse encoding base.
            match self.first() {
                'b' => {
                    base = Binary;

                    self.bump();

                    let has_digits = self.eat_decimal_digits();
                    if !has_digits {
                        return Int { base, is_empty: true };
                    }
                }
                'o' => {
                    base = Octal;

                    self.bump();

                    let has_digits = self.eat_decimal_digits();
                    if !has_digits {
                        return Int { base, is_empty: true };
                    }
                }
                'x' => {
                    base = Hexadecimal;

                    self.bump();

                    let has_digits = self.eat_hexadecimal_digits();
                    if !has_digits {
                        return Int { base, is_empty: true };
                    }
                }
                // Not a base prefix; consume additional digits.
                '0'..='9' | '_' => {
                    self.eat_decimal_digits();
                }

                // Also not a base prefix; nothing more to do here.
                '.' | 'e' | 'E' => {}

                // Just a 0.
                _ => return Int { base, is_empty: true },
            }
        } else {
            // No base prefix, parse number in the usual way.
            self.eat_decimal_digits();
        };

        match self.first() {
            // Don't be greedy if this is actually an integer literal followed
            // by field/method access or a range pattern
            // (`0..2` and `12.foo()`).
            '.' if self.second() != '.' && !self.second().is_ident_start() => {
                // We might have stuff after the `.`, and if we do, it needs to
                // start with a number.
                self.bump();

                let mut has_empty_exponent = false;
                if self.first().is_digit(10) {
                    self.eat_decimal_digits();

                    match self.first() {
                        'e' | 'E' => {
                            self.bump();

                            has_empty_exponent = !self.eat_float_exponent();
                        }
                        _ => (),
                    }
                }

                Float { base, is_empty: has_empty_exponent }
            }
            'e' | 'E' => {
                self.bump();

                let has_empty_exponent = !self.eat_float_exponent();

                Float { base, is_empty: has_empty_exponent }
            }
            _ => Int { base, is_empty: false },
        }
    }

    /// Attempts to recognize the next token as whitespace, only advancing the
    /// cursor and returning it if this attempt is successful.
    fn tokenize_whitespace(&mut self) -> TokenKind {
        debug_assert!(self.prev().is_whitespace());

        self.eat_while(|c| c.is_whitespace());

        Whitespace
    }
}
