use crate::{backend::Backend, error::{error, parser_err::ParserErrOpts}, expdesc::{ExpType, Parameter, StmtType}, internals::{helpers::{self, advance, current, expect, lookahead}, q4r_expressions, q4r_parameters}, signature::Type, tokens::VoidstarTokenTypes};

pub fn fun_declaration<'a>(backend: &mut Backend<'a>, is_extern: bool) -> StmtType<'a>{
    expect(VoidstarTokenTypes::Ident, backend);

    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    expect(VoidstarTokenTypes::OpenParents, backend);
    let mut params: Vec<Parameter> = Vec::new();

    advance(1, backend);
    while backend.tokens[backend.cursor].token_type != VoidstarTokenTypes::CloseParents{
        match q4r_parameters::declare_parameter(backend){
            Ok(param) => params.push(param),

            Err(err) => {
                if err.ty != error::QErrorTypes::Recoverable{
                    error::QError::handle_new_error(
                        error::QErrorTypes::ParserErr(ParserErrOpts::UnknownType), 
                        backend.tokens[backend.cursor].line, backend.tokens[backend.cursor].start, backend.debug
                    )
                }
            },
        }
    }

    let returns = if Type::has(helpers::lookahead(backend).token_type){
        advance(1, backend);
        Type::map(backend.tokens[backend.cursor].token_type)
    } else {
        Type::Void
    };

    if lookahead(backend).token_type == VoidstarTokenTypes::OpenBraces{
        expect(VoidstarTokenTypes::OpenBraces, backend);

    } else {
        advance(1, backend);
    }

    StmtType::FunStatement { returns, ident, params, is_extern }
}

pub fn fun_call<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let ident = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];
    expect(VoidstarTokenTypes::OpenParents, backend);

    let mut params: Vec<ExpType<'a>> = Vec::new();
    advance(1, backend);

    while current(backend).token_type != VoidstarTokenTypes::CloseParents{
        params.push(q4r_expressions::parse_expr(backend));
        helpers::skip_if_comma(backend);
    }

    ExpType::CallExp { ident, params }
}