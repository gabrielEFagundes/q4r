use quac_ast::ast::Ast;
use quac_core::backend::Backend;

pub struct QuacParser<'a>{
    pub backend: Backend<'a>,
    pub ast: Ast,
}

impl<'a> QuacParser<'a>{
    pub fn new(backend: Backend<'a>) -> Self{
        QuacParser { 
            backend,
            ast: Ast { 
                nodes: Vec::new() 
            }
        }
    }
}