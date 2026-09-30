use crate::{
    backend::Backend,
    error::{
        error::{QError, QErrorTypes},
        parser_err::ParserErrOpts,
    },
    expdesc::ExpType,
    internals::{
        helpers::{advance, current, lookahead, matches},
        q4r_functions,
    },
    signature::{Literal, Operator},
    tokens::VoidstarTokenTypes,
};

pub fn parse_primary<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    match backend.tokens[backend.cursor].token_type {
        VoidstarTokenTypes::IntLiteral
        | VoidstarTokenTypes::FloatLiteral
        | VoidstarTokenTypes::CharLiteral
        | VoidstarTokenTypes::BoolLiteral
        | VoidstarTokenTypes::StringLiteral => {
            let current = backend.tokens[backend.cursor];
            advance(1, backend);

            ExpType::LiteralExp {
                val: Literal::to_literal(
                    &backend.source[current.start..current.end],
                    current.token_type,
                ),
            }
        }

        VoidstarTokenTypes::Ident => {
            // fun call
            let next = lookahead(backend);
            if next.token_type == VoidstarTokenTypes::OpenParents {
                return q4r_functions::fun_call(backend);
            }

            let current = backend.tokens[backend.cursor];
            advance(1, backend);

            ExpType::LiteralExp {
                val: Literal::to_literal(
                    &backend.source[current.start..current.end],
                    current.token_type,
                ),
            }
        }

        VoidstarTokenTypes::OpenParents => {
            advance(1, backend);
            let inner = parse_expr(backend);
            matches(VoidstarTokenTypes::CloseParents, backend);
            inner
        }

        _ => QError::handle_new_error(
            QErrorTypes::ParserErr(ParserErrOpts::UnexpectedToken),
            current(backend).line,
            current(backend).start,
            backend.debug,
        ),
    }
}

pub fn parse_unary<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    match current(backend).token_type {
        VoidstarTokenTypes::Ampersand => {
            advance(1, backend);
            ExpType::AddressExp {
                val: Box::new(parse_primary(backend)),
            }
        }

        VoidstarTokenTypes::Asterisk | VoidstarTokenTypes::Minus => {
            let op = Operator::map(current(backend).token_type);
            advance(1, backend);

            ExpType::SignedExp {
                op,
                val: Box::new(parse_primary(backend)),
            }
        }

        VoidstarTokenTypes::Plus => {
            advance(1, backend);
            parse_primary(backend)
        }

        _ => parse_primary(backend),
    }
}

pub fn parse_expr<'a>(backend: &mut Backend<'a>) -> ExpType<'a> {
    let mut left = parse_unary(backend);

    while Operator::is_legal(backend.tokens[backend.cursor].token_type) {
        let operator = Operator::map(current(backend).token_type);
        advance(1, backend);
        let right = parse_unary(backend);
        left = ExpType::BinaryExp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    left
}
