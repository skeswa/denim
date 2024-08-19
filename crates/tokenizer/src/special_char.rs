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
}

impl SpecialChar for char {
    fn is_after_ident_start(self) -> bool {
        UnicodeXID::is_xid_continue(self)
    }

    fn is_ident_start(self) -> bool {
        // This is XID_Start OR '_' (which formally is not a XID_Start).
        self == '_' || UnicodeXID::is_xid_start(self)
    }
}
