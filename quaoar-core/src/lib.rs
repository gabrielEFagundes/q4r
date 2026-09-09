use crate::tokens::VoidstarTokenTypes;

pub mod tokens;
pub mod lexer;

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

// abcdefghijklmnopqrstuvwxyz
const KEYWORDS: [(&str, VoidstarTokenTypes); 18] = [
    ("bool",        VoidstarTokenTypes::Bool),
    ("char",        VoidstarTokenTypes::Char),
    ("cross",       VoidstarTokenTypes::Cross),
    ("else",        VoidstarTokenTypes::Else),
    ("f",           VoidstarTokenTypes::Function),
    ("false",       VoidstarTokenTypes::BoolLiteral),
    ("float",       VoidstarTokenTypes::Float),
    ("for",         VoidstarTokenTypes::For),
    ("goto",        VoidstarTokenTypes::Goto),
    ("if",          VoidstarTokenTypes::If),
    ("int",         VoidstarTokenTypes::Int),
    ("return",      VoidstarTokenTypes::Return),
    ("static",      VoidstarTokenTypes::Static),
    ("true",        VoidstarTokenTypes::BoolLiteral),
    ("use",         VoidstarTokenTypes::Use),
    ("void",        VoidstarTokenTypes::Void),
    ("while",       VoidstarTokenTypes::While),
    ("workspace",   VoidstarTokenTypes::Workspace),
];

const SYMBOLS: [(char, VoidstarTokenTypes); 16] = [
    ('!', VoidstarTokenTypes::Not),
    ('(', VoidstarTokenTypes::OpenParents),
    (')', VoidstarTokenTypes::CloseParents),
    ('*', VoidstarTokenTypes::Asterisk),
    ('+', VoidstarTokenTypes::Plus),
    (',', VoidstarTokenTypes::Comma),
    ('-', VoidstarTokenTypes::Minus),
    ('.', VoidstarTokenTypes::Dot),
    ('/', VoidstarTokenTypes::Slash),
    (':', VoidstarTokenTypes::Colon),
    (';', VoidstarTokenTypes::SemiColon),
    ('=', VoidstarTokenTypes::Equals),
    ('[', VoidstarTokenTypes::OpenBrackets),
    (']', VoidstarTokenTypes::CloseBrackets),
    ('{', VoidstarTokenTypes::OpenBraces),
    ('}', VoidstarTokenTypes::CloseBraces),
];