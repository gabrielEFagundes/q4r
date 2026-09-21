use std::collections::HashMap;

use quaoar_core::{backend::Backend, codegen::Codegen, expdesc::ExpType::self, internals::helpers, signature::{Signature, Type}, tokens::VoidstarTokenTypes};

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
                // on the C's compiler case, the asterisk will always mean it's a pointer
                // asterisks inside expressions are automatically consumed and considered multiplication.
                // VoidstarTokenTypes::Asterisk => {
                //     match helpers::lookahead(backend).token_type(){
                //         VoidstarTokenTypes::Int
                //         | VoidstarTokenTypes::Float
                //         | VoidstarTokenTypes::Bool
                //         | VoidstarTokenTypes::Char
                //         | VoidstarTokenTypes::Void => {
                //             backend.cursor += 1;
                //             self.parse_simple(Type::map(backend.tokens[backend.cursor].token_type()).to_byte_span());
                //             self.parse_simple(b"*");
                //             self.parse_simple();
                //         },
                //         _ => panic!("expected type for pbt pointers (pointer-before-type)")
                //     }
                //     backend.cursor += 1;
                // }

                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(backend);
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
                    self.parse_branch_expression(Self::var_assignment(backend));
                }

                VoidstarTokenTypes::Return => {
                    backend.cursor+=1;
                    
                    match Self::expression(backend){
                        ExpType::OperativeExp(exp_operator) => self.parse_operator_return(exp_operator),
                        ExpType::LiteralExp(exp_literal) => self.parse_literal_return(exp_literal),
                        ExpType::CallExp(exp_callee) => self.parse_fun_call(exp_callee),
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