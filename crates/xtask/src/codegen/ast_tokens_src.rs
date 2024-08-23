use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{codegen::english_util::to_upper_snake_case, CodegenCommands};

use super::{
    ast_src::AstSrc,
    codegen_util::{add_preamble, reformat},
};

pub struct AstTokensSrc {
    tokens: Vec<TokenStream>,
}

impl AstTokensSrc {
    pub fn generate(ast: &AstSrc) -> AstTokensSrc {
        let tokens = ast.tokens.iter().map(|token| {
            let name = format_ident!("{}", token);
            let kind = format_ident!("{}", to_upper_snake_case(token));

            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub struct #name {
                    pub(crate) syntax: SyntaxToken,
                }
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(&self.syntax, f)
                    }
                }
                impl AstToken for #name {
                    fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }
                    fn cast(syntax: SyntaxToken) -> Option<Self> {
                        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                    }
                    fn syntax(&self) -> &SyntaxToken { &self.syntax }
                }
            }
        });

        AstTokensSrc {
            tokens: tokens.collect(),
        }
    }

    pub fn print(self, command: &CodegenCommands) -> String {
        let AstTokensSrc { tokens } = self;

        add_preamble(
            command,
            reformat(
                quote! {
                    use crate::{SyntaxKind::{self, *}, SyntaxToken, ast::AstToken};
                    #(#tokens)*
                }
                .to_string(),
            ),
        )
        .replace("#[derive", "\n#[derive")
    }
}
