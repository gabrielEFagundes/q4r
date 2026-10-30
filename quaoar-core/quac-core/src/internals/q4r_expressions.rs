use crate::{
    backend::Backend, 
    expdesc::ExpType, 
    internals::{
        helpers::{
            self, 
            advance, 
            current, 
            expect_any, 
            lookahead, 
            matches
        }, 
        q4r_functions,
    }, 
    signature::{Literal, Operator}, 
    tokens::VoidstarTokenTypes,
};

pub fn parse_primary<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    match current(backend).token_type {
        VoidstarTokenTypes::IntLiteral
        | VoidstarTokenTypes::FloatLiteral
        | VoidstarTokenTypes::CharLiteral
        | VoidstarTokenTypes::BoolLiteral
        | VoidstarTokenTypes::StringLiteral => {
            let current = current(backend);
            advance(1, backend);

            ExpType::literal_exp(
                Literal::to_literal(
                    &backend.source[current.start()..current.end()],
                    current.token_type,
                )
            )
        }

        VoidstarTokenTypes::Ident => {
            // fun call
            let next = lookahead(backend);
            if next.token_type == VoidstarTokenTypes::OpenParents {
                return q4r_functions::fun_call(backend);
            }

            let current = current(backend);
            advance(1, backend);

            ExpType::literal_exp(
                Literal::to_literal(
                    &backend.source[current.start()..current.end()],
                    current.token_type,
                )
            )
        }

        VoidstarTokenTypes::OpenParents => {
            advance(1, backend);
            let inner = parse_expr(backend);
            matches(VoidstarTokenTypes::CloseParents, backend);
            inner
        }

        _ => {
            helpers::add_error("unexpected token found here", backend);
            ExpType::BadExp
        }
    }
}

pub fn parse_unary<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    match current(backend).token_type {
        VoidstarTokenTypes::Ampersand => {
            advance(1, backend);
            ExpType::address_exp(Box::new(parse_primary(backend)))
        }

        // meh
        VoidstarTokenTypes::Not => {
            expect_any(
                &[
                    VoidstarTokenTypes::BoolLiteral,
                    VoidstarTokenTypes::Ident,
                    VoidstarTokenTypes::OpenParents,
                ],
                backend,
            );
            ExpType::signed_exp(Operator::Not, Box::new(parse_primary(backend)))
        }

        VoidstarTokenTypes::Asterisk | VoidstarTokenTypes::Minus => {
            let op = Operator::map(current(backend).token_type);
            advance(1, backend);

            ExpType::signed_exp(op, Box::new(parse_primary(backend)))
        }

        VoidstarTokenTypes::Plus => {
            advance(1, backend);
            parse_primary(backend)
        }

        _ => parse_primary(backend),
    }
}

// this is strictly a left to right parse, which is bad
// I'll be fixing as I go for the qbe backend, I really can't wait anymore
pub fn parse_expr<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    let mut left = parse_unary(backend);

    while Operator::is_legal(current(backend).token_type) {
        let operator = Operator::map(current(backend).token_type);
        advance(1, backend);

        let right = if operator.is_boolean_algebra() {
            parse_expr(backend)
        } else {
            parse_unary(backend)
        };
        left = ExpType::binary_exp(
            Box::new(left), operator, Box::new(right)
        );
    }

    left
}
