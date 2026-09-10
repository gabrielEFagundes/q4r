use quaoar_core::{expdesc::VarDeclaration, generator::CodeGen, signatures::Type, tokens::{VoidstarToken, VoidstarTokenTypes}};

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
    tokens: &'a[VoidstarToken],
    source: &'a[u8],
    cursor: usize
}

impl<'a> CCompiler<'a>{
    pub fn new(source: &'a[u8], tokens: &'a[VoidstarToken]) -> Self{
        Self{ tokens, source, cursor: 0 }
    }

    fn parse_var_decl(&self, out: &mut Vec<u8>, declaration: VarDeclaration){
        out.extend_from_slice(declaration.ty.to_byte_span());
        out.push(b' ');
        out.extend_from_slice(declaration.ident);
        out.push(b'=');
        out.extend_from_slice(declaration.val);
        out.push(b';');
    }
}

impl<'a> CodeGen for CCompiler<'_>{
    fn generate(&mut self) -> Vec<u8> {
        let mut c_src: Vec<u8> = Vec::new();

        for i in self.tokens{
            match i.token_type(){
                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_var_decl(&mut c_src, dec);
                },

                VoidstarTokenTypes::Function => {
                    //let fun = Self::fun_declaration(self.tokens, self.source, &mut self.cursor);
                }

                _ => continue
            }
        }

        println!("{:#?}", c_src);
        c_src
    }
}