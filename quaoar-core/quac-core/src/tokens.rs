/// The voidstar token types
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum VoidstarTokenTypes {
    Dot,
    DotDot,
    Ellipsis,

    Comma,

    Colon,
    SemiColon,

    Asterisk,

    Ampersand,
    And,
    Pipe,
    Or,

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
    BackSlash,

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
    Extern,
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
    StringLiteral,

    Increment,
    Decrement,
    Multiply,
    Divide,

    Function,
    Goto,
    Cross,
    Return,

    Null,

    #[default]
    Halt,
}

#[derive(Default, Clone, Copy)]
pub struct Span{
    /// Starting byte position
    pub start: usize,
    /// Ending byte position
    pub end: usize,
    /// The span's line
    pub line: usize
}

/// The `Token` struct used by Q4r
#[derive(Default, Clone, Copy)]
pub struct VoidstarToken {
    pub(crate) token_type: VoidstarTokenTypes,
    pub(crate) span: Span,
    pub(crate) last_could_end_stmt: bool,
}

impl VoidstarToken {
    /// Creates a new `VoidstarToken`
    pub fn new(
        token_type: VoidstarTokenTypes,
        span: Span,
        last_could_end_stmt: bool,
    ) -> Self {
        Self {
            token_type,
            span,
            last_could_end_stmt,
        }
    }

    #[inline]
    pub fn token_type(&self) -> VoidstarTokenTypes {
        self.token_type
    }

    #[inline]
    pub fn set_token_type(&mut self, new: VoidstarTokenTypes) {
        self.token_type = new;
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.span.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.span.end
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.span.line
    }

    #[inline]
    pub fn span(&self) -> Span{
        self.span
    }
}
