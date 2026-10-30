use ecow::EcoString;

use crate::{
    expdesc::Parameter, signature::Literal::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, Invalid, StringLiteral, VarLiteral,
    }, tokens::VoidstarTokenTypes,
};

const INT_DEF: isize = 0;
const FLOAT_DEF: f32 = 0.0;
const CHAR_DEF: char = ' ';
const BOOL_DEF: bool = false;

/// Usable types
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Type {
    #[default] Void,
    Int,
    Float,
    Char,
    Bool,
    Pointer(Box<Type>),

    Invalid
}

impl Type {
    /// Maps from `VoidstarTokenTypes` to `Type`
    pub fn map(token_type: VoidstarTokenTypes) -> Self {
        match token_type {
            VoidstarTokenTypes::Void => Self::Void,
            VoidstarTokenTypes::Int => Self::Int,
            VoidstarTokenTypes::Float => Self::Float,
            VoidstarTokenTypes::Char => Self::Char,
            VoidstarTokenTypes::Bool => Self::Bool,
            _ => Self::Invalid,
        }
    }

    pub fn map_pointer(self) -> Self {
        Self::Pointer(Box::new(self))
    }

    pub fn is_invalid(&self) -> bool{
        *self == Type::Invalid
    }

    pub fn to_byte_span(&self) -> Result<Vec<u8>, &str> {
        let mut vec = Vec::new();
        match self {
            Type::Void => vec.extend_from_slice(b"void"),
            Type::Int => vec.extend_from_slice(b"int"),
            Type::Float => vec.extend_from_slice(b"float"),
            Type::Char => vec.extend_from_slice(b"char"),
            Type::Bool => vec.extend_from_slice(b"bool"),
            Type::Pointer(val) => {
                vec.extend_from_slice(&val.to_byte_span()?);
                vec.extend_from_slice(b"*");
            },
            _ => return Err("unknown type found here"),
        };
        Ok(vec)
    }

    /// Returns `true` if the `VoidstarTokenTypes` type is valid inside `Type`, false otherwise
    pub fn has(token_type: VoidstarTokenTypes) -> bool {
        matches!(
            token_type,
            VoidstarTokenTypes::Void
                | VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Bool
        )
    }

    pub fn as_token_type(&self) -> Result<VoidstarTokenTypes, &str> {
        match self {
            Type::Void => Ok(VoidstarTokenTypes::Void),
            Type::Int => Ok(VoidstarTokenTypes::Int),
            Type::Float => Ok(VoidstarTokenTypes::Float),
            Type::Char => Ok(VoidstarTokenTypes::Char),
            Type::Bool => Ok(VoidstarTokenTypes::Bool),
            Type::Pointer(val) => Ok(val.as_token_type()?),
            Type::Invalid => Err("unknown type found here")
        }
    }

