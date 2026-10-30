use ecow::EcoString;
use quac_core::{
    backend::Backend, codegen::Codegen, errors::error::QError, expdesc::{ExpType, StmtType}, internals::helpers, signature::Signature, tokens::VoidstarTokenTypes,
};
use rustc_hash::FxHashMap;

use crate::{TERMINATORS, emitter::CCompiler};

impl<'a> Codegen<'a> for CCompiler {
    fn generate_headers(&mut self, signatures: &FxHashMap<EcoString, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate(&mut self, backend: &mut Backend<'a>) -> (Vec<u8>, Vec<QError>) {
        while !helpers::end(backend) {
            match backend.tokens[backend.cursor].token_type() {
                VoidstarTokenTypes::Asterisk => {
                    helpers::advance(1, backend);
                    match helpers::current(backend).token_type() {
                        VoidstarTokenTypes::Int
                        | VoidstarTokenTypes::Float
                        | VoidstarTokenTypes::Bool
                        | VoidstarTokenTypes::Char
                        | VoidstarTokenTypes::Void => {
                            let dec = Self::var_statement(backend, true);
                            let StmtType::VarStmt(stmt) = dec else {
                                helpers::add_error("this is found to not be a variable declaration", backend);
                                continue;
                            };

                            self.emit_var_decl(stmt);
                        }

                        VoidstarTokenTypes::Ident => {
                            let next = helpers::lookahead(backend);
                            match next.token_type() {
                                VoidstarTokenTypes::OpenParents => {
                                    self.emit_branch_expression(Self::parse_expr(backend));
                                }

                                VoidstarTokenTypes::Equals => {
                                    self.emit_simple(b"*");
                                    self.emit_assign_stmt(Self::assign_statement(backend));
                                }

                                VoidstarTokenTypes::Increment
                                | VoidstarTokenTypes::Decrement
                                | VoidstarTokenTypes::Multiply
                                | VoidstarTokenTypes::Divide => {
                                    self.emit_op_assign_stmt(Self::assign_op_statement(backend));
                                }

                                _ => helpers::add_error("unexpected token found here", backend)
                            }
                        }
                        _ => panic!("bad usage of asterisk at the beggining of statement"),
                    }
                }

                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_statement(backend, false);
                    let StmtType::VarStmt(stmt) = dec else {
                        helpers::add_error("this is found to not be a variable declaration", backend);
                        continue;
                    };
                    self.emit_var_decl(stmt);
                }

                VoidstarTokenTypes::If => {
                    let cmp = Self::relational_statement(backend);
                    let StmtType::RelationalStmt(stmt) = cmp else {
                        helpers::add_error("this is found to not be an if condition", backend);
                        continue;
                    };
                    helpers::advance(1, backend);

                    self.emit_conditional(stmt, backend);
                }

                VoidstarTokenTypes::For => {
                    let loops = Self::loop_statement(backend);
                    match loops {
                        StmtType::LoopStmt(for_stmt) => self.emit_for_loop(for_stmt, backend),

                        StmtType::RelationalStmt(while_stmt) => self.emit_conditional(while_stmt, backend),

                        _ => helpers::add_error("unexpected token found here, check out the for loop syntax, both for and while loops use the 'for' keyword", backend)
                    }
                }

                VoidstarTokenTypes::Else => {
                    backend.cursor += 1;
                    let current = backend.tokens[backend.cursor];

                    if current.token_type() == VoidstarTokenTypes::If {
                        self.emit_simple(b"else ");
                        let cmp = Self::relational_statement(backend);

                        let StmtType::RelationalStmt(stmt) = cmp else {
                            helpers::add_error("this is found to not be an if condition", backend);
                            continue;
                        };
                        self.emit_conditional(stmt, backend);
                    } else {
                        self.emit_else(backend);
                    }
                }

                // on the C compiler's case, it's pointless to define the extern function twice, since it's
                // already defined when generating the table of signatures. That's why we skip it here.
                VoidstarTokenTypes::Extern => {
                    backend.cursor += 1;
                    if backend.tokens[backend.cursor].token_type() == VoidstarTokenTypes::OpenBraces
                    {
                        while backend.tokens[backend.cursor].token_type()
                            != VoidstarTokenTypes::CloseBraces
                        {
                            backend.cursor += 1;
                        }
                    } else {
                        while backend.tokens[backend.cursor].token_type()
                            != VoidstarTokenTypes::CloseParents
                        {
                            backend.cursor += 1;
                        }
                    }
                    backend.cursor += 1;
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(backend, false);
                    let StmtType::FunStmt(stmt) = fun else {
                        helpers::add_error("this is found to not be a function declaration", backend);
                        continue;
                    };

                    self.emit_fun_decl(stmt, backend);
                }

                VoidstarTokenTypes::Ident => {
                    let next = helpers::lookahead(backend);
                    match next.token_type() {
                        VoidstarTokenTypes::OpenParents => {
                            let ExpType::CallExp(exp) = Self::parse_expr(backend) else {
                                helpers::add_error("this is found not to be a function declaration", backend);
                                continue;
                            };

                            self.emit_fun_call(exp);
                            self.emit_simple(b";");
                        }

                        VoidstarTokenTypes::Equals => {
                            self.emit_assign_stmt(Self::assign_statement(backend));
                        }

                        VoidstarTokenTypes::Increment
                        | VoidstarTokenTypes::Decrement
                        | VoidstarTokenTypes::Multiply
                        | VoidstarTokenTypes::Divide => {
                            self.emit_op_assign_stmt(Self::assign_op_statement(backend));
                        }

                        _ => helpers::add_error("unexpected token found here", backend),
                    }
                }

                VoidstarTokenTypes::Return => {
                    helpers::advance(1, backend);

                    self.emit_return(Self::parse_expr(backend));

                    helpers::matches_any(&TERMINATORS, backend);
                }

                VoidstarTokenTypes::OpenBraces => { helpers::advance(1, backend); },
                VoidstarTokenTypes::CloseBraces => {
                    helpers::advance(1, backend);
                    break;
                }

                _ => { helpers::advance(1, backend); },
            }
        }

        (self.c_src.clone(), backend.errors.clone())
    }
}
