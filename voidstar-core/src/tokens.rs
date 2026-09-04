/// The voidstar token types
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

    Equals,

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

    Function,
    Goto,
    Cross,
    Return,
}

/// The `Token` struct used by voidstar
pub(crate) struct VoidstarToken{
    token_type: VoidstarTokenTypes,
    content: String
}

impl VoidstarTokenTypes{
    /// Maps from `VoidstarTokenType` to `VoidstarToken`
    pub fn map(self) -> VoidstarToken{
        let temp_content = match self {
            VoidstarTokenTypes::Dot => ".".to_string(),
            VoidstarTokenTypes::Comma => ",".to_string(),
            VoidstarTokenTypes::Colon => ":".to_string(),
            VoidstarTokenTypes::SemiColon => ";".to_string(),
            VoidstarTokenTypes::Asterisk => "*".to_string(),
            VoidstarTokenTypes::OpenParents => "(".to_string(),
            VoidstarTokenTypes::CloseParents => ")".to_string(),
            VoidstarTokenTypes::OpenBraces => "{".to_string(),
            VoidstarTokenTypes::CloseBraces => "}".to_string(),
            VoidstarTokenTypes::OpenBrackets => "[".to_string(),
            VoidstarTokenTypes::CloseBrackets => "]".to_string(),
            VoidstarTokenTypes::Equals => "=".to_string(),
            VoidstarTokenTypes::If => "if".to_string(),
            VoidstarTokenTypes::Else => "else".to_string(),
            VoidstarTokenTypes::While => "while".to_string(),
            VoidstarTokenTypes::For => "for".to_string(),
            VoidstarTokenTypes::Workspace => "workspace".to_string(),
            VoidstarTokenTypes::Use => "use".to_string(),
            VoidstarTokenTypes::Static => "static".to_string(),
            VoidstarTokenTypes::Annotation => "annotation".to_string(),
            VoidstarTokenTypes::Int => "int".to_string(),
            VoidstarTokenTypes::Float => "float".to_string(),
            VoidstarTokenTypes::Bool => "bool".to_string(),
            VoidstarTokenTypes::Char => "char".to_string(),
            VoidstarTokenTypes::Void => "void".to_string(),
            VoidstarTokenTypes::Function => "f".to_string(),
            VoidstarTokenTypes::Return => "return".to_string(),
            VoidstarTokenTypes::Goto => "goto".to_string(),
            VoidstarTokenTypes::Cross => "cross".to_string(),
        };

        VoidstarToken { token_type: self, content: temp_content }
    }
}