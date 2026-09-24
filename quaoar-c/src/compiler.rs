use std::collections::HashMap;

use quaoar_core::{backend::Backend, codegen::Codegen, expdesc::{ComparisonDeclaration, ExpLiteral, ExpOperator, ExpType::{self}, ForLoopDeclaration, FunCall, FunDeclaration, VarDeclaration}, signature::{Signature, Type}, tokens::VoidstarTokenTypes};
use crate::r#impl::AppendTo;

use crate::emit;

/// The `quaoar-c` compiler generates a source string 
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
pub struct CCompiler{   
    pub(crate) c_src: Vec<u8>,
}

#[allow(unused)]
impl<'a> CCompiler{
    pub fn new() -> Self{
        Self{ c_src: Vec::new() }
    }

    pub(crate) fn gen_signature_headers(&mut self, signatures: &HashMap<String, Signature>){
        for i in signatures{
            match i.1{
                Signature::Function { returns, params, is_extern } => {
                    if *is_extern{
                        self.parse_simple(b"extern ");
                    }

                    emit!(
                        &mut self.c_src, 
                        returns.to_byte_span().as_slice(), b' ', i.0.as_bytes(), b'('
                    );

                    for i in 0..params.len(){
                        if params[i].is_etc{
                            self.parse_simple(b"...");
                        } else {
                            emit!(&mut self.c_src, params[i].ty.to_byte_span().as_slice());

                            if i != params.len()-1{
                                emit!(&mut self.c_src, b',');
                            }
                        }
                    }

                    emit!(&mut self.c_src, b')', b';');
                },
                Signature::Global { ty, value:_ } => {
                    let fintype = if ty.eq(&Type::Bool){ 
                        &Type::Int
                    } else { &ty };

                    emit!(&mut self.c_src, fintype.to_byte_span().as_slice(), b' ', i.0.as_bytes(), b';');
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

    pub(crate) fn parse_decrement(&mut self, amount: &'a[u8]){
        emit!(&mut self.c_src, &b"-="[..], amount, b';');
    }

    pub(crate) fn parse_branch_expression(&mut self, expression: ExpType<'a>){
        match expression{
            ExpType::OperativeExp(exp_operator) => self.parse_operator_expression(exp_operator),
            ExpType::LiteralExp(exp_literal) => self.parse_literal_expression(exp_literal),
            ExpType::CallExp(fun_call) => self.parse_fun_call(fun_call),
            ExpType::AddressExp(exp_literal) => self.parse_literal_expression(exp_literal),
        }
    }

    pub(crate) fn parse_literal_expression(&mut self, expression: ExpLiteral){
        // e.g. (literal)
        if expression.val.from_literal() == VoidstarTokenTypes::CharLiteral{
            emit!(&mut self.c_src, b'(', b'\'', expression.val.to_byte_span().as_slice(), b'\'', b')');

        } else if expression.val.from_literal() == VoidstarTokenTypes::StringLiteral{
            emit!(&mut self.c_src, b'(', b'"', expression.val.to_byte_span().as_slice(), b'"', b')');

        } else {
            emit!(&mut self.c_src, b'(', expression.val.to_byte_span().as_slice(), b')');
        }
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
        self.parse_branch_expression(cmp_expression.expression);

        emit!(&mut self.c_src, b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub fn parse_for_loop(&mut self, for_declaration: ForLoopDeclaration, backend: &mut Backend<'a>){
        emit!(
            &mut self.c_src, for_declaration.kind.to_byte_span(), b'(',
            &b"int"[..], b' ', for_declaration.iterator, b'='
        );
        self.parse_branch_expression(for_declaration.initializer);

        emit!(&mut self.c_src, b';');
        self.parse_branch_expression(for_declaration.exp);

        emit!(
            &mut self.c_src, b';', for_declaration.iterator, b'+', b'=', 
            for_declaration.incrementer.to_byte_span().as_slice(),
            b')', b'{'
        );
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_else(&mut self, backend: &mut Backend<'a>){
        emit!(&mut self.c_src, &b"else"[..], b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_var_decl(&mut self, declaration: VarDeclaration){
        // e.g. type var = value;
        emit!(
            &mut self.c_src, 
            declaration.ty.to_byte_span().as_slice(), b' ', declaration.ident, b'='
        );
        self.parse_branch_expression(declaration.val);
        emit!(&mut self.c_src, b';');
    }

    pub(crate) fn parse_var_assign(&mut self, identifier: &'a[u8], expression: ExpType<'a>){
        emit!(
            &mut self.c_src,
            identifier, b'='
        );
        self.parse_branch_expression(expression);
        emit!(&mut self.c_src, b';')
    }

    pub(crate) fn parse_fun_decl(&mut self, declaration: FunDeclaration, backend: &mut Backend<'a>){
        // int function(int p1, int p2){ }
        if declaration.is_extern{ self.parse_simple(b"extern "); }

        emit!(
            &mut self.c_src,
            declaration.returns.to_byte_span().as_slice(), b' ',
            declaration.ident, b'('
        );

        for i in 0..declaration.params.len(){
            let current = &declaration.params[i];

            if current.is_etc{
                emit!(&mut self.c_src, current.ident);

            } else {
                emit!(&mut self.c_src, 
                    current.ty.to_byte_span().as_slice(), b' ', current.ident
                );

                if i != declaration.params.len()-1{
                    emit!(&mut self.c_src, b',');
                }
            }
        }

        emit!(&mut self.c_src, b')', b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_literal_return(&mut self, expression: ExpLiteral){
        emit!(&mut self.c_src, &b"return "[..], expression.val.to_byte_span().as_slice(), b';');
    }

    pub(crate) fn parse_operator_return(&mut self, expression: ExpOperator){
        emit!(
            &mut self.c_src, &b"return "[..],
            b'(', expression.left, b')',
            expression.operator.to_byte_span(), 
            b'(', expression.right, b')', b';'
        );
    }

    pub(crate) fn parse_fun_call(&mut self, call: FunCall<'a>){
        emit!(
            &mut self.c_src, 
            b'(',
            call.ident, b'('
        );
        let params_len = call.params.len();
        for i in call.params{
            self.parse_branch_expression(i);
            self.parse_simple(b",");
        }
        // if there are parameters, remove the last trailing comma
        if(params_len > 0){
            self.c_src.pop();
        }

        emit!(&mut self.c_src, b')', b')', b';');
    }
}