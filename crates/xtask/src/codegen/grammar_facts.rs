use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct GrammarFacts {
    pub ast_token_names: Vec<String>,
    pub contextual_keywords: Vec<String>,
    pub manually_implemented_rule: Vec<String>,
    pub nodes_with_doc_comments: Vec<String>,
    #[serde(alias = "punctuation-names")]
    pub punctuation_names: HashMap<String, String>,
    pub reserved_words: Vec<String>,
}

impl GrammarFacts {
    pub fn read(project_root: &PathBuf) -> GrammarFacts {
        let grammar_facts_file_path = project_root.join("crates/syntax/denim_grammar_facts.toml");

        let grammar_facts_file_contents = read_to_string(grammar_facts_file_path).unwrap();

        toml::from_str(&grammar_facts_file_contents).unwrap()
    }
}
