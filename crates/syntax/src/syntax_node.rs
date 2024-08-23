pub use crate::denim_language::DenimLanguage;

pub type SyntaxNode = rowan::SyntaxNode<DenimLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<DenimLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<DenimLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<DenimLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<DenimLanguage>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<DenimLanguage>;
