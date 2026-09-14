use quaoar_core::{generator::CodeGen, tokens::VoidstarTokenTypes};

use crate::compiler::CCompiler;

impl<'a> CodeGen for CCompiler<'_>{
    fn generate(&mut self) -> Vec<u8> {
        self.gen_signature_headers();

        while !Self::end(self.tokens, &mut self.cursor){
            println!("{:#?}", self.tokens[self.cursor]);
            match self.tokens[self.cursor].token_type(){
                VoidstarTokenTypes::Int
                | VoidstarTokenTypes::Float
                | VoidstarTokenTypes::Bool
                | VoidstarTokenTypes::Char
                | VoidstarTokenTypes::Void => {
                    let dec = Self::var_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_var_decl(dec);
                },

                VoidstarTokenTypes::If | VoidstarTokenTypes::While => {
                    let cmp = Self::comparison_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_cmp_expression(cmp);
                }

                VoidstarTokenTypes::Function => {
                    let fun = Self::fun_declaration(self.tokens, self.source, &mut self.cursor);
                    self.parse_fun_decl(fun);
                }

                // VoidstarTokenTypes::Ident => {
                //     //let id = Self::standalone_ident(self.tokens, self.source, &mut self.cursor);
                // }

                VoidstarTokenTypes::OpenBraces => self.cursor+=1,
                VoidstarTokenTypes::CloseBraces => break,

                _ => self.cursor+=1
            }
        }

        println!("{:#?}", self.c_src);
        self.c_src.clone()
    }
}