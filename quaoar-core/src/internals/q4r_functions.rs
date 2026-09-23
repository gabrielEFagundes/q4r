use crate::{backend::Backend, error::QErrorTypes, expdesc::{ExpType, FunCall, FunDeclaration, Parameter}, internals::{helpers, q4r_expressions, q4r_parameters}, signature::Type, tokens::VoidstarTokenTypes};

pub fn fun_declaration<'a>(backend: &mut Backend<'a>, is_extern: bool) -> FunDeclaration<'a>{
    helpers::expect(VoidstarTokenTypes::Ident, backend);
    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    helpers::expect(VoidstarTokenTypes::OpenParents, backend);
    let mut params: Vec<Parameter> = Vec::new();

    backend.cursor+=1;
    while backend.tokens[backend.cursor].token_type != VoidstarTokenTypes::CloseParents{
        match q4r_parameters::declare_parameter(backend){
            Ok(param) => params.push(param),

            Err(err) => {
                if err.ty != QErrorTypes::Recoverable{
                    panic!("unknown type as a parameter of `{}`: `{:#?}`", std::str::from_utf8(ident).unwrap(), backend.tokens[backend.cursor].token_type)
                }
            },
        }
    }

    let returns = if Type::has(helpers::lookahead(backend).token_type){
        backend.cursor += 1;
        Type::map(backend.tokens[backend.cursor].token_type)
    } else {
        Type::Void
    };

    if helpers::lookahead(backend).token_type == VoidstarTokenTypes::OpenBraces{
        helpers::expect(VoidstarTokenTypes::OpenBraces, backend);

    } else {
        backend.cursor += 1;
    }

    FunDeclaration { returns, ident, params, is_extern }
}

pub fn fun_call<'a>(backend: &mut Backend<'a>) -> FunCall<'a>{
    let ident = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];
    let mut params: Vec<ExpType<'a>> = Vec::new();
    backend.cursor += 2;

    while backend.tokens[backend.cursor].token_type() != VoidstarTokenTypes::CloseParents{
        match q4r_expressions::expression(backend){
            crate::expdesc::ExpType::OperativeExp(exp_operator) => params.push(ExpType::OperativeExp(exp_operator)),
            crate::expdesc::ExpType::LiteralExp(exp_literal) => params.push(ExpType::LiteralExp(exp_literal)),
            crate::expdesc::ExpType::CallExp(fun_call) => params.push(ExpType::CallExp(fun_call)),
            crate::expdesc::ExpType::AddressExp(exp_literal) => params.push(ExpType::AddressExp(exp_literal)),
        }

        dbg!(backend.tokens[backend.cursor]);
        backend.cursor += 1;

        // revisit this, it's ugly
        if backend.tokens[backend.cursor].token_type == VoidstarTokenTypes::Comma{
            backend.cursor += 1;
        }
    }

    FunCall { ident, params }
}