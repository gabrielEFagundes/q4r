// This parser is only used to generate the signatures table in memory
// It does not generate C or ASM code.

use std::collections::HashMap;

use crate::tokens::{self, VoidstarToken, VoidstarTokenTypes};

/// Usable types
#[derive(Debug)]
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
            _ => panic!("invalid data type {:#?}", token_type)
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
}

/// Each signature must hold the function's name, parameters and return type
///
/// Or, if that's the case, the type of the global variable
#[derive(Debug)]
pub enum Signature{
    Function{ returns: Type, params: Vec<Type> },
    Global{ ty: Type }
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

    pub fn mount(&mut self) -> HashMap<String, Signature>{
        let mut map: HashMap<String, Signature> = HashMap::new();
        self.current_token = self.tokens[self.cursor];

        while self.cursor < self.tokens.len()-1{
            println!("{:#?}", self.current_token);
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
                    map.insert(unsafe{ str::from_utf8_unchecked(ident).to_string() }, Signature::Global { ty });
                },
                _ => self.forward()
            }
        }
        println!("{:#?}", map);
        map
    }
}