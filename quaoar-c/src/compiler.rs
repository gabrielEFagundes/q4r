use quaoar_core::{expdesc::{FunDeclaration, VarDeclaration}, generator::CodeGen, signatures::Type, tokens::{VoidstarToken, VoidstarTokenTypes}};

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
/// - msvc
/// - gcc
/// - clang
/// 
/// Those are the ones that were or are being tested.
/// 
/// That doesn't mean Q4r doesn't support other compilers.
pub struct CCompiler<'a>{
    pub(crate) tokens: &'a[VoidstarToken],
    pub(crate) source: &'a[u8],
    pub(crate) cursor: usize
}

impl<'a> CCompiler<'a>{
    pub fn new(source: &'a[u8], tokens: &'a[VoidstarToken]) -> Self{
        Self{ tokens, source, cursor: 0 }
    }

    pub(crate) fn parse_var_decl(&self, out: &mut Vec<u8>, declaration: VarDeclaration){
        // int var = value
        out.extend_from_slice(declaration.ty.to_byte_span());
        out.push(b' ');
        out.extend_from_slice(declaration.ident);
        out.push(b'=');
        out.extend_from_slice(declaration.val);
        out.push(b';');
    }

    pub(crate) fn parse_fun_decl(&self, out: &mut Vec<u8>, declaration: FunDeclaration){
        // int function(int p1, int p2){ }
        out.extend_from_slice(declaration.returns.to_byte_span());
        out.push(b' ');
        out.extend_from_slice(declaration.ident);
        out.push(b'(');
        for i in declaration.params{
            out.extend_from_slice(i);
        }
        out.push(b')');
        out.push(b';');
    }
}