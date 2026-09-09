/// The voidstar token types
#[derive(Debug, Copy, Clone)]
pub enum VoidstarTokenTypes{
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

    Greater,
    Lesser,
    GreaterEq,
    LesserEq,
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

    Increment,
    Decrement,

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

impl VoidstarToken{
    /// Creates a new `VoidstarToken`
    pub fn new(token_type: VoidstarTokenTypes, content: String) -> VoidstarToken{
        VoidstarToken { token_type, content }
    }
}