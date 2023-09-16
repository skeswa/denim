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

#[cfg(test)]
mod tests {
    use expect_test::expect_file;

    use crate::testing::GoldenTestCases;

    #[test]
    fn ok_goldens() {
        for golden_test_case in GoldenTestCases::in_dir("ok").into_iter() {
            expect_file![golden_test_case.tokens_path.clone()]
                .assert_eq(&golden_test_case.stringify_tokenized_source());
        }
    }
}
