use std::collections::HashMap;

use crate::r#impl::AppendTo;
use quaoar_core::{
    backend::Backend,
    codegen::Codegen,
    expdesc::{
        AssignOpStmt, AssignStmt,
        ExpType::{self},
        StmtType,
    },
    signature::{Signature, Type},
    tokens::VoidstarTokenTypes,
};

use crate::emit;

/// The `quaoar-c` compiler generates a source bytespan
/// and parses it to the first gcc compiler Q4r finds on PATH.
///
/// `gcc -x c -o <prog_name> <<< '<code_str>'`
///
/// Quac adapts to each available compiler at the time.
///
/// ## Available Compilers
/// Currently, Q4r supports the following C compilers:
/// - zig cc
/// - gcc
///
/// Those are the ones that were tested.
pub struct CCompiler {
    pub(crate) c_src: Vec<u8>,
}

#[allow(unused)]
impl Default for CCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CCompiler {
    pub fn new() -> Self {
        Self { c_src: Vec::new() }
    }

    pub(crate) fn gen_signature_headers(&mut self, signatures: &HashMap<String, Signature>) {
        for i in signatures {
            match i.1 {
                Signature::Function {
                    returns,
                    params,
                    is_extern,
                } => {
                    if *is_extern {
                        self.emit_simple(b"extern ");
                    }

                    emit!(
                        &mut self.c_src,
                        returns.to_byte_span().as_slice(),
                        b' ',
                        i.0.as_bytes(),
                        b'('
                    );

                    for i in 0..params.len() {
                        if params[i].is_etc {
                            self.emit_simple(b"...");
                        } else {
                            emit!(&mut self.c_src, params[i].ty.to_byte_span().as_slice());

                            if i != params.len() - 1 {
                                emit!(&mut self.c_src, b',');
                            }
                        }
                    }

                    emit!(&mut self.c_src, b')', b';');
                }
                Signature::Global { ty, value: _ } => {
                    let fintype = if ty.eq(&Type::Bool) { &Type::Int } else { ty };

                    emit!(
                        &mut self.c_src,
                        fintype.to_byte_span().as_slice(),
                        b' ',
                        i.0.as_bytes(),
                        b';'
                    );
                }
            }
        }
    }

    #[inline]
    pub(crate) fn emit_simple(&mut self, to_emit: &'a [u8]) {
        emit!(&mut self.c_src, to_emit);
    }

    pub(crate) fn emit_branch_expression(&mut self, expression: ExpType<'a>) {
        match expression {
            ExpType::BinaryExp {
                left,
                operator,
                right,
            } => {
                let exp_operator = ExpType::BinaryExp {
                    left,
                    operator,
                    right,
                };
                self.emit_operator_expression(exp_operator)
            }
            ExpType::LiteralExp { val } => {
                let exp_literal = ExpType::LiteralExp { val };
                self.emit_literal_expression(exp_literal)
            }
            ExpType::CallExp { ident, params } => {
                let fun_call = ExpType::CallExp { ident, params };
                self.emit_fun_call(fun_call)
            }
            ExpType::AddressExp { val } => {
                emit!(&mut self.c_src, b'&');
                self.emit_branch_expression(*val);
            }
            ExpType::SignedExp { op, val } => {
                emit!(&mut self.c_src, op.to_byte_span());
                self.emit_branch_expression(*val);
            }
        }
    }

    pub(crate) fn emit_literal_expression(&mut self, expression: ExpType) {
        // e.g. (literal)                               enhance this for everywhere it appears
        let ExpType::LiteralExp { val } = expression else {
            panic!()
        };

        if val.from_literal() == VoidstarTokenTypes::CharLiteral {
            emit!(
                &mut self.c_src,
                b'(',
                b'\'',
                val.to_byte_span().as_slice(),
                b'\'',
                b')'
            );
        } else if val.from_literal() == VoidstarTokenTypes::StringLiteral {
            emit!(
                &mut self.c_src,
                b'(',
                b'"',
                val.to_byte_span().as_slice(),
                b'"',
                b')'
            );
        } else {
            emit!(&mut self.c_src, b'(', val.to_byte_span().as_slice(), b')');
        }
    }

