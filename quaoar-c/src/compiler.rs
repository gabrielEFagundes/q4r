use std::collections::HashMap;

use quaoar_core::{backend::Backend, emitter::Codegen, expdesc::{ComparisonDeclaration, ExpLiteral, ExpOperator, ExpType::{self, LiteralExp}, ExpVariable, ForLoopDeclaration, FunDeclaration, VarDeclaration}, signature::{self, Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes::{self, Function}}};
use crate::r#impl::AppendTo;

use crate::emit;

/// The `quaoar-c` transpiler generates a source string 
/// and parses it to the first gcc compiler Q4r finds on PATH.
/// 
/// `gcc -x c -o <prog_name> <<< '<code_str>'`
/// 
/// The command adapts to each available compiler at the time.
/// 
/// ## Available Compilers
/// Currently, Q4r supports the following C compilers:
/// - zig cc
/// - gcc
/// 
/// Those are the ones that were tested.
pub struct CCompiler{   
    pub(crate) c_src: Vec<u8>,
}

impl<'a> CCompiler{
    pub fn new() -> Self{
        Self{ c_src: Vec::new() }
    }

    pub(crate) fn gen_signature_headers(&mut self, signatures: &HashMap<String, Signature>){
        for i in signatures{
            match i.1{
                Signature::Function { returns, params } => {
                    let finreturns = if returns.eq(&Type::Bool){
                        &Type::Int
                    } else { &returns };

                    emit!(
                        &mut self.c_src, 
                        finreturns.to_byte_span(), b' ', i.0.as_bytes(), b'('
                    );

                    for i in 0..params.len(){
                        emit!(&mut self.c_src, params[i].to_byte_span());
                        if i != params.len()-1{
                            emit!(&mut self.c_src, b',');
                        }
                    }

                    emit!(&mut self.c_src, b')', b';');
                },
                Signature::Global { ty, value:_ } => {
                    let fintype = if ty.eq(&Type::Bool){ 
                        &Type::Int
                    } else { &ty };

                    emit!(&mut self.c_src, fintype.to_byte_span(), b' ', i.0.as_bytes(), b';');
                },
            }
        }
    }

    pub(crate) fn parse_simple(&mut self, to_emit: &'a[u8]){
        emit!(&mut self.c_src, to_emit);
    }

    pub(crate) fn parse_increment(&mut self, amount: &'a[u8]){
        emit!(&mut self.c_src, &b"+="[..], amount, b';');
    }

    pub(crate) fn parse_assignment_expression(&mut self, expression: ExpLiteral){
        // e.g. ident = expression
        emit!(&mut self.c_src, b'=', expression.val.to_byte_span().as_slice(), b';');
    }

    pub(crate) fn parse_literal_expression(&mut self, expression: ExpLiteral){
        // e.g. (literal)
        emit!(&mut self.c_src, b'(', expression.val.to_byte_span().as_slice(), b')');
    }

    pub(crate) fn parse_operator_expression(&mut self, expression: ExpOperator){
        // e.g. ((left)==(right)) or ((left)+(right))
        emit!(
            &mut self.c_src,
            b'(', b'(', expression.left, b')',
            expression.operator.to_byte_span(),
            b'(', expression.right, b')', b')'
        );
    }

    pub(crate) fn parse_conditional(&mut self, cmp_expression: ComparisonDeclaration, backend: &mut Backend<'a>){
        // e.g. if expression{} or while expression{}
        emit!(&mut self.c_src, cmp_expression.kind.to_byte_span(), b' ');
        if let ExpType::OperativeExp(ExpOperator { left, operator, right }) = cmp_expression.expression{
            self.parse_operator_expression(ExpOperator { left, operator, right });

        } else if let ExpType::LiteralExp(ExpLiteral { val }) = cmp_expression.expression{
            self.parse_literal_expression(ExpLiteral { val });
        }

        emit!(&mut self.c_src, b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub fn parse_for_loop(&mut self, for_declaration: ForLoopDeclaration, backend: &mut Backend<'a>){
        emit!(
            &mut self.c_src, for_declaration.kind.to_byte_span(), b'(',
            &b"int"[..], b' ', for_declaration.iterator, b';',
            // to finish lmao
        )
    }

    pub(crate) fn parse_else(&mut self, backend: &mut Backend<'a>){
        emit!(&mut self.c_src, &b"else"[..], b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_var_decl(&mut self, declaration: VarDeclaration){
        // int var = value;
        let fintype = if declaration.ty.eq(&Type::Bool){
            &Type::Int
        } else { &declaration.ty };

        emit!(
            &mut self.c_src, 
            fintype.to_byte_span(), b' ', declaration.ident, b'='
        );
        if let ExpType::LiteralExp(ExpLiteral{ val }) = declaration.val{
            emit!(&mut self.c_src, val.to_byte_span().as_slice(), b';')
        }
    }

    pub(crate) fn parse_fun_decl(&mut self, declaration: FunDeclaration, backend: &mut Backend<'a>){
        // int function(int p1, int p2){ }
        let finreturns = if declaration.returns.eq(&Type::Bool){
            &Type::Int
        } else { &declaration.returns };

        emit!(
            &mut self.c_src,
            finreturns.to_byte_span(), b' ',
            declaration.ident, b'('
        );

        for i in 0..declaration.params.len(){
            let current = &declaration.params[i];
            // default arguments to be implemented in the future
            // they're currently ignored on the C compiler's case
            emit!(&mut self.c_src, 
                current.ty.to_byte_span(), b' ', current.ident
            );

            if i != declaration.params.len()-1{
                emit!(&mut self.c_src, b',');
            }
        }
        emit!(&mut self.c_src, b')', b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_literal_return(&mut self, expression: ExpLiteral){
        emit!(&mut self.c_src, &b"return"[..], b' ', expression.val.to_byte_span().as_slice(), b';');
    }

    pub(crate) fn parse_operator_return(&mut self, expression: ExpOperator){
        emit!(
            &mut self.c_src, &b"return"[..], b' ',
            b'(', expression.left, b')',
            expression.operator.to_byte_span(), 
            b'(', expression.right, b')', b';'
        );
    }

    pub(crate) fn parse_fun_call(&mut self, params: Vec<u8>){
        emit!(
            &mut self.c_src, b'(',
            params.as_slice(), b')', b';'
        );
    }
}