use crate::tokens::VoidstarTokenTypes;

pub mod tokens;
pub mod lexer;
pub mod signatures;

const OPEN_PAREN: u8 = b'(';
const CLOSE_PAREN: u8 = b')';
const OPEN_BRACE: u8 = b'{';
const CLOSE_BRACE: u8 = b'}';
const OPEN_BRACKET: u8 = b'[';
const CLOSE_BRACKET: u8 = b']';

const APOSTROPHE: u8 = b'\'';
const QUOTES: u8 = b'\"';

const DOT: u8 = b'.';
const COMMA: u8 = b',';
const UNDERSCORE: u8 = b'_';

const COLON: u8 = b':';
const SEMICOL: u8 = b';';

const PLUS: u8 = b'+';
const MINUS: u8 = b'-';
const SLASH: u8 = b'/';

const GREATER: u8 = b'>';
const LESSER: u8 = b'<';
const EQ: u8 = b'=';
const NOT: u8 = b'!';
const ASTRSK: u8 = b'*';

const LINE_FEED: u8 = b'\n';
const TABULATION: u8 = b'\t';
const SPACE: u8 = b' ';

const HALT: u8 = b'\0';

const KEYWORDS: [(&[u8], VoidstarTokenTypes); 18] = [
    (b"bool",        VoidstarTokenTypes::Bool),
    (b"char",        VoidstarTokenTypes::Char),
    (b"cross",       VoidstarTokenTypes::Cross),
    (b"else",        VoidstarTokenTypes::Else),
    (b"f",           VoidstarTokenTypes::Function),
    (b"false",       VoidstarTokenTypes::BoolLiteral),
    (b"float",       VoidstarTokenTypes::Float),
    (b"for",         VoidstarTokenTypes::For),
    (b"goto",        VoidstarTokenTypes::Goto),
    (b"if",          VoidstarTokenTypes::If),
    (b"int",         VoidstarTokenTypes::Int),
    (b"return",      VoidstarTokenTypes::Return),
    (b"static",      VoidstarTokenTypes::Static),
    (b"true",        VoidstarTokenTypes::BoolLiteral),
    (b"use",         VoidstarTokenTypes::Use),
    (b"void",        VoidstarTokenTypes::Void),
    (b"while",       VoidstarTokenTypes::While),
    (b"workspace",   VoidstarTokenTypes::Workspace),
];

const SYMBOLS: [(u8, VoidstarTokenTypes); 16] = [
    (b'!', VoidstarTokenTypes::Not),
    (b'(', VoidstarTokenTypes::OpenParents),
    (b')', VoidstarTokenTypes::CloseParents),
    (b'*', VoidstarTokenTypes::Asterisk),
    (b'+', VoidstarTokenTypes::Plus),
    (b',', VoidstarTokenTypes::Comma),
    (b'-', VoidstarTokenTypes::Minus),
    (b'.', VoidstarTokenTypes::Dot),
    (b'/', VoidstarTokenTypes::Slash),
    (b':', VoidstarTokenTypes::Colon),
    (b';', VoidstarTokenTypes::SemiColon),
    (b'=', VoidstarTokenTypes::Equals),
    (b'[', VoidstarTokenTypes::OpenBrackets),
    (b']', VoidstarTokenTypes::CloseBrackets),
    (b'{', VoidstarTokenTypes::OpenBraces),
    (b'}', VoidstarTokenTypes::CloseBraces),
];