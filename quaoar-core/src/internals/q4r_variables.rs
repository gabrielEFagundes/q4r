use crate::{backend::Backend, expdesc::{ExpLiteral, ExpOperator, ExpType, VarDeclaration}, internals::{helpers, q4r_expressions, q4r_functions}, signature::{Literal, Operator, Type}, tokens::VoidstarTokenTypes};

pub fn var_declaration<'a>(backend: &mut Backend<'a>, is_pointer: bool) -> VarDeclaration<'a>{
    let mut ty = Type::map(backend.tokens[backend.cursor].token_type);

    if is_pointer{
        ty = ty.map_pointer();
    }

    helpers::expect(VoidstarTokenTypes::Ident, backend);

    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    let val: ExpType = if helpers::lookahead(backend).token_type == VoidstarTokenTypes::Equals{
        match var_callee(backend){
            ExpType::OperativeExp(exp_operator) => ExpType::OperativeExp(exp_operator),
            ExpType::LiteralExp(exp_literal) => ExpType::LiteralExp(exp_literal),
            ExpType::CallExp(fun_call) => ExpType::CallExp(fun_call),
            ExpType::AddressExp(exp_literal) => ExpType::AddressExp(exp_literal),
        }
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
                if helpers::lookahead(backend).token_type() == VoidstarTokenTypes::OpenParents{
                    let fun_call = q4r_functions::fun_call(backend);

                    return ExpType::CallExp(fun_call)
                } else {
                    let var_call_current = backend.tokens[backend.cursor];
                    return ExpType::LiteralExp(ExpLiteral { 
                        val: Literal::to_literal(
                            &backend.source[var_call_current.start..var_call_current.end], 
                            VoidstarTokenTypes::Ident
                        ) 
                    })
                }
            } else {
                backend.cursor += 1;
                return q4r_expressions::expression(backend);
            }
        },

        VoidstarTokenTypes::Increment | VoidstarTokenTypes::Decrement => {
            let identifier = &backend.source[
                backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
            ];
            backend.cursor += 1;

            let operator = Operator::map(backend.tokens[backend.cursor].token_type);
            backend.cursor += 1;

            let current = backend.tokens[backend.cursor];

            ExpType::OperativeExp(ExpOperator { left: identifier, operator, right: &backend.source[current.start..current.end] })
        },

        _ => panic!("invalid identifier call `{:#?}`", next)
    }
}