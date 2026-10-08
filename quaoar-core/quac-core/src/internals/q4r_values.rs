use crate::{
    backend::Backend,
    signature::{Literal, Operator},
    tokens::VoidstarTokenTypes,
};

pub fn unary_literal<'a>(backend: &mut Backend<'a>) -> Literal {
    let sign_type = backend.tokens[backend.cursor];
    let sign =
        &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];
    backend.cursor += 1;

    if !Operator::is_arithmetic(sign_type.token_type) {
        return Literal::to_literal(sign, VoidstarTokenTypes::Ident);
    }

    let number =
        &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];
    let bytes = [sign, number].concat();

    Literal::to_literal(&bytes, VoidstarTokenTypes::Int)
}
