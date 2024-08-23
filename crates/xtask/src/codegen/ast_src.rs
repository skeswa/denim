use std::collections::BTreeSet;

use quote::{format_ident, quote};

use super::{english_util::to_lower_snake_case, grammar_facts::GrammarFacts};

#[derive(Default, Debug)]
pub struct AstSrc {
    pub tokens: Vec<String>,
    pub nodes: Vec<AstNodeSrc>,
    pub enums: Vec<AstEnumSrc>,
}

impl AstSrc {
    pub fn deduplicate_fields(&mut self) {
        for node in &mut self.nodes {
            let mut i = 0;
            'outer: while i < node.fields.len() {
                for j in 0..i {
                    let f1 = &node.fields[i];
                    let f2 = &node.fields[j];
                    if f1 == f2 {
                        node.fields.remove(i);
                        continue 'outer;
                    }
                }
                i += 1;
            }
        }
    }

    pub fn extract_enum_traits(&mut self) {
        for enm in &mut self.enums {
            if enm.name == "Stmt" {
                continue;
            }
            let nodes = &self.nodes;
            let mut variant_traits = enm
                .variants
                .iter()
                .map(|var| nodes.iter().find(|it| &it.name == var).unwrap())
                .map(|node| node.traits.iter().cloned().collect::<BTreeSet<_>>());

            let mut enum_traits = match variant_traits.next() {
                Some(it) => it,
                None => continue,
            };
            for traits in variant_traits {
                enum_traits = enum_traits.intersection(&traits).cloned().collect();
            }
            enm.traits = enum_traits.into_iter().collect();
        }
    }

    pub fn extract_enums(&mut self) {
        for node in &mut self.nodes {
            for enm in &self.enums {
                let mut to_remove = Vec::new();
                for (i, field) in node.fields.iter().enumerate() {
                    let ty = field.ty().to_string();
                    if enm.variants.iter().any(|it| it == &ty) {
                        to_remove.push(i);
                    }
                }
                if to_remove.len() == enm.variants.len() {
                    node.remove_field(to_remove);
                    let ty = enm.name.clone();
                    let name = to_lower_snake_case(&ty);
                    node.fields.push(Field::Node {
                        name,
                        ty,
                        cardinality: Cardinality::Optional,
                    });
                }
            }
        }
    }

    pub fn extract_struct_traits(&mut self, grammar_facts: &GrammarFacts) {
        for node in &mut self.nodes {
            for (name, methods) in TRAITS {
                node.extract_struct_trait(name, methods);
            }
        }

        for node in &mut self.nodes {
            if grammar_facts.nodes_with_doc_comments.contains(&node.name) {
                node.traits.push("HasDocComments".into());
            }
        }
    }
}

#[derive(Debug)]
pub struct AstNodeSrc {
    pub doc: Vec<String>,
    pub name: String,
    pub traits: Vec<String>,
    pub fields: Vec<Field>,
}

impl AstNodeSrc {
    fn extract_struct_trait(&mut self, trait_name: &str, methods: &[&str]) {
        let mut to_remove = Vec::new();
        for (i, field) in self.fields.iter().enumerate() {
            let method_name = field.method_name();
            if methods.iter().any(|&it| it == method_name) {
                to_remove.push(i);
            }
        }
        if to_remove.len() == methods.len() {
            self.traits.push(trait_name.to_owned());
            self.remove_field(to_remove);
        }
    }

    fn remove_field(&mut self, to_remove: Vec<usize>) {
        to_remove.into_iter().rev().for_each(|idx| {
            self.fields.remove(idx);
        });
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

impl Field {
    pub fn is_many(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::Many,
                ..
            }
        )
    }
    pub fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = token.parse().unwrap();
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }
    pub fn method_name(&self) -> String {
        match self {
            Field::Token(name) => {
                let name = match name.as_str() {
                    ";" => "semicolon",
                    "->" => "thin_arrow",
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    "<" => "l_angle",
                    ">" => "r_angle",
                    "=" => "eq",
                    "!" => "excl",
                    "*" => "star",
                    "&" => "amp",
                    "-" => "minus",
                    "_" => "underscore",
                    "." => "dot",
                    ".." => "dotdot",
                    "..." => "dotdotdot",
                    "..=" => "dotdoteq",
                    "=>" => "fat_arrow",
                    "@" => "at",
                    ":" => "colon",
                    "::" => "coloncolon",
                    "#" => "pound",
                    "?" => "question_mark",
                    "," => "comma",
                    "|" => "pipe",
                    "~" => "tilde",
                    _ => name,
                };
                format!("{name}_token",)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    String::from("ty")
                } else {
                    name.to_owned()
                }
            }
        }
    }
    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub struct AstEnumSrc {
    pub doc: Vec<String>,
    pub name: String,
    pub traits: Vec<String>,
    pub variants: Vec<String>,
}

const TRAITS: &[(&str, &[&str])] = &[
    ("HasAttrs", &["attrs"]),
    ("HasName", &["name"]),
    ("HasVisibility", &["visibility"]),
    ("HasGenericParams", &["generic_param_list", "where_clause"]),
    ("HasGenericArgs", &["generic_arg_list"]),
    ("HasTypeBounds", &["type_bound_list", "colon_token"]),
    ("HasModuleItem", &["items"]),
    ("HasLoopBody", &["label", "loop_body"]),
    ("HasArgList", &["arg_list"]),
];
