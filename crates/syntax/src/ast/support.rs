use either::Either;

use super::{AstChildren, AstNode, HasAttrs, SyntaxKind, SyntaxNode, SyntaxToken};

#[inline]
pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(N::cast)
}

#[inline]
pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
    AstChildren::new(parent)
}

#[inline]
pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent.children_with_tokens().filter_map(|it| it.into_token()).find(|it| it.kind() == kind)
}

impl<L, R> AstNode for Either<L, R>
where
    L: AstNode,
    R: AstNode,
{
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        L::can_cast(kind) || R::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if L::can_cast(syntax.kind()) {
            L::cast(syntax).map(Either::Left)
        } else {
            R::cast(syntax).map(Either::Right)
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        self.as_ref().either(L::syntax, R::syntax)
    }
}

impl<L, R> HasAttrs for Either<L, R>
where
    L: HasAttrs,
    R: HasAttrs,
{
}
