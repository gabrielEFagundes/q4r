use crate::{backend::Backend, expdesc::{ExpType, StmtType}, internals::{helpers::{self, advance, expect}, q4r_expressions}, signature::{Literal, Type}, tokens::VoidstarTokenTypes};

pub fn var_statement<'a>(backend: &mut Backend<'a>, is_pointer: bool) -> StmtType<'a>{
    let mut ty: Type;

    if is_pointer{
        ty = Type::map(backend.tokens[backend.cursor].token_type);
        ty = ty.map_pointer();
    } else {
        ty = Type::map(backend.tokens[backend.cursor].token_type);
    }

    expect(VoidstarTokenTypes::Ident, backend);
    
    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    let val: ExpType = if helpers::lookahead(backend).token_type == VoidstarTokenTypes::Equals{
        advance(2, backend);
        q4r_expressions::parse_expr(backend)
    }else{
        ExpType::LiteralExp{
            val: Literal::to_literal(&ty.default_literal().to_byte_span(), ty.as_token_type())
        }
    };

    StmtType::var_stmt(ty, ident, val)
}