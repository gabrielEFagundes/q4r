use quaoar_core::{expdesc::{ExpLiteral, ExpType::{self, LiteralExp}}, generator::CodeGen, tokens::VoidstarTokenTypes};

use crate::{compiler::CCompiler, emit};

impl<'a> CodeGen for CCompiler<'_>{
    fn generate_headers(&mut self) -> &mut Self {
        self.gen_signature_headers();
        self
    }

    fn generate(&mut self) -> Vec<u8> {
        while !Self::end(self.tokens, &mut self.cursor){
            match self.tokens[self.cursor].token_type(){
                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If | VoidstarTokenTypes::While => {
                    let cmp = Self::comparison_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_conditional(cmp);
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_fun_decl(fun);
                }

                VoidstarTokenTypes::Ident => {
                    // emit ident and check either it's a function call or a variable attribution/increment/decrement
                    self.parse_simple(&self.source[self.tokens[self.cursor].start()..self.tokens[self.cursor].end()]);
                    self.cursor += 1;

                    match self.tokens[self.cursor].token_type(){
                        // func call
                        VoidstarTokenTypes::OpenParents => {
                            let mut params: Vec<u8> = Vec::new();
                            self.cursor += 1;

                            while self.tokens[self.cursor].token_type() != VoidstarTokenTypes::CloseParents{
                                    let current = self.tokens[self.cursor];
                                    params.extend_from_slice(&self.source[current.start()..current.end()]);
                                    self.cursor += 1;
                            }

                            self.parse_fun_call(params);
                        },

                        // var assignment
                        VoidstarTokenTypes::Equals => {
                            self.cursor += 1;

                            let current = self.tokens[self.cursor];
                            let val = &self.source[current.start()..current.end()];
                            self.parse_literal_expression(ExpLiteral{ val });
                        },

                        // decrement/increment

                        _ => panic!()
                    }
                }

                VoidstarTokenTypes::Return => {
                    self.cursor+=1;
                    
                    match Self::expression(self.tokens, self.source, &mut self.cursor){
                        ExpType::OperativeExp(exp_operator) => self.parse_operator_return(exp_operator),
                        ExpType::LiteralExp(exp_literal) => self.parse_literal_return(exp_literal),
                    };
                }

                VoidstarTokenTypes::OpenBraces => {
                    self.enter_scope();
                    self.cursor+=1
                },
                VoidstarTokenTypes::CloseBraces => {
                    self.exit_scope();
                    self.cursor+=1;
                    break
                },

                _ => self.cursor+=1
            }
        }

        self.c_src.clone()
    }
}