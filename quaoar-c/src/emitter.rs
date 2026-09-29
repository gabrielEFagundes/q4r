use std::collections::HashMap;

use quaoar_core::{backend::Backend, codegen::Codegen, error::{error::{self, QErrorTypes}, parser_err::ParserErrOpts}, expdesc::{ExpType::self, StmtType}, internals::helpers, signature::Signature, tokens::VoidstarTokenTypes};

use crate::{compiler::CCompiler};

impl<'a> Codegen<'a> for CCompiler{
    fn generate_headers(&mut self, signatures: &HashMap<String, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate(&mut self, backend: &mut Backend<'a>) -> Vec<u8> {
        while !helpers::end(backend){
            match backend.tokens[backend.cursor].token_type(){
                // asterisks located HERE on Q4r will always mean it's a pointer
                // expressions consume the multiplication asterisks as they parse.
                VoidstarTokenTypes::Asterisk => {
                    match helpers::lookahead(backend).token_type(){
                        VoidstarTokenTypes::Int
                        | VoidstarTokenTypes::Float
                        | VoidstarTokenTypes::Bool
                        | VoidstarTokenTypes::Char
                        | VoidstarTokenTypes::Void => {
                            let dec = Self::var_statement(backend, true);
                            helpers::expect(VoidstarTokenTypes::SemiColon, backend);
                            self.parse_var_decl(dec);
                        },

                        VoidstarTokenTypes::Ident => {
                            backend.cursor += 1;
                            let identifier = &backend.source[
                                backend.tokens[backend.cursor].start()..backend.tokens[backend.cursor].end()
                            ];

                            let mut v = Vec::from(b"*");
                            v.extend_from_slice(identifier);

                            self.parse_var_stmt(v.as_slice(), Self::assign_statement(backend));
                        },
                        _ => panic!("bad usage of asterisk at the beggining of statement")
                    }
                },

                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_statement(backend, false);
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If => {
                    let cmp = Self::relational_statement(backend);
                    backend.cursor+=1;

                    self.parse_conditional(cmp, backend);
                }

                VoidstarTokenTypes::For => {
                    let loops = Self::loop_statement(backend);
                    match loops {
                        StmtType::ForLoopStmt { kind, iterator, initializer, exp, incrementer } 
                            => self.parse_for_loop(loops, backend),

                        StmtType::RelationalStmt { kind, expression }
                            => self.parse_conditional(loops, backend)
                        
                        _ => error::QError::handle_new_error(
                            QErrorTypes::ParserErr(ParserErrOpts::UnexpectedToken), 
                            helpers::current(backend).line(), helpers::current(backend).start(), backend.debug
                        )
                    }
                }

                VoidstarTokenTypes::Else => {
                    backend.cursor += 1;
                    let current = backend.tokens[backend.cursor];

                    if current.token_type() == VoidstarTokenTypes::If{
                        self.parse_simple(b"else ");
                        let cmp = Self::relational_statement(backend);
                        self.parse_conditional(cmp, backend);
                    }else {
                        self.parse_else(backend);
                    }
                }

                // on the C compiler's case, it's pointless to define the extern function twice, since it's
                // already defined when generating the table of signatures. That's why we skip it here.
                VoidstarTokenTypes::Extern => {
                    backend.cursor += 1;
                    if backend.tokens[backend.cursor].token_type() == VoidstarTokenTypes::OpenBraces{
                        while backend.tokens[backend.cursor].token_type() != VoidstarTokenTypes::CloseBraces{
                            backend.cursor += 1;
                        }
                    } else {
                        while backend.tokens[backend.cursor].token_type() != VoidstarTokenTypes::CloseParents{
                            backend.cursor += 1;
                        }
                    }
                    backend.cursor += 1;
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(backend, false);
                    self.parse_fun_decl(fun, backend);
                }

                VoidstarTokenTypes::Ident => {
                    let identifier = &backend.source[
                        backend.tokens[backend.cursor].start()..backend.tokens[backend.cursor].end()
                    ];

                    let next = helpers::lookahead(backend);
                    match next.token_type(){
                        VoidstarTokenTypes::OpenParents => {
                            self.parse_branch_expression(Self::parse_expr(backend));
                        },

                        VoidstarTokenTypes::Equals
                        | VoidstarTokenTypes::Increment
                        | VoidstarTokenTypes::Decrement => {
                            self.parse_var_stmt(identifier, Self::assign_statement(backend));
                        }

                        _ => error::QError::handle_new_error(
                            QErrorTypes::ParserErr(ParserErrOpts::UnexpectedToken), 
                            helpers::current(backend).line(), helpers::current(backend).start(), backend.debug
                        )
                    }
                }

                VoidstarTokenTypes::Return => {
                    backend.cursor+=1;
                    
                    self.parse_return(Self::parse_expr(backend));

                    helpers::matches(VoidstarTokenTypes::SemiColon, backend);
                }

                VoidstarTokenTypes::OpenBraces => {
                    backend.cursor+=1
                },
                VoidstarTokenTypes::CloseBraces => {
                    backend.cursor+=1;
                    break;
                },

                _ => backend.cursor+=1
            }
        }

        self.c_src.clone()
    }
}