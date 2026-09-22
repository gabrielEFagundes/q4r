/// The voidstar token types
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum VoidstarTokenTypes{
    Dot,
    Comma,

    Colon,
    SemiColon,

    Asterisk,
    Ampersand,

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
    CompEquals,
    NotEquals,

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

    Null,

    #[default] Halt,
}

/// The `Token` struct used by Q4r
#[derive(Debug, Default, Clone, Copy)]
pub struct VoidstarToken{
    pub(crate) token_type: VoidstarTokenTypes,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) last_could_end_stmt: bool
} 

impl VoidstarToken{
    /// Creates a new `VoidstarToken`
    pub fn new(token_type: VoidstarTokenTypes, start: usize, end: usize, last_could_end_stmt: bool) -> Self{
        Self { token_type, start, end, last_could_end_stmt }
    }

    pub fn token_type(&self) -> VoidstarTokenTypes{
        self.token_type
    }

    pub fn set_token_type(&mut self, new: VoidstarTokenTypes){
        self.token_type = new;
    }

    pub fn start(&self) -> usize{
        self.start
    }

    pub fn end(&self) -> usize{
        self.end
    }
}