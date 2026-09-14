use std::collections::HashMap;

use quaoar_core::{expdesc::{ComparisonDeclaration, Expression, FunDeclaration, VarDeclaration}, generator::CodeGen, signatures::{self, Signature, Type}, tokens::{VoidstarToken, VoidstarTokenTypes::{self, Function}}};
use crate::impls::AppendTo;

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

    // pub fn resolve_in_scope(&mut self, ident: &str) -> Option<&Type>{
    //     self.scopes.iter().rev()
    //         .find_map(|s| s.get(ident))
    //         .or_else(|| self.signatures.get(ident))
    // }

    pub(crate) fn gen_signature_headers(&mut self){
        for i in &self.signatures{
            match i.1{
                Signature::Function { returns, params } => {
                    emit!(
                        &mut self.c_src, 
                        returns.to_byte_span(), b' ', i.0.as_bytes(), b'('
                    );
                    for i in params{
                        emit!(&mut self.c_src, i.to_byte_span());
                    }
                    emit!(&mut self.c_src, b')', b';');
                },
                Signature::Global { ty, value } => {
                    emit!(&mut self.c_src, ty.to_byte_span(), b' ', i.0.as_bytes(), b';');
                },
            }
        }
    }

    pub(crate) fn parse_expression(&mut self, expression: Expression){
        // e.g. ((left)==(right))
        emit!(
            &mut self.c_src,
            b'(', b'(', expression.left, b')',
            expression.operator.to_byte_span(),
            b'(', expression.right, b')', b')'
        );
    }

    pub(crate) fn parse_cmp_expression(&mut self, cmp_expression: ComparisonDeclaration){
        // e.g. if expression{} or while expression{}
        emit!(&mut self.c_src, cmp_expression.kind.to_byte_span());
        self.parse_expression(cmp_expression.expression);
        emit!(&mut self.c_src, b'{');
        self.generate();
        emit!(&mut self.c_src, b'}');
    }

    pub(crate) fn parse_var_decl(&mut self, declaration: VarDeclaration){
        // int var = value
        emit!(
            &mut self.c_src, 
            declaration.ty.to_byte_span(), b' ',
            declaration.ident, b'=', declaration.val, b';'
        );
    }

    pub(crate) fn parse_fun_decl(&mut self, declaration: FunDeclaration){
        // int function(int p1, int p2){ }
        emit!(
            &mut self.c_src,
            declaration.returns.to_byte_span(), b' ',
            declaration.ident, b'('
        );
        for i in declaration.params{
            emit!(&mut self.c_src, i);
        }
        emit!(&mut self.c_src, b')', b'{');
        self.generate();
        emit!(&mut self.c_src, b'}');
    }
}