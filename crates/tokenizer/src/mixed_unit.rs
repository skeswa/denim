/// Used for mixed utf8 string literals, i.e. those that allow both unicode
/// chars and high bytes.
pub enum MixedUnit {
    /// Used for ASCII chars (written directly or via `\x00`..`\x7f` escapes)
    /// and Unicode chars (written directly or via `\u` escapes).
    ///
    /// For example, if '¥' appears in a string it is represented here as
    /// `MixedUnit::Char('¥')`, and it will be appended to the relevant byte
    /// string as the two-byte UTF-8 sequence `[0xc2, 0xa5]`
    Char(char),

    /// Used for high bytes (`\x80`..`\xff`).
    ///
    /// For example, if `\xa5` appears in a string it is represented here as
    /// `MixedUnit::HighByte(0xa5)`, and it will be appended to the relevant
    /// byte string as the single byte `0xa5`.
    HighByte(u8),
}

impl From<char> for MixedUnit {
    fn from(c: char) -> Self {
        MixedUnit::Char(c)
    }
}

impl From<u8> for MixedUnit {
    fn from(n: u8) -> Self {
        if n.is_ascii() {
            MixedUnit::Char(n as char)
        } else {
            MixedUnit::HighByte(n)
        }
    }
}
