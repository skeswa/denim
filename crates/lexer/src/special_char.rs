use unicode_xid::UnicodeXID;

/// [char] that can be recognized as serving a specific purpose in the Denim
/// language.
pub(crate) trait SpecialChar {
    /// Returns `true` if this [char] is valid as a non-first character of an
    /// identifier.
    ///
    /// Denim uses Rust's language specification to define identifiers. For
    /// more, see the
    /// [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html).
    fn is_after_ident_start(self) -> bool;

    /// Returns `true` if this [char] is valid as a first character of an
    /// identifier.
    ///
    /// Denim uses Rust's language specification to define identifiers. For more,
    /// see the
    /// [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html).
    fn is_ident_start(self) -> bool;

    /// Returns `true` if this [char] is considered a whitespace according to
    /// the language spec.
    ///
    /// Denim uses Rust's language specification to define whitespace. For more,
    /// see the
    /// [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html).
    fn is_whitespace(self) -> bool;
}

impl SpecialChar for char {
    fn is_after_ident_start(self) -> bool {
        UnicodeXID::is_xid_continue(self)
    }

    fn is_ident_start(self) -> bool {
        // This is XID_Start OR '_' (which formally is not a XID_Start).
        self == '_' || UnicodeXID::is_xid_start(self)
    }

    fn is_whitespace(self) -> bool {
        // This is Pattern_White_Space.
        //
        // Note that this set is stable (ie, it doesn't change with different
        // Unicode versions), so it's ok to just hard-code the values.

        matches!(
            self,
            // Usual ASCII suspects
            '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

            // NEXT LINE from latin1
            | '\u{0085}'

            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
        )
    }
}
