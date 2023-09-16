mod constants;
mod cursor;
mod cursor_eaters;
mod cursor_tokenizers;
mod literal_kind;
mod numeric_base;
mod raw_str_error;
mod special_char;
mod testing;
mod token;
mod token_kind;
pub mod tokenize;

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
