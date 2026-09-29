use std::collections::HashMap;

use quaoar_core::{backend::Backend, codegen::Codegen, expdesc::{ExpType::{self}, StmtType}, signature::{Signature, Type}, tokens::VoidstarTokenTypes};
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
            ExpType::BinaryExp { left, operator, right } 
                => {
                    let exp_operator = ExpType::BinaryExp { left, operator, right };
                    self.parse_operator_expression(exp_operator)
                },
            ExpType::LiteralExp { val } 
                => {
                    let exp_literal = ExpType::LiteralExp { val };
                    self.parse_literal_expression(exp_literal)
                },
            ExpType::CallExp { ident, params }
                => {
                    let fun_call = ExpType::CallExp { ident, params };
                    self.parse_fun_call(fun_call)
                },
            ExpType::AddressExp { val }
                => {
                    let address_of = ExpType::LiteralExp { val };
                    self.parse_literal_expression(address_of)
                },
        }
    }

    pub(crate) fn parse_literal_expression(&mut self, expression: ExpType){
        // e.g. (literal)                               enhance this for everywhere it appears
        let ExpType::LiteralExp { val } = expression else {panic!()};
        
        if val.from_literal() == VoidstarTokenTypes::CharLiteral{
            emit!(&mut self.c_src, b'(', b'\'', val.to_byte_span().as_slice(), b'\'', b')');

        } else if val.from_literal() == VoidstarTokenTypes::StringLiteral{
            emit!(&mut self.c_src, b'(', b'"', val.to_byte_span().as_slice(), b'"', b')');

        } else {
            emit!(&mut self.c_src, b'(', val.to_byte_span().as_slice(), b')');
        }
    }

        pub(crate) fn parse_operator_expression(&mut self, expression: ExpType){
            // e.g. ((left)==(right)) or ((left)+(right))
            let ExpType::BinaryExp { left, operator, right } = expression else {panic!()};

            emit!(
                &mut self.c_src,
                b'(', b'('
            );

            self.parse_branch_expression(*left);

            emit!(&mut self.c_src, b')',
                operator.to_byte_span(),
                b'('
            );

            self.parse_branch_expression(*right);

            emit!(&mut self.c_src, b')', b')');
        }

    pub(crate) fn parse_conditional(&mut self, cmp_expression: StmtType, backend: &mut Backend<'a>){
        // e.g. if expression{} or while expression{}
        let StmtType::RelationalStmt { kind, expression } = cmp_expression else {panic!()};

        emit!(&mut self.c_src, kind.to_byte_span(), b' ');
        self.parse_branch_expression(expression);

        emit!(&mut self.c_src, b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub fn parse_for_loop(&mut self, for_declaration: StmtType, backend: &mut Backend<'a>){
        let StmtType::ForLoopStmt { 
            kind, 
            iterator, 
            initializer, 
            exp, 
            incrementer 
        } = for_declaration else {panic!()};

        emit!(
            &mut self.c_src, kind.to_byte_span(), b'(',
            &b"int"[..], b' ', iterator, b'='
        );
        self.parse_branch_expression(initializer);

        emit!(&mut self.c_src, b';');
        self.parse_branch_expression(exp);

        emit!(
            &mut self.c_src, b';', iterator, b'+', b'=', 
            incrementer.to_byte_span().as_slice(),
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

    pub(crate) fn parse_var_decl(&mut self, declaration: StmtType){
        // e.g. type var = value;
        let StmtType::VarStatement { ty, ident, val } = declaration else {panic!()};

        emit!(
            &mut self.c_src, 
            ty.to_byte_span().as_slice(), b' ', ident, b'='
        );
        self.parse_branch_expression(val);
        emit!(&mut self.c_src, b';');
    }

    pub(crate) fn parse_var_stmt(&mut self, identifier: &'a[u8], expression: ExpType<'a>){
        emit!(
            &mut self.c_src,
            identifier, b'='
        );
        self.parse_branch_expression(expression);
        emit!(&mut self.c_src, b';')
    }

    pub(crate) fn parse_fun_decl(&mut self, declaration: StmtType, backend: &mut Backend<'a>){
        // int function(int p1, int p2){ }
        let StmtType::FunStatement { 
            returns, 
            ident, 
            params, 
            is_extern 
        } = declaration else {panic!()};
        
        if is_extern{ self.parse_simple(b"extern "); }

        emit!(
            &mut self.c_src,
            returns.to_byte_span().as_slice(), b' ',
            ident, b'('
        );

        for i in 0..params.len(){
            let current = &params[i];

            if current.is_etc{
                emit!(&mut self.c_src, current.ident);

            } else {
                emit!(&mut self.c_src, 
                    current.ty.to_byte_span().as_slice(), b' ', current.ident
                );

                if i != params.len()-1{
                    emit!(&mut self.c_src, b',');
                }
            }
        }

        emit!(&mut self.c_src, b')', b'{');
        self.generate(backend);
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_fun_call(&mut self, call: ExpType<'a>){
        let ExpType::CallExp { ident, params } = call else {panic!()};

        emit!(
            &mut self.c_src, 
            b'(',
            ident, b'('
        );
        let params_len = params.len();
        for i in params{
            self.parse_branch_expression(i);
            self.parse_simple(b",");
        }
        // if there are parameters, remove the last trailing comma
        if(params_len > 0){
            self.c_src.pop();
        }

        emit!(&mut self.c_src, b')', b')', b';');
    }

    pub(crate) fn parse_return(&mut self, expression: ExpType<'a>){
        emit!(&mut self.c_src, &b"return"[..]);
        self.parse_branch_expression(expression);
        emit!(&mut self.c_src, b';');
    }
}