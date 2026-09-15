// This parser is only used to generate the signatures table in memory
// It does not generate C or ASM code.

use std::collections::HashMap;

use crate::{signatures::Type::Void, tokens::{VoidstarToken, VoidstarTokenTypes}};

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

    pub fn default_literal(&self) -> Literal{
        match self{
            Type::Int => Literal::IntLiteral(0),
            Type::Float => Literal::FloatLiteral(0.0),
            Type::Char => Literal::CharLiteral(' '),
            Type::Bool => Literal::BoolLiteral(false),
            Type::Pointer(_) => todo!("pointers not implemented yet"),
            // default_literal() is only called on variables, that's why this is valid
            _ => panic!("invalid syntax: `void` on variable type")
        }
    }
}

pub enum Literal{
    IntLiteral(usize), FloatLiteral(f32), CharLiteral(char), BoolLiteral(bool)
}

impl Literal{
    pub fn to_byte_span(&self) -> Vec<u8>{
        match self{
            Literal::IntLiteral(v) => v.to_le_bytes().to_vec(),
            Literal::FloatLiteral(v) => v.to_le_bytes().to_vec(),
            Literal::CharLiteral(v) => Vec::from([*v as u8]),
            Literal::BoolLiteral(v) => Vec::from([u8::from(*v)]),
        }
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

/// Main struct used for mounting the table that holds the signatures
pub struct SignatureMounter<'a>{
    source: &'a[u8],
    tokens: &'a[VoidstarToken],
    cursor: usize,
    current_token: VoidstarToken
}

// fix because ugly
impl<'a> SignatureMounter<'a>{
    pub fn new(source: &'a[u8], tokens: &'a[VoidstarToken]) -> Self{
        Self { source, tokens, cursor: 0, current_token: VoidstarToken::default() }
    }

    fn forward(&mut self){
        self.cursor+=1;
        if self.cursor < self.tokens.len(){ self.current_token = self.tokens[self.cursor]; }
    }

    fn peekaboo(&mut self) -> VoidstarToken{
        if self.cursor+1 < self.tokens.len(){ return self.tokens[self.cursor+1] }
        self.tokens[self.cursor]
    }

    pub fn mount(&mut self) -> HashMap<String, Signature>{
        let mut map: HashMap<String, Signature> = HashMap::new();
        self.current_token = self.tokens[self.cursor];

        while self.cursor < self.tokens.len()-1{
            match self.current_token.token_type{
                VoidstarTokenTypes::Function => {
                    self.forward();
                    let ident = &self.source[self.current_token.start..self.current_token.end];
                    let mut params: Vec<Type> = Vec::new();

                    self.forward();
                    while self.current_token.token_type != VoidstarTokenTypes::CloseParents{
                        self.forward();
                        if Type::has(self.current_token.token_type){
                            params.push(Type::map(self.current_token.token_type));
                        }
                    }
                    self.forward();
                    let returns = Type::map(self.current_token.token_type);
                    
                    self.forward();
                    map.insert(
                        unsafe{ str::from_utf8_unchecked(ident).to_string() },
                        Signature::Function { returns, params }
                    );
                },
                VoidstarTokenTypes::Void
                |VoidstarTokenTypes::Int
                |VoidstarTokenTypes::Float
                |VoidstarTokenTypes::Char
                |VoidstarTokenTypes::Bool => {
                    let ty = Type::map(self.current_token.token_type);
                    self.forward();

                    let ident = &self.source[self.current_token.start..self.current_token.end];
                    let mut value = ty.default_literal().to_byte_span();

                    if self.peekaboo().token_type == VoidstarTokenTypes::Equals{
                        self.forward(); self.forward();
                        value = self.source[self.current_token.start..self.current_token.end].to_vec();
                    }

                    map.insert(unsafe{ str::from_utf8_unchecked(ident).to_string() }, Signature::Global { ty, value });
                },
                _ => self.forward()
            }
        }
        map
    }
}