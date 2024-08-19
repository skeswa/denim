use crate::{cursor::Cursor, token::Token, token_kind::TokenKind};

/// Creates a new [Iterator] that exhaustively produces each Denim [Token] that
/// comprises `input` in order from top of the file to the bottom of the file.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);

    std::iter::from_fn(move || {
        let token = cursor.tokenize_next();

        if token.kind != TokenKind::End {
            Some(token)
        } else {
            None
        }
    })
}
