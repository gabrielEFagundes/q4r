use crate::{backend::Backend, error::parser_err::ParserErrOpts, expdesc::{ExpLiteral, ExpOperator, ExpType}, internals::{helpers, q4r_functions, q4r_values, q4r_variables}, prelude::error, signature::{Literal, Operator}, tokens::VoidstarTokenTypes};

pub fn parse_primary<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    match backend.tokens[backend.cursor].token_type{
        VoidstarTokenTypes::IntLiteral
        |VoidstarTokenTypes::FloatLiteral
        |VoidstarTokenTypes::CharLiteral
        |VoidstarTokenTypes::BoolLiteral
        |VoidstarTokenTypes::StringLiteral
        |VoidstarTokenTypes::Ident => {
            let current = backend.tokens[backend.cursor];
            backend.cursor += 1;

            ExpType::LiteralExp(ExpLiteral {
                 val: Literal::to_literal(&backend.source[current.start..current.end], current.token_type)
            })
        },

        _ => todo!("fuck whoever's reading this")
    }
}

pub fn parse_unary<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    match backend.tokens[backend.cursor].token_type{
        VoidstarTokenTypes::Ampersand => {
            backend.cursor += 1;
            let current = backend.tokens[backend.cursor];
            let address_of = parse_primary(backend);
            
            ExpType::AddressExp(ExpLiteral { 
                val: Literal::to_literal(&backend.source[current.start..current.end], VoidstarTokenTypes::Ident)
            })
        },

        _ => todo!("I'm rewriting this, please be patient")
    }
}

pub fn parse_expr<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let mut left = parse_primary(backend);

    backend.cursor += 1;
    let operator = Operator::map(backend.tokens[backend.cursor].token_type);

    backend.cursor += 1;
    let right = parse_primary(backend);

    if helpers::lookahead(backend).token_type
    todo!()
}