/// The voidstar token types
#[derive(Debug, Copy, Clone, PartialEq, Default)]
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

    #[default] Halt,
}

/// The `Token` struct used by Q4r
#[derive(Debug, Default, Clone, Copy)]
pub struct VoidstarToken{
    pub(crate) token_type: VoidstarTokenTypes,
    pub(crate) start: usize,
    pub(crate) end: usize
} 

impl VoidstarToken{
    /// Creates a new `VoidstarToken`
    pub fn new(token_type: VoidstarTokenTypes, start: usize, end: usize) -> Self{
        Self { token_type, start, end }
    }
}