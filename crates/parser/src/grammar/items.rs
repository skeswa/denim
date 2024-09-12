use super::*;

// test source_contents
// fn foo() {}
// type Bar = { b: i32 };
// impl Bar { fn bar(self) -> i32 { self.b } }
pub(super) fn source_contents(p: &mut Parser<'_>, stop_on_r_curly: bool) {
    attributes::inner_attrs(p);
    while !(p.at(EOF) || (p.at(T!['}']) && stop_on_r_curly)) {
        item(p, stop_on_r_curly);
    }
}

pub(super) const ITEM_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T![fn], T![enum], T![impl], T![trait], T![let], T![pub], T![use], T![;]]);

pub(super) fn item(p: &mut Parser<'_>, stop_on_r_curly: bool) {
    let m = p.start();
    attributes::outer_attrs(p);

    let m = match opt_item(p, m) {
        Ok(()) => {
            if p.at(T![;]) {
                p.err_and_bump(
                    "expected item, found `;`\n\
                         consider removing this semicolon",
                );
            }
            return;
        }
        Err(m) => m,
    };

    m.abandon(p);
    match p.current() {
        T!['{'] => error_block(p, "expected an item"),
        T!['}'] if !stop_on_r_curly => {
            let e = p.start();
            p.error("unmatched `}`");
            p.bump(T!['}']);
            e.complete(p, ERROR);
        }
        EOF | T!['}'] => p.error("expected an item"),
        T![let] => error_let_stmt(p, "expected an item"),
        _ => p.err_and_bump("expected an item"),
    }
}

/// Try to parse an item, completing `m` in case of success.
pub(super) fn opt_item(p: &mut Parser<'_>, m: Marker) -> Result<(), Marker> {
    todo!()
}

pub(crate) fn token_tree(p: &mut Parser<'_>) {
    let closing_paren_kind = match p.current() {
        T!['{'] => T!['}'],
        T!['('] => T![')'],
        T!['['] => T![']'],
        _ => unreachable!(),
    };
    let m = p.start();
    p.bump_any();
    while !p.at(EOF) && !p.at(closing_paren_kind) {
        match p.current() {
            T!['{'] | T!['('] | T!['['] => token_tree(p),
            T!['}'] => {
                p.error("unmatched `}`");
                m.complete(p, TOKEN_TREE);
                return;
            }
            T![')'] | T![']'] => p.err_and_bump("unmatched brace"),
            _ => p.bump_any(),
        }
    }
    p.expect(closing_paren_kind);
    m.complete(p, TOKEN_TREE);
}
