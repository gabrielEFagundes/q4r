use quac_ast::ast::Ast;
use quac_core::{errors::error::QError, tokens::VoidstarToken};

use crate::parser::QuacParser;

pub mod parser;

pub fn run<'a>(src: &'a[u8], tokens: &'a[VoidstarToken]) -> (Ast, QError){
    #[allow(unused)]
    let parser = QuacParser::new(src, tokens);

    todo!();
}