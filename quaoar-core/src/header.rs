// This parser is only used to generate the signatures table in memory
// It does not generate C or ASM code.

use std::collections::HashMap;

use crate::{signature::{Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes}};

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
                    let returns = if Type::has(self.peekaboo().token_type){
                        self.forward();
                        Type::map(self.current_token.token_type)
                    } else {
                        Type::Void
                    };
                    
                    self.forward();
                    map.insert(
                        unsafe{ str::from_utf8_unchecked(ident).to_string() },
                        Signature::Function { returns, params }
                    );

                    // Doesn't take any scoped variables inside the function.
                    while self.current_token.token_type != VoidstarTokenTypes::CloseBraces{ self.forward(); }
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