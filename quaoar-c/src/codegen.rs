use std::collections::HashMap;

use quaoar_core::{backend::Backend, emitter::Codegen, expdesc::{ExpLiteral, ExpType::{self, LiteralExp}}, signature::{Literal, Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes}};

use crate::{compiler::CCompiler};

impl<'a> Codegen<'a, CCompiler> for CCompiler{
    fn generate_headers(&mut self, signatures: &HashMap<String, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate(&mut self, backend: &mut Backend<'a>) -> Vec<u8> {
        while !Self::end(backend){
            match backend.tokens[backend.cursor].token_type(){
                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(backend);
                    backend.declare_in_scope(str::from_utf8(dec.ident).unwrap().to_string(), dec.ty.clone());
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If | VoidstarTokenTypes::While => {
                    let cmp = Self::comparison_declaration(backend);
                    self.parse_conditional(cmp, backend);
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(backend);
                    self.parse_fun_decl(fun, backend);
                }

                VoidstarTokenTypes::Ident => {
                    // emit ident and check either it's a function call or a variable attribution/increment/decrement
                    let identifier = backend.tokens[backend.cursor];
                    self.parse_simple(&backend.source[identifier.start()..identifier.end()]);
                    
                    backend.cursor += 1;

                    match backend.tokens[backend.cursor].token_type(){
                        // func call
                        VoidstarTokenTypes::OpenParents => {
                            let mut params: Vec<u8> = Vec::new();
                            backend.cursor += 1;

                            while backend.tokens[backend.cursor].token_type() != VoidstarTokenTypes::CloseParents{
                                    let current = backend.tokens[backend.cursor];
                                    params.extend_from_slice(&backend.source[current.start()..current.end()]);
                                    backend.cursor += 1;
                            }

                            self.parse_fun_call(params);
                        },

                        // var assignment
                        VoidstarTokenTypes::Equals => {
                            backend.cursor += 1;
                            
                            match Self::expression(backend){
                                ExpType::OperativeExp(exp_operator) => self.parse_operator_expression(exp_operator),
                                ExpType::LiteralExp(exp_literal) => self.parse_literal_expression(exp_literal),
                                ExpType::VariableExp(exp_var) => self.parse_var_expression(exp_var),
                            }
                            backend.cursor += 1;
                        },

                        VoidstarTokenTypes::Increment | VoidstarTokenTypes::Decrement => {
                            backend.cursor += 1;

                            let current = backend.tokens[backend.cursor];
                            self.parse_increment(&backend.source[current.start()..current.end()]);
                        }

                        _ => panic!("invalid identifier call `{:#?}`", backend.tokens[backend.cursor].token_type())
                    }
                }

                VoidstarTokenTypes::Return => {
                    backend.cursor+=1;
                    
                    match Self::expression(backend){
                        ExpType::OperativeExp(exp_operator) => self.parse_operator_return(exp_operator),
                        ExpType::LiteralExp(exp_literal) => self.parse_literal_return(exp_literal),
                        ExpType::VariableExp(exp_var) => self.parse_var_expression(exp_var),
                    };
                }

                VoidstarTokenTypes::OpenBraces => {
                    backend.enter_scope();
                    backend.cursor+=1
                },
                VoidstarTokenTypes::CloseBraces => {
                    backend.exit_scope();
                    backend.cursor+=1;
                    break
                },

                _ => backend.cursor+=1
            }
        }

        self.c_src.clone()
    }
}