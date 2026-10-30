use crate::{
    backend::Backend, 
    internals::helpers::{advance, current}, 
    signature::{Literal, Operator}, 
    tokens::VoidstarTokenTypes,
};

pub fn unary_literal<'a>(backend: &mut Backend<'a>) -> Literal {
    let sign_type = current(backend);
    let sign =
        &backend.source[current(backend).start()..current(backend).end()];
    advance(1, backend);

    if !Operator::is_arithmetic(sign_type.token_type) {
        return Literal::to_literal(sign, VoidstarTokenTypes::Ident);
    }

    let number =
        &backend.source[current(backend).start()..current(backend).end()];
    let bytes = [sign, number].concat();

    Literal::to_literal(&bytes, VoidstarTokenTypes::Int)
}
