use crate::{backend::Backend, error::{error::QError, parser_err::ParserErrOpts}, expdesc::{ExpLiteral, ExpType, VarDeclaration}, internals::{helpers, q4r_expressions, q4r_functions}, prelude::error, signature::{Literal, Operator, Type}, tokens::VoidstarTokenTypes};

pub fn var_declaration<'a>(backend: &mut Backend<'a>, is_pointer: bool) -> VarDeclaration<'a>{
    let mut ty: Type;

    if is_pointer{
        backend.cursor += 1;
        ty = Type::map(backend.tokens[backend.cursor].token_type);
        ty = ty.map_pointer();
    } else {
        ty = Type::map(backend.tokens[backend.cursor].token_type);
    }

    helpers::expect(VoidstarTokenTypes::Ident, backend);

    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    let val: ExpType = if helpers::lookahead(backend).token_type == VoidstarTokenTypes::Equals{
        var_callee(backend)
    }else{
        ExpType::LiteralExp(ExpLiteral{
            val: Literal::to_literal(&ty.default_literal().to_byte_span(), ty.as_token_type())
        })
    };

    VarDeclaration { ty, ident, val }
}

pub fn var_callee<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let next = helpers::lookahead(backend).token_type;
    match next{
        // func call
        VoidstarTokenTypes::OpenParents => {
            let fun_call = q4r_functions::fun_call(backend);

            return ExpType::CallExp(fun_call)
        },

        // var assignment
        VoidstarTokenTypes::Equals => {
            backend.cursor += 1;

            if helpers::lookahead(backend).token_type() == VoidstarTokenTypes::Ident{
                backend.cursor += 1;
                q4r_expressions::parse_expr(backend)
                
            } else {
                backend.cursor += 1;
                return q4r_expressions::parse_expr(backend);
            }
        },

        // FUCK YOU INCREMENT DECREMENT
        // VoidstarTokenTypes::Increment | VoidstarTokenTypes::Decrement => {
        //     let identifier = &backend.source[
        //         backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
        //     ];
        //     backend.cursor += 1;

        //     let operator = Operator::map(backend.tokens[backend.cursor].token_type);
        //     backend.cursor += 1;

        //     let current = backend.tokens[backend.cursor];

        //     ExpType::OperativeExp(ExpOperator { left: identifier, operator, right: &backend.source[current.start..current.end] })
        // },

        _ => QError::handle_new_error(
            error::QErrorTypes::ParserErr(ParserErrOpts::UnknownSymbol), 
            backend.tokens[backend.cursor].line, backend.tokens[backend.cursor].start, backend.debug
        )
    }
}