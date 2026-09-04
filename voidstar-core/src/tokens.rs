/// The voidstar token types
#[derive(Debug, Copy, Clone)]
pub(crate) enum VoidstarTokenTypes{
    Dot,
    Comma,

    Colon,
    SemiColon,

    Asterisk,

    OpenParents,
    CloseParents,
    OpenBraces,
    CloseBraces,
    OpenBrackets,
    CloseBrackets,

    Quotes,
    Apostrophe,

    Plus,
    Minus,
    Times,
    Slash,

    Equals,
    Not,

    If,
    Else,
    While,
    For,

    Workspace,
    Use,
    Static,
    Annotation,
    
    Int,
    Float,
    Bool,
    Char,
    Void,
    Ident,

    IntLiteral,
    FloatLiteral,
    BoolLiteral,
    CharLiteral,

    Function,
    Goto,
    Cross,
    Return,

    Halt,
}

/// The `Token` struct used by voidstar
#[derive(Debug)]
pub struct VoidstarToken{
    token_type: VoidstarTokenTypes,
    content: String
} 

impl VoidstarTokenTypes{
    /// Creates a new `VoidstarTokenTypes`
    pub fn new(self, content: String) -> VoidstarToken{
        VoidstarToken { token_type: self, content }
    }
}