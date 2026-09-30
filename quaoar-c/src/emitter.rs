use std::collections::HashMap;

use quaoar_core::{
    backend::Backend, codegen::Codegen, diagnosis::error::QError, expdesc::StmtType, internals::helpers, signature::Signature, tokens::VoidstarTokenTypes,
};

use crate::compiler::CCompiler;

impl<'a> Codegen<'a> for CCompiler {
    fn generate_headers(&mut self, signatures: &HashMap<String, Signature>) -> &mut Self {
        self.gen_signature_headers(signatures);
        self
    }

    fn generate(&mut self, backend: &mut Backend<'a>) -> Vec<u8> {
        while !helpers::end(backend) {
            match backend.tokens[backend.cursor].token_type() {
                // asterisks located HERE on Q4r will always mean it's a pointer
                // expressions consume the multiplication asterisks as they parse.
                VoidstarTokenTypes::Asterisk => {
                    helpers::advance(1, backend);
                    match helpers::current(backend).token_type() {
                        VoidstarTokenTypes::Int
                        | VoidstarTokenTypes::Float
                        | VoidstarTokenTypes::Bool
                        | VoidstarTokenTypes::Char
                        | VoidstarTokenTypes::Void => {
                            let dec = Self::var_statement(backend, true);
                            self.emit_var_decl(dec);
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

                                _ => QError::evaluate_new_err(
                                    format_args!("unexpected token found on line {}:{}", helpers::current(backend).line(), helpers::current(backend).start()), backend.debug
                                )
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
                    self.emit_var_decl(dec);
                }

                VoidstarTokenTypes::If => {
                    let cmp = Self::relational_statement(backend);
                    backend.cursor += 1;

                    self.emit_conditional(cmp, backend);
                }

                VoidstarTokenTypes::For => {
                    let loops = Self::loop_statement(backend);
                    match loops {
                        StmtType::LoopStmt(_) => self.emit_for_loop(loops, backend),

                        StmtType::RelationalStmt(_)
                            => self.emit_conditional(loops, backend),
                        
                        _ => QError::evaluate_new_err(
                            format_args!("unexpected token found on line {}:{}", helpers::current(backend).line(), helpers::current(backend).start()), backend.debug
                        )
                    }
                }

                VoidstarTokenTypes::Else => {
                    backend.cursor += 1;
                    let current = backend.tokens[backend.cursor];

                    if current.token_type() == VoidstarTokenTypes::If {
                        self.emit_simple(b"else ");
                        let cmp = Self::relational_statement(backend);
                        self.emit_conditional(cmp, backend);
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
                    self.emit_fun_decl(fun, backend);
                }

                VoidstarTokenTypes::Ident => {
                    let next = helpers::lookahead(backend);
                    match next.token_type() {
                        VoidstarTokenTypes::OpenParents => {
                            self.emit_fun_call(Self::parse_expr(backend));
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

                        _ => QError::evaluate_new_err(
                            format_args!("unexpected token found on line {}:{}", helpers::current(backend).line(), helpers::current(backend).start()), backend.debug
                        )
                    }
                }

                VoidstarTokenTypes::Return => {
                    backend.cursor += 1;

                    self.emit_return(Self::parse_expr(backend));

                    helpers::matches(VoidstarTokenTypes::SemiColon, backend);
                }

                VoidstarTokenTypes::OpenBraces => backend.cursor += 1,
                VoidstarTokenTypes::CloseBraces => {
                    backend.cursor += 1;
                    break;
                }

                _ => backend.cursor += 1,
            }
        }

        self.c_src.clone()
    }
}
