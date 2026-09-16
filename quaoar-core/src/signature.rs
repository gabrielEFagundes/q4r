use std::collections::HashMap;

use crate::{signature::Literal::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral}, tokens::{VoidstarToken, VoidstarTokenTypes}};

const INT_DEF: isize = 0;
const FLOAT_DEF: f32 = 0.0;
const CHAR_DEF: char = ' ';
const BOOL_DEF: bool = false;

/// Usable types
#[derive(Debug, PartialEq, Eq)]
pub enum Type{
    Void, Int, Float, Char, Bool,
    Pointer(Box<Type>)
}

impl Type{
    /// Maps from `VoidstarTokenTypes` to `Type`
    pub fn map(token_type: VoidstarTokenTypes) -> Self{
        match token_type{
            VoidstarTokenTypes::Void => Self::Void,
            VoidstarTokenTypes::Int => Self::Int,
            VoidstarTokenTypes::Float => Self::Float,
            VoidstarTokenTypes::Char => Self::Char,
            VoidstarTokenTypes::Bool => Self::Bool,
            _ => panic!("invalid data type `{:#?}`", token_type)
        }
    }

    /// Maps from a `VoidstarTokenTypes` representing a Literal type to `Type`
    pub fn map_literal_token(token_type: VoidstarTokenTypes) -> Self{
        match token_type{
            VoidstarTokenTypes::IntLiteral => Self::Int,
            VoidstarTokenTypes::FloatLiteral => Self::Float,
            VoidstarTokenTypes::CharLiteral => Self::Char,
            VoidstarTokenTypes::BoolLiteral => Self::Bool,
            _ => panic!("invalid data type `{:#?}`", token_type)
        }
    }

    pub fn to_byte_span(&self) -> &'static[u8]{
        match self{
            Type::Void => b"void",
            Type::Int => b"int",
            Type::Float => b"float",
            Type::Char => b"char",
            Type::Bool => b"bool",
            Type::Pointer(_) => todo!(),
        }
    }

    /// Returns `true` if the `VoidstarTokenTypes` type is valid inside `Type`, false otherwise
    pub fn has(token_type: VoidstarTokenTypes) -> bool{
        match token_type{
            VoidstarTokenTypes::Void
            |VoidstarTokenTypes::Int
            |VoidstarTokenTypes::Float
            |VoidstarTokenTypes::Char
            |VoidstarTokenTypes::Bool => true,
            _ => false
        }
    }

    pub fn as_token_type(&self) -> VoidstarTokenTypes{
        match self{
            Type::Void => VoidstarTokenTypes::Void,
            Type::Int => VoidstarTokenTypes::Int,
            Type::Float => VoidstarTokenTypes::Float,
            Type::Char => VoidstarTokenTypes::Char,
            Type::Bool => VoidstarTokenTypes::Bool,
            _ => panic!("bad conversion from `{:#?}` to token_type", self)
        }
    }

    pub fn default_literal(&self) -> Literal{
        match self{
            Type::Int => Literal::IntLiteral(INT_DEF),
            Type::Float => Literal::FloatLiteral(FLOAT_DEF),
            Type::Char => Literal::CharLiteral(CHAR_DEF),
            Type::Bool => Literal::BoolLiteral(BOOL_DEF),
            Type::Pointer(_) => todo!("pointers not implemented yet"),
            // default_literal() is only called on values, that's why this is valid
            _ => panic!("invalid syntax: `{:#?}` on variable type", self)
        }
    }
}

#[derive(Debug)]
pub enum Literal{
    IntLiteral(isize), FloatLiteral(f32), CharLiteral(char), BoolLiteral(bool), 
    
    #[deprecated = "Used only on v0.1 snapshot, completely unstable in terms of updates"] 
    VarLiteral(&'static[u8])
}

impl Literal{
    /// Funny enough, this function is ALWAYS the source of most problems
    /// I've been finding so far
    pub fn to_byte_span(&self) -> Vec<u8>{
        match self{
            Literal::IntLiteral(v) => v.to_string().into_bytes(),
            Literal::FloatLiteral(v) => v.to_string().into_bytes(),
            Literal::CharLiteral(v) => Vec::from([*v as u8]),
            Literal::BoolLiteral(v) => v.to_string().into_bytes(),
            Literal::VarLiteral(v) => v.to_vec()
        }
    }

    /// The bytes are read as single values.
    /// 
    /// This means that, for example, a value is 10, the
    /// converted value inside the byte slice would be
    /// `0x1` and `0x0` (`1` and `0`, separately)
    pub fn to_literal(bytes: &[u8], ty: VoidstarTokenTypes) -> Literal{
        match ty{
            VoidstarTokenTypes::Int => IntLiteral(Self::parse_int(bytes)),
            VoidstarTokenTypes::Float => FloatLiteral(Self::parse_float(bytes)),
            VoidstarTokenTypes::Bool => BoolLiteral(Self::parse_bool(bytes)),
            VoidstarTokenTypes::Char => CharLiteral(Self::parse_char(bytes)),
            _ => panic!("invalid syntax: `{:#?}` on variable type", ty)
        }
    }

