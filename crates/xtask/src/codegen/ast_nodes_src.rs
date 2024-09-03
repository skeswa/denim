use std::collections::HashSet;

use quote::{format_ident, quote};

use crate::{codegen::english_util::to_upper_snake_case, CodegenCommands};

use super::{
    ast_src::AstSrc, codegen_util::{add_preamble, reformat, write_doc_comment}, english_util::to_pascal_case, grammar_facts::GrammarFacts, syntax_kinds_src::SyntaxKindsSrc
};
use itertools::Itertools;

pub struct AstNodesSrc<'a, 'b>(&'a AstSrc, &'b SyntaxKindsSrc);

impl<'a, 'b> AstNodesSrc<'a, 'b> {
    pub fn generate(
        ast_src: &'a AstSrc,
        syntax_kinds_src: &'b SyntaxKindsSrc,
    ) -> AstNodesSrc<'a, 'b> {
        AstNodesSrc(ast_src, syntax_kinds_src)
    }

    pub fn print(self, command: &CodegenCommands, grammar_facts: &GrammarFacts) -> String {
        let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = self.0
                .nodes
                .iter()
                .map(|node| {
                    let name = format_ident!("{}", node.name);
                    let kind = format_ident!("{}", to_upper_snake_case(&node.name));
                    let traits = node
                        .traits
                        .iter()
                        .filter(|trait_name| {
                            // Loops have two expressions so this might collide, therefore manual impl it
                            node.name != "ForExpr" && node.name != "WhileExpr"
                                || trait_name.as_str() != "HasLoopBody"
                        })
                        .map(|trait_name| {
                            let trait_name = format_ident!("{}", trait_name);
                            quote!(impl ast::#trait_name for #name {})
                        });

                    let methods = node.fields.iter().map(|field| {
                        let method_name = format_ident!("{}", field.method_name(grammar_facts));
                        let ty = field.ty();
                        
                        if field.is_many() {
                            quote! {
                                #[inline]
                                pub fn #method_name(&self) -> AstChildren<#ty> {
                                    support::children(&self.syntax)
                                }
                            }
                        } else if let Some(token_kind) = field.token_kind() {
                            quote! {
                                #[inline]
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::token(&self.syntax, #token_kind)
                                }
                            }
                        } else {
                            quote! {
                                #[inline]
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::child(&self.syntax)
                                }
                            }
                        }
                    });
                    (
                        quote! {
                            #[pretty_doc_comment_placeholder_workaround]
                            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                            pub struct #name {
                                pub(crate) syntax: SyntaxNode,
                            }

                            #(#traits)*

                            impl #name {
                                #(#methods)*
                            }
                        },
                        quote! {
                            impl AstNode for #name {
                                #[inline]
                                fn can_cast(kind: SyntaxKind) -> bool {
                                    kind == #kind
                                }
                                #[inline]
                                fn cast(syntax: SyntaxNode) -> Option<Self> {
                                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                                }
                                #[inline]
                                fn syntax(&self) -> &SyntaxNode { &self.syntax }
                            }
                        },
                    )
                })
                .unzip();

        let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = self
            .0
            .enums
            .iter()
            .map(|en| {
                let variants: Vec<_> = en
                    .variants
                    .iter()
                    .map(|var| format_ident!("{}", var))
                    .sorted()
                    .collect();
                let name = format_ident!("{}", en.name);
                let kinds: Vec<_> = variants
                    .iter()
                    .map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
                    .collect();
                let traits = en.traits.iter().sorted().map(|trait_name| {
                    let trait_name = format_ident!("{}", trait_name);
                    quote!(impl ast::#trait_name for #name {})
                });

                let ast_node = if en.name == "Stmt" {
                    quote! {}
                } else {
                    quote! {
                        impl AstNode for #name {
                            #[inline]
                            fn can_cast(kind: SyntaxKind) -> bool {
                                matches!(kind, #(#kinds)|*)
                            }
                            #[inline]
                            fn cast(syntax: SyntaxNode) -> Option<Self> {
                                let res = match syntax.kind() {
                                    #(
                                    #kinds => #name::#variants(#variants { syntax }),
                                    )*
                                    _ => return None,
                                };
                                Some(res)
                            }
                            #[inline]
                            fn syntax(&self) -> &SyntaxNode {
                                match self {
                                    #(
                                    #name::#variants(it) => &it.syntax,
                                    )*
                                }
                            }
                        }
                    }
                };

                (
                    quote! {
                        #[pretty_doc_comment_placeholder_workaround]
                        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                        pub enum #name {
                            #(#variants(#variants),)*
                        }

                        #(#traits)*
                    },
                    quote! {
                        #(
                            impl From<#variants> for #name {
                                #[inline]
                                fn from(node: #variants) -> #name {
                                    #name::#variants(node)
                                }
                            }
                        )*
                        #ast_node
                    },
                )
            })
            .unzip();
        let (any_node_defs, any_node_boilerplate_impls): (Vec<_>, Vec<_>) = self
            .0
            .nodes
            .iter()
            .flat_map(|node| node.traits.iter().map(move |t| (t, node)))
            .into_group_map()
            .into_iter()
            .sorted_by_key(|(name, _)| *name)
            .map(|(trait_name, nodes)| {
                let name = format_ident!("Any{}", trait_name);
                let trait_name = format_ident!("{}", trait_name);
                let kinds: Vec<_> = nodes
                    .iter()
                    .map(|name| format_ident!("{}", to_upper_snake_case(&name.name.to_string())))
                    .collect();
                let nodes = nodes.iter().map(|node| format_ident!("{}", node.name));
                (
                    quote! {
                        #[pretty_doc_comment_placeholder_workaround]
                        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                        pub struct #name {
                            pub(crate) syntax: SyntaxNode,
                        }
                        impl ast::#trait_name for #name {}
                    },
                    quote! {
                        impl #name {
                            #[inline]
                            pub fn new<T: ast::#trait_name>(node: T) -> #name {
                                #name {
                                    syntax: node.syntax().clone()
                                }
                            }
                        }
                        impl AstNode for #name {
                            #[inline]
                            fn can_cast(kind: SyntaxKind) -> bool {
                                matches!(kind, #(#kinds)|*)
                            }
                            #[inline]
                            fn cast(syntax: SyntaxNode) -> Option<Self> {
                                Self::can_cast(syntax.kind()).then_some(#name { syntax })
                            }
                            #[inline]
                            fn syntax(&self) -> &SyntaxNode {
                                &self.syntax
                            }
                        }

                        #(
                            impl From<#nodes> for #name {
                                #[inline]
                                fn from(node: #nodes) -> #name {
                                    #name { syntax: node.syntax }
                                }
                            }
                        )*
                    },
                )
            })
            .unzip();

        let enum_names = self.0.enums.iter().map(|it| &it.name);
        let node_names = self.0.nodes.iter().map(|it| &it.name);

        let display_impls = enum_names
            .chain(node_names.clone())
            .map(|it| format_ident!("{}", it))
            .map(|name| {
                quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            std::fmt::Display::fmt(self.syntax(), f)
                        }
                    }
                }
            });

        let defined_nodes: HashSet<_> = node_names.collect();

        for node in self
            .1
            .nodes
            .iter()
            .map(|kind| to_pascal_case(kind))
            .filter(|name| !defined_nodes.iter().any(|&it| it == name))
        {
            drop(node)
            // FIXME: restore this
            // eprintln!("Warning: node {} not defined in ast source", node);
        }

        let ast = quote! {
            #![allow(non_snake_case)]
            use crate::{
                SyntaxNode, SyntaxToken, SyntaxKind::{self, *},
                ast::{self, AstNode, AstChildren, support},
                T,
            };

            #(#node_defs)*
            #(#enum_defs)*
            #(#any_node_defs)*
            #(#node_boilerplate_impls)*
            #(#enum_boilerplate_impls)*
            #(#any_node_boilerplate_impls)*
            #(#display_impls)*
        };

        let ast = ast.to_string().replace("T ! [", "T![");

        let mut res = String::with_capacity(ast.len() * 2);

        let mut docs = self
            .0
            .nodes
            .iter()
            .map(|it| &it.doc)
            .chain(self.0.enums.iter().map(|it| &it.doc));

        for chunk in ast.split("# [pretty_doc_comment_placeholder_workaround] ") {
            res.push_str(chunk);
            if let Some(doc) = docs.next() {
                write_doc_comment(doc, &mut res);
            }
        }

        let res = add_preamble(command, reformat(res));
        res.replace("#[derive", "\n#[derive")
    }
}
