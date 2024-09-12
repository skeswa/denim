use super::*;

pub(super) const PATH_FIRST: TokenSet = TokenSet::new(&[IDENT, T![self], T![Self], T![:], T![<]]);

pub(super) fn is_path_start(p: &Parser<'_>) -> bool {
    is_use_path_start(p) || p.at(T![<]) || p.at(T![Self])
}

pub(super) fn is_use_path_start(p: &Parser<'_>) -> bool {
    match p.current() {
        IDENT | T![self] => true,
        T![:] if p.at(T![::]) => true,
        _ => false,
    }
}

pub(super) fn use_path(p: &mut Parser<'_>) {
    path(p, Mode::Use);
}

pub(crate) fn type_path(p: &mut Parser<'_>) {
    path(p, Mode::Type);
}

pub(super) fn expr_path(p: &mut Parser<'_>) {
    path(p, Mode::Expr);
}

pub(crate) fn type_path_for_qualifier(
    p: &mut Parser<'_>,
    qual: CompletedMarker,
) -> CompletedMarker {
    path_for_qualifier(p, Mode::Type, qual)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Mode {
    Use,
    Type,
    Expr,
}

fn path(p: &mut Parser<'_>, mode: Mode) {
    let path = p.start();
    path_segment(p, mode, true);
    let qual = path.complete(p, PATH);
    path_for_qualifier(p, mode, qual);
}

fn path_for_qualifier(
    p: &mut Parser<'_>,
    mode: Mode,
    mut qual: CompletedMarker,
) -> CompletedMarker {
    loop {
        let use_tree = mode == Mode::Use && matches!(p.nth(2), T![*] | T!['{']);
        if p.at(T![::]) && !use_tree {
            let path = qual.precede(p);
            p.bump(T![::]);
            path_segment(p, mode, false);
            let path = path.complete(p, PATH);
            qual = path;
        } else {
            return qual;
        }
    }
}

const EXPR_PATH_SEGMENT_RECOVERY_SET: TokenSet =
    items::ITEM_RECOVERY_SET.union(TokenSet::new(&[T![')'], T![,], T![let]]));
const TYPE_PATH_SEGMENT_RECOVERY_SET: TokenSet = types::TYPE_RECOVERY_SET;

fn path_segment(p: &mut Parser<'_>, mode: Mode, first: bool) {
    let m = p.start();
    let empty = if first {
        p.eat(T![::]);
        false
    } else {
        true
    };

    match p.current() {
        IDENT => name_ref(p),

        // test self_path
        // use self::foo;
        T![self] | T![Self] => {
            let m = p.start();
            p.bump_any();
            m.complete(p, NAME_REF);
        }
        _ => {
            let recover_set = match mode {
                Mode::Use => items::ITEM_RECOVERY_SET,
                Mode::Type => TYPE_PATH_SEGMENT_RECOVERY_SET,
                Mode::Expr => EXPR_PATH_SEGMENT_RECOVERY_SET,
            };
            p.err_recover("expected identifier", recover_set);
            if empty {
                // test_err empty_segment
                // use crate::;
                m.abandon(p);
                return;
            }
        }
    };
    m.complete(p, PATH_SEGMENT);
}
