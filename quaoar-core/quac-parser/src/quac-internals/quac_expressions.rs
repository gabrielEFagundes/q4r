use quac_ast::{ExprId, Expression, expressions::{BinaryExpression, UnaryExpression}};
use quac_core::{backend::Backend, internals::helpers::expect, tokens::Span};

use crate::{
    parser::QuacParser, 
    prelude::{
        VoidstarTokenTypes, 
        helpers::{add_error, advance, current, lookahead, matches}
    }
};

fn parse_primary(parser: &mut QuacParser, backend: &mut Backend) -> Expression{
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
            let inner = parse_expr(parser, backend);
            matches(VoidstarTokenTypes::CloseParents, backend);
            inner
        }

        _ => {
            add_error("unexpected token found here", backend);
            Expression::BadExpression
        }
    }
}

fn parse_nud(parser: &mut QuacParser, backend: &mut Backend) -> Expression{
    match current(backend).token_type() {
        VoidstarTokenTypes::Ampersand => {
            let current_start = current(backend).start();
            let current_line = current(backend).line();
            // assuming it'd only accept the memory reference of variables
            // not too sure if that's the behavior I expect, but for now this will do
            expect(VoidstarTokenTypes::Ident, backend);
            
            let span = Span{ start: current_start, end: current(backend).end(), line: current_line };
            
            Expression::LiteralExpression(span)
        }

        VoidstarTokenTypes::Asterisk | VoidstarTokenTypes::Minus | VoidstarTokenTypes::Not => {
            let operator = current(backend).span();
            let expression = parse_expr(parser, backend);
            let expr_id = alloc(parser, expression);

            Expression::UnaryExpression(UnaryExpression{ operator, expression: expr_id }) // need to get the position of the expression on the ast
        }

        VoidstarTokenTypes::Plus => {
            advance(1, backend);
            parse_primary(parser, backend)
        }

        _ => parse_primary(parser, backend),
    }
}

fn parse_led(parser: &mut QuacParser, backend: &mut Backend) -> Expression{
    todo!()
}

pub fn parse_expr(parser: &mut QuacParser, backend: &mut Backend) -> Expression{
    let mut left = parse_nud(parser, backend);

    // if it gets here, it's (so far) 100% a binary expression
    while quac_core::signature::Operator::is_legal(current(backend).token_type()){
        let expr = BinaryExpression{
            left: alloc(parser, left),
            operator: None,// oh my god
            right: alloc(parser, parse_led(parser, backend))
        };

        left = Expression::BinaryExpression(expr);
    }

    left
}

/// Allocates the expression into the `exprs` vector of the ast and returns it's ID (or position)
fn alloc(parser: &mut QuacParser, e: Expression) -> ExprId{
    let id = parser.ast.exprs.len();
    parser.ast.exprs.push(e);
    id
}