use quac_ast::ast::Ast;
use quac_core::{backend::Backend, errors::error::QError};

use crate::parser::QuacParser;

pub mod parser;
pub mod prelude;

#[path = "quac-internals/mod.rs"]
pub mod quac_internals;

pub fn run<'a>(backend: Backend<'a>) -> (Ast, QError){
    #[allow(unused)]
    let parser = QuacParser::new(backend);

    todo!();
}