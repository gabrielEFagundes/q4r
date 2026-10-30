use crate::{
    backend::Backend, expdesc::{ExpType, StmtType}, internals::{
        helpers::{self, add_error, advance, current, expect}, 
        q4r_expressions,
    }, 
    signature::{Literal, Type}, 
    tokens::VoidstarTokenTypes,
};

pub fn var_statement<'a>(backend: &mut Backend<'a>, is_pointer: bool) -> StmtType<'a> {
    let mut ty: Type = Type::map(current(backend).token_type);
    if ty.is_invalid(){
        add_error("invalid data type here", backend);
    }

    if is_pointer { ty = ty.map_pointer(); }

    expect(VoidstarTokenTypes::Ident, backend);

    let ident =
        &backend.source[current(backend).start()..current(backend).end()];

    let val: ExpType = if helpers::lookahead(backend).token_type == VoidstarTokenTypes::Equals {
        advance(2, backend);
        q4r_expressions::parse_expr(backend)
    } else {
        let _def_lit = &ty.default_literal();
        let def = helpers::ok_or_report(backend, _def_lit.to_byte_span());

        ExpType::literal_exp(Literal::to_literal(def.as_ref().unwrap().as_slice(), ty.as_token_type().unwrap()))
    };

    StmtType::var_stmt(ty, ident, val)
}
