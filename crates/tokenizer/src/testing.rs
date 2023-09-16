#[cfg(test)]
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

#[cfg(test)]
use crate::{token::Token, tokenize::tokenize};

/// A single test case that compares the "correct" tokenization of Denim
/// source code against its "actual" tokenization.
#[cfg(test)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GoldenTestCase {
    pub(crate) source_path: PathBuf,
    pub(crate) tokens_path: PathBuf,
    source: String,
}

#[cfg(test)]
impl GoldenTestCase {
    /// Returns the stringified form of the sequence of tokens represented
    /// by `self.source`.
    pub(crate) fn stringify_tokenized_source(self) -> String {
        let tokens = tokenize(&self.source).collect::<Vec<Token>>();

        let mut i = 0usize;
        let mut token_summaries = Vec::new();

        for token in tokens {
            let start_index: usize = i;
            let end_index: usize = i + usize::try_from(token.len).unwrap();

            let token_substring = &self
                .source
                .get(start_index..end_index)
                .unwrap_or("")
                .replace("\n", "\\n")
                .replace("\t", "\\t");

            let padded_token_len = format!("{: <3}", token.len.to_string());
            let quoted_token_substring = if token_substring.chars().count() > 80 - 2 {
                format!(
                    "\"{}…",
                    token_substring.chars().take(80 - 2).collect::<String>()
                )
            } else {
                format!("\"{}\"", token_substring)
            };

            let serialized_token = format!(
                "{: <80} | {} | {:?}",
                quoted_token_substring, padded_token_len, token.kind
            );

            token_summaries.push(serialized_token);

            i = end_index;
        }

        token_summaries.join("\n")
    }
}

/// Collection of related [GoldenTestCase] instances.
#[cfg(test)]
pub(crate) struct GoldenTestCases(Vec<GoldenTestCase>);

#[cfg(test)]
impl GoldenTestCases {
    /// Reads all of the golden test cases defined within
    /// `goldens/{dir_name}`.
    pub(crate) fn in_dir(dir_name: &'static str) -> GoldenTestCases {
        let crate_root_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));

        let goldens_dir_path = crate_root_dir_path.join("goldens");

        let dir_path = goldens_dir_path.join(dir_name);

        let mut golden_test_cases = Vec::new();

        let file_results = read_dir(&dir_path)
            .unwrap_or_else(|err| panic!("can't `read_dir` {}: {err}", dir_path.display()));

        for file_result in file_results {
            let file = file_result.unwrap();

            let file_path = file.path();

            // Skip non-source files.
            if file_path.extension().unwrap_or_default() != "👖" {
                continue;
            }

            let source_path = file_path.clone();
            let tokens_path = file_path.clone().with_extension("👖.tokens");

            let source = read_to_string(&source_path).unwrap();

            golden_test_cases.push(GoldenTestCase {
                source,
                source_path,
                tokens_path,
            });
        }

        golden_test_cases.sort();

        GoldenTestCases(golden_test_cases)
    }

    /// Creates a consuming iterator, that is, one that moves each value out
    /// of the vector (from start to end). [GoldenTestCases] cannot be used
    /// after calling this.
    pub(crate) fn into_iter(self) -> impl Iterator<Item = GoldenTestCase> {
        self.0.into_iter()
    }
}
