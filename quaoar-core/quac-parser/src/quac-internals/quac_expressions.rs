use quac_ast::{Expression, expressions::UnaryExpression};
use quac_core::{backend::Backend, internals::helpers::{expect, expect_any}, tokens::Span};

use crate::prelude::{
    VoidstarTokenTypes, 
    helpers::{current, advance, lookahead, matches, add_error}
};

pub fn parse_primary(backend: &mut Backend) -> Expression{
    match current(backend).token_type() {
        VoidstarTokenTypes::IntLiteral
        | VoidstarTokenTypes::FloatLiteral
        | VoidstarTokenTypes::CharLiteral
        | VoidstarTokenTypes::BoolLiteral
        | VoidstarTokenTypes::StringLiteral => {
            let current = current(backend);
            advance(1, backend);

            Expression::LiteralExpression(current.span())
        }

        VoidstarTokenTypes::Ident => {
            // fun call
            if lookahead(backend).token_type() == VoidstarTokenTypes::OpenParents {
                todo!(); // return fun_call()
            }

            let current = current(backend);
            advance(1, backend);

            Expression::LiteralExpression(current.span())
        }

        VoidstarTokenTypes::OpenParents => {
            advance(1, backend);
            let inner = parse_expr(backend);
            matches(VoidstarTokenTypes::CloseParents, backend);
            inner
        }

        _ => {
            add_error("unexpected token found here", backend);
            Expression::BadExpression
        }
    }
}

pub fn parse_unary(backend: &mut Backend) -> Expression{
    match current(backend).token_type() {
        VoidstarTokenTypes::Ampersand => {
            let current_start = current(backend).start();
            let current_line = current(backend).line();
            // assuming it'd only accept the memory reference of variables
            // not too sure if that's the behavior I expect, but for now this will do
            expect(VoidstarTokenTypes::Ident, backend);
            
            let span = Span{ start: current_start, end: current(backend).end(), line: current_line };
            
            todo!()
        }

        VoidstarTokenTypes::Asterisk | VoidstarTokenTypes::Minus | VoidstarTokenTypes::Not => {
            let operator = current(backend).span();
            let expression = parse_expr(backend);

            Expression::UnaryExpression(UnaryExpression{ operator, expression }) // need to get the position of the expression on the ast
        }

        VoidstarTokenTypes::Plus => {
            advance(1, backend);
            parse_primary(backend)
        }

        _ => parse_primary(backend),
    }
}

pub fn parse_expr(backend: &mut Backend) -> Expression{
    todo!()
}