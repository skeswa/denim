use std::{fs::read_to_string, path::PathBuf};

use ungrammar::Grammar;

use crate::{
    codegen::{
        ast_tokens_src::AstTokensSrc, file_util::ensure_file_contents, lower::lower_grammar,
        syntax_kinds_src::SyntaxKindsSrc,
    },
    CodegenCommands,
};

use super::grammar_facts::GrammarFacts;

pub fn codegen_grammar_files(command: &CodegenCommands, project_root: PathBuf, check: bool) {
    let grammar = read_to_string(project_root.join("crates/syntax/references/rust.ungram"))
        .unwrap()
        .parse::<Grammar>()
        .unwrap();
    let grammar_facts = GrammarFacts::read(&project_root);

    let ast = lower_grammar(&grammar, &grammar_facts);

    let syntax_kinds_src =
        SyntaxKindsSrc::generate(&ast.nodes, &ast.enums, &grammar, &grammar_facts);

    let syntax_kinds_file_content = syntax_kinds_src.print(command);
    let syntax_kinds_file_path = project_root.join("crates/parser/src/syntax_kind/generated.rs");

    ensure_file_contents(
        command,
        &project_root,
        syntax_kinds_file_path.as_path(),
        &syntax_kinds_file_content,
        check,
    );

    let ast_tokens_src = AstTokensSrc::generate(&ast);

    let ast_tokens_file_content = ast_tokens_src.print(command);
    let ast_tokens_file_path = project_root.join("crates/syntax/src/ast/generated/tokens.rs");

    ensure_file_contents(
        command,
        &project_root,
        ast_tokens_file_path.as_path(),
        &ast_tokens_file_content,
        check,
    );
}
