// use crate::{ast, attr_kind::AttrKind, SmolStr};
use crate::{
    ast::{self, AstNode},
    attr_kind::AttrKind,
    SmolStr,
};

impl ast::Attr {
    pub fn as_simple_atom(&self) -> Option<SmolStr> {
        let meta = self.meta()?;
        if meta.eq_token().is_some() || meta.token_tree().is_some() {
            return None;
        }
        self.simple_name()
    }

    pub fn as_simple_call(&self) -> Option<(SmolStr, ast::TokenTree)> {
        let tt = self.meta()?.token_tree()?;
        Some((self.simple_name()?, tt))
    }

    pub fn simple_name(&self) -> Option<SmolStr> {
        let path = self.meta()?.path()?;
        match (path.segment(), path.qualifier()) {
            (Some(segment), None) => Some(segment.syntax().first_token()?.text().into()),
            _ => None,
        }
    }

    pub fn kind(&self) -> AttrKind {
        match self.bang_token() {
            Some(_) => AttrKind::Inner,
            None => AttrKind::Outer,
        }
    }

    pub fn path(&self) -> Option<ast::Path> {
        self.meta()?.path()
    }

    pub fn expr(&self) -> Option<ast::Expr> {
        self.meta()?.expr()
    }

    pub fn token_tree(&self) -> Option<ast::TokenTree> {
        self.meta()?.token_tree()
    }
}
