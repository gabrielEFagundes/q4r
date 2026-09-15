use std::collections::HashMap;

use quaoar_core::{expdesc::{ComparisonDeclaration, ExpLiteral, ExpOperator, ExpType::{self, LiteralExp}, FunDeclaration, VarDeclaration}, generator::CodeGen, signatures::{self, Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes::{self, Function}}};
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
pub struct CCompiler<'a>{
    pub(crate) tokens: &'a[VoidstarToken],
    pub(crate) source: &'a[u8],
    pub(crate) cursor: usize,

    signatures: HashMap<String, Signature>,
    
    pub(crate) c_src: Vec<u8>,
    scopes: Vec<HashMap<String, Type>> // only variables (name and type, helps the typechecker)
}

impl<'a> CCompiler<'a>{
    pub fn new(source: &'a[u8], tokens: &'a[VoidstarToken], signatures: HashMap<String, Signature>) -> Self{
        Self{ tokens, source, cursor: 0, signatures, c_src: Vec::new(), scopes: Vec::new() }
    }

    pub fn enter_scope(&mut self){
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self){
        self.scopes.pop();
    }

    pub fn declare_in_scope(&mut self, ident: String, ty: Type){
        self.scopes.last_mut().unwrap().insert(ident, ty);
    }

    pub fn resolve_in_scope(&mut self, ident: &str) -> Option<&Type>{
        self.scopes.iter().rev()
            .find_map(|s| s.get(ident))
            .or_else(|| {
                let sign = self.signatures.get(ident).unwrap();
                Some(&Signature::destructure_glob(sign).0).map(|v| &**v)
            })
    }

    pub(crate) fn gen_signature_headers(&mut self){
        for i in &self.signatures{
            println!("{:#?}", i);
            match i.1{
                Signature::Function { returns, params } => {
                    let finreturns = if returns.eq(&Type::Bool){
                        &Type::Int
                    } else { returns };

                    emit!(
                        &mut self.c_src, 
                        finreturns.to_byte_span(), b' ', i.0.as_bytes(), b'('
                    );

                    for i in params{
                        emit!(&mut self.c_src, i.to_byte_span());
                    }

                    emit!(&mut self.c_src, b')', b';');
                },
                Signature::Global { ty, value:_ } => {
                    let fintype = if ty.eq(&Type::Bool){ 
                        &Type::Int
                    } else { ty };

                    emit!(&mut self.c_src, fintype.to_byte_span(), b' ', i.0.as_bytes(), b';');
                },
            }
        }
    }

    pub(crate) fn parse_simple(&mut self, to_emit: &'a[u8]){
        emit!(&mut self.c_src, to_emit);
    }

    pub(crate) fn parse_literal_expression(&mut self, expression: ExpLiteral){
        emit!(&mut self.c_src, b'=', expression.val, b';');
    }

    pub(crate) fn parse_operator_expression(&mut self, expression: ExpOperator){
        // e.g. ((left)==(right))
        emit!(
            &mut self.c_src,
            b'(', b'(', expression.left, b')',
            expression.operator.to_byte_span(),
            b'(', expression.right, b')', b')'
        );
    }

    pub(crate) fn parse_conditional(&mut self, cmp_expression: ComparisonDeclaration){
        // e.g. if expression{} or while expression{}
        emit!(&mut self.c_src, cmp_expression.kind.to_byte_span(), b' ');
        if let ExpType::OperativeExp(ExpOperator { left, operator, right }) = cmp_expression.expression{
            self.parse_operator_expression(ExpOperator { left, operator, right });
        }

        emit!(&mut self.c_src, b'{');
        self.generate();
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
            emit!(&mut self.c_src, val, b';')
        }
    }

    pub(crate) fn parse_fun_decl(&mut self, declaration: FunDeclaration){
        // int function(int p1, int p2){ }
        let finreturns = if declaration.returns.eq(&Type::Bool){
            &Type::Int
        } else { &declaration.returns };

        emit!(
            &mut self.c_src,
            finreturns.to_byte_span(), b' ',
            declaration.ident, b'('
        );
        for i in declaration.params{
            emit!(&mut self.c_src, i);
        }
        emit!(&mut self.c_src, b')', b'{');
        self.generate();
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_literal_return(&mut self, expression: ExpLiteral){
        emit!(&mut self.c_src, &b"return"[..], b' ', expression.val, b';');
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