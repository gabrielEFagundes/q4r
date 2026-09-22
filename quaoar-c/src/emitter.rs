use std::collections::HashMap;

use quaoar_core::{backend::Backend, codegen::Codegen, expdesc::{ExpType::self, FunCall}, internals::helpers, signature::{Signature, Type}, tokens::VoidstarTokenTypes};

use crate::{compiler::CCompiler};

impl<'a> Codegen<'a> for CCompiler{
    fn generate_headers(&mut self, signatures: &HashMap<String, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate(&mut self, backend: &mut Backend<'a>) -> Vec<u8> {
        while !helpers::end(backend){
            //dbg!("{:#?}", backend.tokens[backend.cursor]);
            match backend.tokens[backend.cursor].token_type(){
                // asterisks located HERE on Q4r will always mean it's a pointer
                // expressions consume the multiplication asterisks anyway.
                VoidstarTokenTypes::Asterisk => {
                    match helpers::lookahead(backend).token_type(){
                        VoidstarTokenTypes::Int
                        | VoidstarTokenTypes::Float
                        | VoidstarTokenTypes::Bool
                        | VoidstarTokenTypes::Char
                        | VoidstarTokenTypes::Void => {
                            backend.cursor += 1;
                            let dec = Self::var_declaration(backend, true);
                            self.parse_var_decl(dec);
                        },

                        VoidstarTokenTypes::Ident => {
                            backend.cursor += 1;
                            let identifier = &backend.source[
                                backend.tokens[backend.cursor].start()..backend.tokens[backend.cursor].end()
                            ];

                            let mut v = Vec::from(b"*");
                            v.extend_from_slice(identifier);

                            self.parse_var_assign(v.as_slice(), Self::var_callee(backend));
                        },
                        _ => panic!("bad usage of asterisk at the beggining of statement")
                    }
                },

                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(backend, false);
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If => {
                    let cmp = Self::comparison_declaration(backend);
                    backend.cursor+=1;

                    self.parse_conditional(cmp, backend);
                }

                VoidstarTokenTypes::For => {
                    let loops = Self::loop_declaration(backend);
                    match loops {
                        quaoar_core::expdesc::DeclType::ForLoopDecl(for_loop) 
                            => self.parse_for_loop(for_loop, backend),

                        quaoar_core::expdesc::DeclType::WhileLoopDecl(while_loop) 
                            => {
                                self.parse_conditional(while_loop, backend)
                            },
                    }
                }

                VoidstarTokenTypes::Else => {
                    backend.cursor += 1;
                    let current = backend.tokens[backend.cursor];

                    if current.token_type() == VoidstarTokenTypes::If{
                        self.parse_simple(b"else ");
                        let cmp = Self::comparison_declaration(backend);
                        self.parse_conditional(cmp, backend);
                    }else {
                        self.parse_else(backend);
                    }
                }

                VoidstarTokenTypes::Plus | VoidstarTokenTypes::Minus => {
                    todo!("number signment (plus or minus) yet to be implemented");
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(backend);
                    self.parse_fun_decl(fun, backend);
                }

                VoidstarTokenTypes::Ident => {
                    let identifier = &backend.source[
                        backend.tokens[backend.cursor].start()..backend.tokens[backend.cursor].end()
                    ];

                    let callee = Self::var_callee(backend);
                    match callee{
                        ExpType::OperativeExp(_) => self.parse_var_assign(identifier, callee),
                        ExpType::LiteralExp(_) => self.parse_var_assign(identifier, callee),
                        ExpType::CallExp(fun_call) => self.parse_fun_call(fun_call),
                        ExpType::AddressExp(_) => self.parse_var_assign(identifier, callee),
                    }
                }

                VoidstarTokenTypes::Return => {
                    backend.cursor+=1;
                    
                    match Self::expression(backend){
                        ExpType::OperativeExp(exp_operator) => self.parse_operator_return(exp_operator),
                        ExpType::LiteralExp(exp_literal) => self.parse_literal_return(exp_literal),
                        ExpType::CallExp(exp_callee) => self.parse_fun_call(exp_callee),
                        ExpType::AddressExp(exp_literal) => self.parse_literal_expression(exp_literal),
                    };
                }

                VoidstarTokenTypes::OpenBraces => {
                    backend.cursor+=1
                },
                VoidstarTokenTypes::CloseBraces => {
                    backend.cursor+=1;
                    break
                },

                _ => backend.cursor+=1
            }
        }

        self.c_src.clone()
    }
}