    #[deprecated = "Used only on v0.1 snapshot, completely unstable in terms of updates"]
    pub fn to_identifier(bytes: &'static[u8]) -> Literal{
        Literal::VarLiteral(bytes)
    }

    /// Guaranteed to be an integer at this point of compilation
    pub fn parse_int(bytes: &[u8]) -> isize{
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a float at this point of compilation
    pub fn parse_float(bytes: &[u8]) -> f32{
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a char at this point of compilation
    pub fn parse_char(bytes: &[u8]) -> char{
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }

    /// Guaranteed to be a bool at this point of compilation
    pub fn parse_bool(bytes: &[u8]) -> bool{
        std::str::from_utf8(bytes).unwrap().parse().unwrap()
    }
}

#[derive(Clone, Copy)]
pub enum Operator{
    EqualsEquals, Greater, GreaterEq, Lesser, LesserEq,
    Plus, Minus, Multiplication, Division
}

impl Operator{
    pub fn map(token_type: VoidstarTokenTypes) -> Operator{
        match token_type{
            VoidstarTokenTypes::CompEquals => Operator::EqualsEquals,
            VoidstarTokenTypes::Greater => Operator::Greater,
            VoidstarTokenTypes::GreaterEq => Operator::GreaterEq,
            VoidstarTokenTypes::Lesser => Operator::Lesser,
            VoidstarTokenTypes::LesserEq => Operator::LesserEq,
            VoidstarTokenTypes::Plus => Operator::Plus,
            VoidstarTokenTypes::Minus => Operator::Minus,
            VoidstarTokenTypes::Asterisk => Operator::Multiplication,
            VoidstarTokenTypes::Slash => Operator::Division,
            _ => panic!("invalid operator type `{:#?}`", token_type)
        }
    }

    pub fn to_byte_span(&self) -> &'static[u8]{
        match self{
            Operator::EqualsEquals => b"==",
            Operator::Greater => b">",
            Operator::GreaterEq => b">=",
            Operator::Lesser => b"<",
            Operator::LesserEq => b"<=",
            Operator::Plus => b"+",
            Operator::Minus => b"-",
            Operator::Multiplication => b"*",
            Operator::Division => b"/",
        }
    }

    pub fn is_comparative(token_type: VoidstarTokenTypes) -> bool{
        match token_type{
            VoidstarTokenTypes::Equals
            | VoidstarTokenTypes::Greater
            | VoidstarTokenTypes::GreaterEq
            | VoidstarTokenTypes::Lesser
            | VoidstarTokenTypes::LesserEq => true,
            _ => false
        }
    }

    pub fn is_arithmetic(token_type: VoidstarTokenTypes) -> bool{
        match token_type{
            VoidstarTokenTypes::Plus
            |VoidstarTokenTypes::Minus
            |VoidstarTokenTypes::Slash
            |VoidstarTokenTypes::Asterisk => true,
            _ => false
        }
    }

    pub fn is_legal(token_type: VoidstarTokenTypes) -> bool{
        match token_type{
            VoidstarTokenTypes::Plus
            | VoidstarTokenTypes::Minus
            | VoidstarTokenTypes::Slash
            | VoidstarTokenTypes::Asterisk
            | VoidstarTokenTypes::Equals
            | VoidstarTokenTypes::Greater
            | VoidstarTokenTypes::GreaterEq
            | VoidstarTokenTypes::Lesser
            | VoidstarTokenTypes::LesserEq
            | VoidstarTokenTypes::Ident
            | VoidstarTokenTypes::IntLiteral
            | VoidstarTokenTypes::FloatLiteral => true,
            _ => false
        }
    }
}

pub enum DecKind{
    If, While, For
}

impl DecKind{
    pub fn map(token_type: VoidstarTokenTypes) -> Self{
        match token_type{
            VoidstarTokenTypes::If => Self::If,
            VoidstarTokenTypes::While => Self::While,
            VoidstarTokenTypes::For => Self::For,
            _ => panic!("invalid declaration kind `{:#?}`", token_type)
        }
    }

    pub fn to_byte_span(&self) -> &'static[u8]{
        match self{
            DecKind::If => b"if",
            DecKind::While => b"while",
            DecKind::For => b"for",
        }
    }
}

/// Each signature must hold the function's name, parameters and return type
///
/// Or, if that's the case, the type and value of the global variable
#[derive(Debug)]
pub enum Signature{
    Function{ returns: Type, params: Vec<Type> },
    Global{ ty: Type, value: Vec<u8> }
}

impl Signature{
    pub fn destructure_fun(v: Signature) -> (Type, Vec<Type>){
        if let Signature::Function { returns, params } = v{
            return (returns, params)
        }
        panic!("bad call of `destructure_fun`")
    }

    pub fn destructure_glob(v: &Signature) -> (&Type, &Vec<u8>){
        if let Signature::Global { ty, value } = v{
            return (ty, value)
        }
        panic!("bad call of `destructure_glob`")
    }
}