    pub fn default_literal(&self) -> Literal {
        match self {
            Type::Int => Literal::IntLiteral(INT_DEF),
            Type::Float => Literal::FloatLiteral(FLOAT_DEF),
            Type::Char => Literal::CharLiteral(CHAR_DEF),
            Type::Bool => Literal::BoolLiteral(BOOL_DEF),
            Type::Pointer(val) => Literal::PointerLiteral(Box::new(val.default_literal())),
            // default_literal() is only called on values (literals), that's why this is valid
            _ => Literal::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    IntLiteral(isize),
    FloatLiteral(f32),
    CharLiteral(char),
    BoolLiteral(bool),
    StringLiteral(EcoString),
    PointerLiteral(Box<Literal>),

    /// Used only on v0.1 snapshot, completely unstable in terms of updates
    VarLiteral(Vec<u8>),

    Invalid
}

impl Literal {
    /// Funny enough, this function is ALWAYS the source of most problems
    /// I've been finding so far
    /// 
    /// Certainly won't be used on v0.2
    pub fn to_byte_span(&self) -> Result<Vec<u8>, &str> {
        match self {
            Literal::IntLiteral(v) => Ok(v.to_string().into_bytes()),
            Literal::FloatLiteral(v) => Ok(v.to_string().into_bytes()),
            Literal::CharLiteral(v) => Ok(Vec::from([*v as u8])),
            Literal::BoolLiteral(v) => Ok(v.to_string().into_bytes()),
            Literal::PointerLiteral(v) => Ok(v.to_byte_span()?),
            Literal::StringLiteral(v) => Ok(v.as_bytes().to_vec()),
            Literal::VarLiteral(v) => Ok(v.to_vec()),
            Invalid => Err("invalid literal found here")
        }
    }

    /// The bytes are read as single values.
    ///
    /// This means that, for example, a value is 10, the
    /// converted value inside the byte slice would be
    /// `0x1` and `0x0` (`1` and `0`, separately)
    pub fn to_literal(bytes: &[u8], ty: VoidstarTokenTypes) -> Literal {
        match ty {
            VoidstarTokenTypes::Int => IntLiteral(Self::parse_int(bytes)),
            VoidstarTokenTypes::Float => FloatLiteral(Self::parse_float(bytes)),
            VoidstarTokenTypes::Bool => BoolLiteral(Self::parse_bool(bytes)),
            VoidstarTokenTypes::Char => CharLiteral(Self::parse_char(bytes)),
            VoidstarTokenTypes::Ident => VarLiteral(bytes.to_vec()),

            VoidstarTokenTypes::IntLiteral => IntLiteral(Self::parse_int(bytes)),
            VoidstarTokenTypes::FloatLiteral => FloatLiteral(Self::parse_float(bytes)),
            VoidstarTokenTypes::CharLiteral => CharLiteral(Self::parse_char(bytes)),
            VoidstarTokenTypes::BoolLiteral => BoolLiteral(Self::parse_bool(bytes)),
            VoidstarTokenTypes::StringLiteral => StringLiteral(Self::parse_str(bytes)),
            _ => Invalid
        }
    }

    pub fn from_literal(&self) -> VoidstarTokenTypes {
        match self {
            IntLiteral(_) => VoidstarTokenTypes::IntLiteral,
            FloatLiteral(_) => VoidstarTokenTypes::FloatLiteral,
            CharLiteral(_) => VoidstarTokenTypes::CharLiteral,
            BoolLiteral(_) => VoidstarTokenTypes::BoolLiteral,
            StringLiteral(_) => VoidstarTokenTypes::StringLiteral,
            VarLiteral(_) => VoidstarTokenTypes::Ident,
            _ => VoidstarTokenTypes::Halt
        }
    }

    pub fn to_type_token(ty: VoidstarTokenTypes) -> VoidstarTokenTypes {
        match ty {
            VoidstarTokenTypes::IntLiteral => VoidstarTokenTypes::Int,
            VoidstarTokenTypes::FloatLiteral => VoidstarTokenTypes::Float,
            VoidstarTokenTypes::BoolLiteral => VoidstarTokenTypes::Bool,
            VoidstarTokenTypes::CharLiteral => VoidstarTokenTypes::Char,
            _ => VoidstarTokenTypes::Halt,
        }
    }

    pub fn to_identifier(bytes: Vec<u8>) -> Literal {
        Literal::VarLiteral(bytes)
    }

    /// Guaranteed to be an integer at this point of compilation
    pub fn parse_int(bytes: &[u8]) -> isize {
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a float at this point of compilation
    pub fn parse_float(bytes: &[u8]) -> f32 {
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a char at this point of compilation
    pub fn parse_char(bytes: &[u8]) -> char {
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a string at this point of compilation
    pub fn parse_str(bytes: &[u8]) -> EcoString {
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a bool at this point of compilation
    pub fn parse_bool(bytes: &[u8]) -> bool {
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    EqualsEquals,
    Different,
    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
    Plus,
    Minus,
    Multiplication,
    Division,
    Increment,
    Decrement,
    Multiply,
    Divide,
    And,
    Or,
    Not,

    Invalid
}

impl Operator {
    pub fn map(token_type: VoidstarTokenTypes) -> Operator {
        match token_type {
            VoidstarTokenTypes::CompEquals => Operator::EqualsEquals,
            VoidstarTokenTypes::NotEquals => Operator::Different,
            VoidstarTokenTypes::Greater => Operator::Greater,
            VoidstarTokenTypes::GreaterEq => Operator::GreaterEq,
            VoidstarTokenTypes::Lesser => Operator::Lesser,
            VoidstarTokenTypes::LesserEq => Operator::LesserEq,
            VoidstarTokenTypes::Plus => Operator::Plus,
            VoidstarTokenTypes::Minus => Operator::Minus,
            VoidstarTokenTypes::Asterisk => Operator::Multiplication,
            VoidstarTokenTypes::Slash => Operator::Division,
            VoidstarTokenTypes::Increment => Operator::Increment,
            VoidstarTokenTypes::Decrement => Operator::Decrement,
            VoidstarTokenTypes::Multiply => Operator::Multiply,
            VoidstarTokenTypes::Divide => Operator::Divide,
            VoidstarTokenTypes::And => Operator::And,
            VoidstarTokenTypes::Or => Operator::Or,
            VoidstarTokenTypes::Not => Operator::Not,
            _ => Operator::Invalid,
        }
    }

    pub fn to_byte_span(&self) -> Result<&'static [u8], &str> {
        match self {
            Operator::EqualsEquals => Ok(b"=="),
            Operator::Different => Ok(b"!="),
            Operator::Greater => Ok(b">"),
            Operator::GreaterEq => Ok(b">="),
            Operator::Lesser => Ok(b"<"),
            Operator::LesserEq => Ok(b"<="),
            Operator::Plus => Ok(b"+"),
            Operator::Minus => Ok(b"-"),
            Operator::Multiplication => Ok(b"*"),
            Operator::Division => Ok(b"/"),
            Operator::Increment => Ok(b"+="),
            Operator::Decrement => Ok(b"-="),
            Operator::Multiply => Ok(b"*="),
            Operator::Divide => Ok(b"/="),
            Operator::And => Ok(b"&&"),
            Operator::Or => Ok(b"||"),
            Operator::Not => Ok(b"!"),
            _ => Err("unknown operator found here")
        }
    }

    pub fn is_comparative(token_type: VoidstarTokenTypes) -> bool {
        matches!(
            token_type,
            VoidstarTokenTypes::CompEquals
                | VoidstarTokenTypes::NotEquals
                | VoidstarTokenTypes::Greater
                | VoidstarTokenTypes::GreaterEq
                | VoidstarTokenTypes::Lesser
                | VoidstarTokenTypes::LesserEq
        )
    }

    pub fn is_boolean_algebra(&self) -> bool {
        matches!(self, Operator::And | Operator::Or)
    }

    pub fn is_arithmetic(token_type: VoidstarTokenTypes) -> bool {
        matches!(
            token_type,
            VoidstarTokenTypes::Plus
                | VoidstarTokenTypes::Minus
                | VoidstarTokenTypes::Slash
                | VoidstarTokenTypes::Asterisk
        )
    }

    pub fn is_legal(token_type: VoidstarTokenTypes) -> bool {
        matches!(
            token_type,
            VoidstarTokenTypes::Plus
                | VoidstarTokenTypes::Minus
                | VoidstarTokenTypes::Slash
                | VoidstarTokenTypes::Asterisk
                | VoidstarTokenTypes::CompEquals
                | VoidstarTokenTypes::NotEquals
                | VoidstarTokenTypes::Greater
                | VoidstarTokenTypes::GreaterEq
                | VoidstarTokenTypes::Lesser
                | VoidstarTokenTypes::LesserEq
                | VoidstarTokenTypes::And
                | VoidstarTokenTypes::Or
                | VoidstarTokenTypes::Not
                | VoidstarTokenTypes::Ident
                | VoidstarTokenTypes::IntLiteral
                | VoidstarTokenTypes::FloatLiteral
        )
    }
}

#[derive(Debug)]
pub enum DecKind {
    If,
    For,
    While,
    
    Invalid
}

impl DecKind {
    pub fn map(token_type: VoidstarTokenTypes) -> Self {
        match token_type {
            VoidstarTokenTypes::If => Self::If,
            VoidstarTokenTypes::For => Self::For,
            _ => Self::Invalid,
        }
    }

    pub fn to_byte_span(&self) -> Result<&'static [u8], &str> {
        match self {
            DecKind::If => Ok(b"if"),
            DecKind::For => Ok(b"for"),
            DecKind::While => Ok(b"while"),
            _ => Err("unknown declaration type found here")
        }
    }
}

/// Each signature must hold the function's name, parameters and return type
///
/// Or, if that's the case, the type and value of the global variable
pub enum Signature<'a> {
    Function {
        returns: Type,
        params: Vec<Parameter<'a>>,
        is_extern: bool,
    },
    Global {
        ty: Type,
        value: Vec<u8>,
    },
}

impl<'a> Signature<'a> {
    pub fn destructure_fun(v: Signature) -> (Type, Vec<Parameter>, bool) {
        if let Signature::Function {
            returns,
            params,
            is_extern,
        } = v
        {
            return (returns, params, is_extern);
        }
        panic!("rust: bad call of `destructure_fun`") // internal
    }

    pub fn destructure_glob(v: &'a Signature) -> (&'a Type, &'a Vec<u8>) {
        if let Signature::Global { ty, value } = v {
            return (ty, value);
        }
        panic!("rust: bad call of `destructure_glob`") // internal
    }
}
