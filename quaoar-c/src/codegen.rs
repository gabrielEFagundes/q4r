use std::collections::HashMap;

use quaoar_core::{emitter::CodeGen, expdesc::{ExpLiteral, ExpType::{self, LiteralExp}}, signature::{Literal, Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes}};

use crate::{compiler::CCompiler, emit};

impl CodeGen for CCompiler{
    fn generate_headers(&mut self, signatures: HashMap<String, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate<'a>(&mut self, tokens: &'a[VoidstarToken], source: &'a[u8], cursor: &mut usize) -> Vec<u8> {
        while !Self::end(tokens, cursor){
            match tokens[*cursor].token_type(){
                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(tokens, source, cursor);
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If | VoidstarTokenTypes::While => {
                    let cmp = Self::comparison_declaration(tokens, source, cursor);
                    self.parse_conditional(cmp, tokens, source, cursor);
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(tokens, source, cursor);
                    self.parse_fun_decl(fun, tokens, source, cursor);
                }

                VoidstarTokenTypes::Ident => {
                    // emit ident and check either it's a function call or a variable attribution/increment/decrement
                    self.parse_simple(&source[tokens[*cursor].start()..tokens[*cursor].end()]);
                    *cursor += 1;

                    match tokens[*cursor].token_type(){
                        // func call
                        VoidstarTokenTypes::OpenParents => {
                            let mut params: Vec<u8> = Vec::new();
                            *cursor += 1;

                            while tokens[*cursor].token_type() != VoidstarTokenTypes::CloseParents{
                                    let current = tokens[*cursor];
                                    params.extend_from_slice(&source[current.start()..current.end()]);
                                    *cursor += 1;
                            }

                            self.parse_fun_call(params);
                        },

                        // var assignment
                        VoidstarTokenTypes::Equals => {
                            *cursor += 1;
                            self.parse_simple(b"=");
                            
                            match Self::expression(tokens, source, cursor){
                                ExpType::OperativeExp(exp_operator) => self.parse_operator_expression(exp_operator),
                                ExpType::LiteralExp(exp_literal) => self.parse_literal_expression(exp_literal),
                            }
                            self.parse_simple(b";");
                        },

                        VoidstarTokenTypes::Increment | VoidstarTokenTypes::Decrement => {
                            *cursor += 1;

                            let current = tokens[*cursor];
                            self.parse_increment(&source[current.start()..current.end()]);
                        }

                        _ => panic!()
                    }
                }

                VoidstarTokenTypes::Return => {
                    *cursor+=1;
                    
                    match Self::expression(tokens, source, cursor){
                        ExpType::OperativeExp(exp_operator) => self.parse_operator_return(exp_operator),
                        ExpType::LiteralExp(exp_literal) => self.parse_literal_return(exp_literal),
                    };
                }

                VoidstarTokenTypes::OpenBraces => {
                    //self.enter_scope();
                    *cursor+=1
                },
                VoidstarTokenTypes::CloseBraces => {
                    //self.exit_scope();
                    *cursor+=1;
                    break
                },

                _ => *cursor+=1
            }
        }

        self.c_src.clone()
    }
}