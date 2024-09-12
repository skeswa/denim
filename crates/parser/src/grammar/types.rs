use super::*;

pub(super) const TYPE_FIRST: TokenSet = paths::PATH_FIRST.union(TokenSet::new(&[
    T!['('],
    T!['['],
    T![<],
    T![!],
    T![*],
    T![&],
    T![_],
    T![fn],
    T![for],
    T![impl],
    T![Self],
]));

pub(super) const TYPE_RECOVERY_SET: TokenSet = TokenSet::new(&[
    T![')'],
    T![>],
    T![,],
    // test_err struct_field_recover
    // struct S { f pub g: () }
    T![pub],
]);

pub(crate) fn type_(p: &mut Parser<'_>) {
    todo!()
}
