use Mode::*;

/// What kind of literal do we parse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Char,

    Byte,

    Str,
    RawStr,

    ByteStr,
    RawByteStr,

    CStr,
    RawCStr,
}

impl Mode {
    pub fn in_double_quotes(self) -> bool {
        match self {
            Str | RawStr | ByteStr | RawByteStr | CStr | RawCStr => true,
            Char | Byte => false,
        }
    }

    /// Are `\x80`..`\xff` allowed?
    pub fn allow_high_bytes(self) -> bool {
        match self {
            Char | Str => false,
            Byte | ByteStr | CStr => true,
            RawStr | RawByteStr | RawCStr => unreachable!(),
        }
    }

    /// Are unicode (non-ASCII) chars allowed?
    #[inline]
    pub fn allow_unicode_chars(self) -> bool {
        match self {
            Byte | ByteStr | RawByteStr => false,
            Char | Str | RawStr | CStr | RawCStr => true,
        }
    }

    /// Are unicode escapes (`\u`) allowed?
    pub fn allow_unicode_escapes(self) -> bool {
        match self {
            Byte | ByteStr => false,
            Char | Str | CStr => true,
            RawByteStr | RawStr | RawCStr => unreachable!(),
        }
    }

    pub fn prefix_noraw(self) -> &'static str {
        match self {
            Char | Str | RawStr => "",
            Byte | ByteStr | RawByteStr => "b",
            CStr | RawCStr => "c",
        }
    }
}
