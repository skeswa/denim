use super::*;

#[derive(PartialEq, Eq)]
pub(super) enum Semicolon {
    Required,
    Optional,
    Forbidden,
}

pub(super) fn expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    todo!()
}
