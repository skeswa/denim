#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AttrKind {
    Inner,
    Outer,
}

impl AttrKind {
    /// Returns `true` if the attr_kind is [`Inner`](Self::Inner).
    pub fn is_inner(&self) -> bool {
        matches!(self, Self::Inner)
    }

    /// Returns `true` if the attr_kind is [`Outer`](Self::Outer).
    pub fn is_outer(&self) -> bool {
        matches!(self, Self::Outer)
    }
}
