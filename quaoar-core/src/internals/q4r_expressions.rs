use crate::{backend::Backend, error::{error::{QError, QErrorTypes}, parser_err::ParserErrOpts}, expdesc::ExpType, internals::{helpers::{advance, current, lookahead, matches}, q4r_functions::{self, fun_call}}, signature::{Literal, Operator}, tokens::VoidstarTokenTypes};

pub fn parse_primary<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    match backend.tokens[backend.cursor].token_type{
        VoidstarTokenTypes::IntLiteral
        |VoidstarTokenTypes::FloatLiteral
        |VoidstarTokenTypes::CharLiteral
        |VoidstarTokenTypes::BoolLiteral
        |VoidstarTokenTypes::StringLiteral => {
            let current = backend.tokens[backend.cursor];
            advance(1, backend);

            ExpType::LiteralExp {
                 val: Literal::to_literal(&backend.source[current.start..current.end], current.token_type)
            }
        },

        VoidstarTokenTypes::Ident => {
            // fun call
            let next = lookahead(backend);
            if next.token_type == VoidstarTokenTypes::OpenParents{
                return q4r_functions::fun_call(backend)
            }

            let current = backend.tokens[backend.cursor];
            advance(1, backend);
            
            ExpType::LiteralExp{
                val: Literal::to_literal(&backend.source[current.start..current.end], current.token_type)
            }
        },

        VoidstarTokenTypes::OpenParents => {
            advance(1, backend);
            let inner = parse_expr(backend);
            matches(VoidstarTokenTypes::CloseParents, backend);
            inner
        }

        _ => QError::handle_new_error(
            QErrorTypes::ParserErr(ParserErrOpts::UnexpectedToken), 
            current(backend).line, current(backend).start, backend.debug)
    }
}

pub fn parse_unary<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    match backend.tokens[backend.cursor].token_type{
        VoidstarTokenTypes::Ampersand => {
            // backend.cursor += 1;
            // let current = backend.tokens[backend.cursor];
            // let address_of = parse_primary(backend);
            
            // ExpType::AddressExp(ExpLiteral { 
            //     val: Literal::to_literal(&backend.source[current.start..current.end], VoidstarTokenTypes::Ident)
            // })
            todo!()
        },

        VoidstarTokenTypes::Plus | VoidstarTokenTypes::Minus => {
            todo!()
        }

        _ => parse_primary(backend)
    }
}

pub fn parse_expr<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let mut left = parse_unary(backend);

    while Operator::is_legal(backend.tokens[backend.cursor].token_type){
        let operator = Operator::map(current(backend).token_type);
        advance(1, backend);
        let right = parse_unary(backend);
        left = ExpType::BinaryExp{
            left: Box::new(left), 
            operator, 
            right: Box::new(right) 
        };
    }
    
    left
}