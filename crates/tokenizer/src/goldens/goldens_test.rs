use crate::goldens::golden_test_cases::GoldenTestCases;
use expect_test::expect_file;

#[test]
fn err_goldens() {
    for golden_test_case in GoldenTestCases::in_dir("err") {
        println!("Golden test: {}", golden_test_case.source_path.to_str().unwrap_or_default());

        expect_file![golden_test_case.tokens_path.clone()]
            .assert_eq(&golden_test_case.stringify_tokenized_source());
    }
}

#[test]
fn ok_goldens() {
    for golden_test_case in GoldenTestCases::in_dir("ok") {
        println!("Golden test: {}", golden_test_case.source_path.to_str().unwrap_or_default());

        expect_file![golden_test_case.tokens_path.clone()]
            .assert_eq(&golden_test_case.stringify_tokenized_source());
    }
}
