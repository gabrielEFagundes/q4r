// This parser is only used to generate the signatures table in memory
// It does not generate C or ASM code.

use std::collections::HashMap;

use crate::{expdesc::Parameter, signature::{Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes}};

/// Main struct used for mounting the table that holds the signatures
pub struct SignatureMounter<'a>{
    source: &'a[u8],
    tokens: &'a[VoidstarToken],
    cursor: usize,
    current_token: VoidstarToken
}

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

    fn is_pointer(&mut self) -> bool{
        if self.current_token.token_type == VoidstarTokenTypes::Asterisk{
            return true
        }
        false
    }

    fn mount_function(&mut self, map: &mut HashMap<String, Signature<'a>>, is_function_extern: bool){
        self.forward();
        let ident = &self.source[self.current_token.start..self.current_token.end];
        let mut params: Vec<Parameter> = Vec::new();

        // fucking mess
        self.forward(); self.forward();
        while self.current_token.token_type != VoidstarTokenTypes::CloseParents{
            if self.current_token.token_type == VoidstarTokenTypes::Comma{
                self.forward();
                continue;
            }

            if self.current_token.token_type == VoidstarTokenTypes::Ellipsis{
                params.push(Parameter { 
                    ty: Type::Void, ident: &b"..."[..], is_etc: true
                });
                self.forward();
                break;
            }

            let ty: Type;
            if self.is_pointer(){
                self.forward();
                ty = Type::map_pointer(Type::map(self.current_token.token_type));

            } else {
                ty = Type::map(self.current_token.token_type);
            }

            self.forward();
            let ident = &self.source[self.current_token.start..self.current_token.end];
            params.push(Parameter {
                ty, ident, is_etc: false
            });
            self.forward();
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
            Signature::Function { returns, params, is_extern: is_function_extern }
        );
    }

    pub fn mount(&mut self) -> HashMap<String, Signature<'a>>{
        let mut map: HashMap<String, Signature> = HashMap::new();
        self.current_token = self.tokens[self.cursor];

        while self.cursor < self.tokens.len()-1{
            match self.current_token.token_type{
                VoidstarTokenTypes::Extern => {
                    self.forward();
                    
                    match self.current_token.token_type{
                        VoidstarTokenTypes::OpenBraces => {
                            self.forward();
                            while self.current_token.token_type != VoidstarTokenTypes::CloseBraces{
                                self.mount_function(&mut map, true);
                                self.forward();
                            }
                        },

                        VoidstarTokenTypes::Function => {
                            self.mount_function(&mut map, true);
                        },

                        _ => panic!("bad call of extern keyword")
                    }
                },
                
                VoidstarTokenTypes::Function => {
                    self.mount_function(&mut map, false);

                    // Don't take any scoped variables inside the function.
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