    pub(crate) fn emit_operator_expression(&mut self, expression: ExpType) {
        // e.g. ((left)==(right)) or ((left)+(right))
        let ExpType::BinaryExp {
            left,
            operator,
            right,
        } = expression
        else {
            panic!()
        };

        emit!(&mut self.c_src, b'(', b'(');

        self.emit_branch_expression(*left);

        emit!(&mut self.c_src, b')', operator.to_byte_span(), b'(');

        self.emit_branch_expression(*right);

        emit!(&mut self.c_src, b')', b')');
    }

    pub(crate) fn emit_conditional(&mut self, cmp_expression: StmtType, backend: &mut Backend<'a>) {
        // e.g. if expression{} or while expression{}
        let StmtType::RelationalStmt(stmt) = cmp_expression else {
            panic!()
        };

        emit!(&mut self.c_src, stmt.kind.to_byte_span(), b' ');
        self.emit_branch_expression(stmt.exp);

        emit!(&mut self.c_src, b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub fn emit_for_loop(&mut self, for_declaration: StmtType, backend: &mut Backend<'a>) {
        let StmtType::LoopStmt(stmt) = for_declaration else {
            panic!()
        };

        emit!(
            &mut self.c_src,
            stmt.kind.to_byte_span(),
            b'(',
            &b"int "[..]
        );

        let identifier = stmt.initializer.ident;
        self.emit_assign_stmt(stmt.initializer);
        self.emit_branch_expression(stmt.exp);

        emit!(
            &mut self.c_src,
            b';',
            identifier,
            b'+',
            b'=',
            stmt.incrementer.to_byte_span().as_slice(),
            b')',
            b'{'
        );
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn emit_else(&mut self, backend: &mut Backend<'a>) {
        emit!(&mut self.c_src, &b"else"[..], b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn emit_var_decl(&mut self, declaration: StmtType) {
        // e.g. type var = value;
        let StmtType::VarStmt(stmt) = declaration else {
            panic!()
        };

        emit!(
            &mut self.c_src,
            stmt.ty.to_byte_span().as_slice(),
            b' ',
            stmt.ident,
            b'='
        );
        self.emit_branch_expression(stmt.val);
        emit!(&mut self.c_src, b';');
    }

    pub(crate) fn emit_assign_stmt(&mut self, statement: AssignStmt<'a>) {
        emit!(&mut self.c_src, statement.ident, b'=');
        self.emit_branch_expression(statement.val);
        emit!(&mut self.c_src, b';')
    }

    pub(crate) fn emit_op_assign_stmt(&mut self, statement: AssignOpStmt<'a>) {
        emit!(
            &mut self.c_src,
            statement.ident,
            statement.op.to_byte_span()
        );
        self.emit_branch_expression(statement.val);
        emit!(&mut self.c_src, b';')
    }

    pub(crate) fn emit_fun_decl(&mut self, declaration: StmtType, backend: &mut Backend<'a>) {
        // int function(int p1, int p2){ }
        let StmtType::FunStmt(stmt) = declaration else {
            panic!()
        };

        if stmt.is_extern {
            self.emit_simple(b"extern ");
        }

        emit!(
            &mut self.c_src,
            stmt.returns.to_byte_span().as_slice(),
            b' ',
            stmt.ident,
            b'('
        );

        for i in 0..stmt.params.len() {
            let current = &stmt.params[i];

            if current.is_etc {
                emit!(&mut self.c_src, current.ident);
            } else {
                emit!(
                    &mut self.c_src,
                    current.ty.to_byte_span().as_slice(),
                    b' ',
                    current.ident
                );

                if i != stmt.params.len() - 1 {
                    emit!(&mut self.c_src, b',');
                }
            }
        }

        emit!(&mut self.c_src, b')', b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn emit_fun_call(&mut self, call: ExpType<'a>) {
        let ExpType::CallExp { ident, params } = call else {
            panic!()
        };

        emit!(&mut self.c_src, b'(', ident, b'(');
        let params_len = params.len();
        for i in params {
            self.emit_branch_expression(i);
            self.emit_simple(b",");
        }
        // if there are parameters, remove the last trailing comma
        if params_len > 0 {
            self.c_src.pop();
        }

        emit!(&mut self.c_src, b')', b')');
    }

    pub(crate) fn emit_return(&mut self, expression: ExpType<'a>) {
        emit!(&mut self.c_src, &b"return"[..]);
        self.emit_branch_expression(expression);
        emit!(&mut self.c_src, b';');
    }
}
