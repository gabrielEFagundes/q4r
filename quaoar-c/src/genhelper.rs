use quaoar_core::{generator::CodeGen, tokens::VoidstarTokenTypes};

use crate::compiler::CCompiler;

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
                    let fun = Self::fun_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_fun_decl(&mut c_src, fun);
                }

                _ => continue
            }
        }

        println!("{:#?}", c_src);
        c_src
    }
}