use crate::CodegenCommands;

use super::{
    ast_src::{AstEnumSrc, AstNodeSrc},
    codegen_util::{add_preamble, reformat},
    english_util::to_upper_snake_case,
    grammar_facts::GrammarFacts,
};
use proc_macro2::{Punct, Spacing};
use quote::{format_ident, quote};

#[derive(Copy, Clone, Debug)]
pub struct SyntaxKindsSrc {
    pub punct: &'static [(String, String)],
    pub keywords: &'static [String],
    pub literals: &'static [&'static str],
    pub tokens: &'static [&'static str],
    pub nodes: &'static [&'static str],
}

impl SyntaxKindsSrc {
    pub fn generate(
        nodes: &[AstNodeSrc],
        enums: &[AstEnumSrc],
        grammar: &ungrammar::Grammar,
        grammar_facts: &GrammarFacts,
    ) -> SyntaxKindsSrc {
        let mut keywords: Vec<String> = Vec::new();
        let mut tokens: Vec<&_> = TOKENS.to_vec();
        let mut literals: Vec<&_> = Vec::new();
        let puncts: Vec<_> = grammar_facts
            .punctuation_names
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();

        let mut used_puncts = vec![false; puncts.len()];

        // We need to mark `$` as used to avoid checks below. We want it to be
        // registered punctuation even though it might not be a part of the
        // grammar.
        let dollar_index = puncts.iter().position(|(k, _)| k == "$").unwrap();

        used_puncts[dollar_index] = true;

        let contextual_keywords: Vec<&str> = grammar_facts
            .contextual_keywords
            .iter()
            .map(String::as_str)
            .collect();

        grammar.tokens().for_each(|token| {
            let name = &*grammar[token].name;
            if name == EOF {
                return;
            }

            match name.split_at(1) {
                ("@", lit) if !lit.is_empty() => {
                    literals.push(String::leak(to_upper_snake_case(lit)));
                }
                ("#", token) if !token.is_empty() => {
                    tokens.push(String::leak(to_upper_snake_case(token)));
                }
                _ if contextual_keywords.contains(&name) => {}
                _ if name.chars().all(char::is_alphabetic) => {
                    keywords.push(name.to_owned());
                }
                _ => {
                    let idx = puncts
                        .iter()
                        .position(|(punct, _)| punct == &name)
                        .unwrap_or_else(|| {
                            panic!("Grammar references unknown punctuation {name:?}")
                        });
                    used_puncts[idx] = true;
                }
            }
        });

        puncts
            .iter()
            .zip(used_puncts)
            .filter(|(_, used)| !used)
            .for_each(|((punct, _), _)| {
                panic!("Punctuation {punct:?} is not used in grammar");
            });
        keywords.extend(
            grammar_facts
                .reserved_words
                .iter()
                .map(|reserved_word| reserved_word.to_owned()),
        );
        keywords.sort();
        keywords.dedup();

        // we leak things here for simplicity, that way we don't have to deal with lifetimes
        // The execution is a one shot job so thats fine
        let nodes = nodes
            .iter()
            .map(|it| &it.name)
            .chain(enums.iter().map(|it| &it.name))
            .map(|it| to_upper_snake_case(it))
            .map(String::leak)
            .map(|it| &*it)
            .collect();
        let nodes = Vec::leak(nodes);
        nodes.sort();
        let keywords = Vec::leak(keywords);
        let literals = Vec::leak(literals);
        literals.sort();
        let tokens = Vec::leak(tokens);
        tokens.sort();

        SyntaxKindsSrc {
            punct: Vec::leak(puncts),
            nodes,
            keywords,
            literals,
            tokens,
        }
    }

    pub fn print(self, command: &CodegenCommands) -> String {
        let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = self
            .punct
            .iter()
            .filter(|(token, _name)| token.len() == 1)
            .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
            .unzip();

        let punctuation_values = self.punct.iter().map(|(token, _name)| {
            if "{}[]()".contains(token) {
                let c = token.chars().next().unwrap();
                quote! { #c }
            } else {
                let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
                quote! { #(#cs)* }
            }
        });
        let punctuation = self
            .punct
            .iter()
            .map(|(_token, name)| format_ident!("{}", name))
            .collect::<Vec<_>>();

        let fmt_kw_as_variant = |name| match name {
            "Self" => format_ident!("SELF_TYPE_KW"),
            name => format_ident!("{}_KW", to_upper_snake_case(name)),
        };
        let strict_keywords = self.keywords;
        let strict_keywords_variants = strict_keywords
            .iter()
            .map(|s| s.as_str())
            .map(fmt_kw_as_variant)
            .collect::<Vec<_>>();
        let strict_keywords_tokens = strict_keywords.iter().map(|it| format_ident!("{it}"));

        let literals = self
            .literals
            .iter()
            .map(|name| format_ident!("{}", name))
            .collect::<Vec<_>>();

        let tokens = self
            .tokens
            .iter()
            .map(|name| format_ident!("{}", name))
            .collect::<Vec<_>>();

        let nodes = self
            .nodes
            .iter()
            .map(|name| format_ident!("{}", name))
            .collect::<Vec<_>>();

        let ast = quote! {
            #![allow(bad_style, dead_code, missing_docs, unreachable_pub)]

            /// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`.
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            #[repr(u16)]
            pub enum SyntaxKind {
                // Technical SyntaxKinds: they appear temporally during parsing,
                // but never end up in the final tree
                #[doc(hidden)]
                TOMBSTONE,
                #[doc(hidden)]
                EOF,
                #(#punctuation,)*
                #(#strict_keywords_variants,)*
                #(#literals,)*
                #(#tokens,)*
                #(#nodes,)*

                // Technical kind so that we can cast from u16 safely
                #[doc(hidden)]
                __LAST,
            }
            use self::SyntaxKind::*;

            impl SyntaxKind {
                /// Checks whether this syntax kind is a strict keyword for the given edition.
                /// Strict keywords are identifiers that are always considered keywords.
                pub fn is_strict_keyword(self) -> bool {
                    matches!(self, #(#strict_keywords_variants)|*)
                }

                /// Checks whether this syntax kind is a strict or weak keyword for the given edition.
                pub fn is_keyword(self) -> bool {
                    matches!(self, #(#strict_keywords_variants)|*)
                }

                pub fn is_punct(self) -> bool {
                    matches!(self, #(#punctuation)|*)
                }

                pub fn is_literal(self) -> bool {
                    matches!(self, #(#literals)|*)
                }

                pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                    let kw = match ident {
                        #(#strict_keywords => #strict_keywords_variants,)*
                        _ => return None,
                    };
                    Some(kw)
                }

                pub fn from_char(c: char) -> Option<SyntaxKind> {
                    let tok = match c {
                        #(#single_byte_tokens_values => #single_byte_tokens,)*
                        _ => return None,
                    };
                    Some(tok)
                }
            }

            #[macro_export]
            macro_rules! T {
                #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
                #([#strict_keywords_tokens] => { $crate::SyntaxKind::#strict_keywords_variants };)*
                [lifetime_ident] => { $crate::SyntaxKind::LIFETIME_IDENT };
                [int_number] => { $crate::SyntaxKind::INT_NUMBER };
                [ident] => { $crate::SyntaxKind::IDENT };
                [string] => { $crate::SyntaxKind::STRING };
                [shebang] => { $crate::SyntaxKind::SHEBANG };
            }
        };

        add_preamble(command, reformat(ast.to_string()))
    }
}

const EOF: &str = "EOF";
const TOKENS: &[&str] = &["ERROR", "WHITESPACE", "NEWLINE", "COMMENT"];
