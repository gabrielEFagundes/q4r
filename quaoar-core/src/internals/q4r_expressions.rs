use crate::{backend::Backend, expdesc::{ExpLiteral, ExpOperator, ExpType}, internals::{helpers, q4r_functions, q4r_values, q4r_variables}, signature::{Literal, Operator}, tokens::VoidstarTokenTypes};

pub fn mount_expression<'a>(backend: &mut Backend<'a>) -> ExpOperator<'a>{
    let left: &[u8];
    let right: &[u8];
    let operator;
    
    let lstart = backend.tokens[backend.cursor].start;
    
    while !(Operator::is_comparative(backend.tokens[backend.cursor].token_type) ||
            Operator::is_arithmetic(backend.tokens[backend.cursor].token_type)){
            backend.cursor+=1;
    }
    
    let lend = backend.tokens[backend.cursor-1].end;
    left = &backend.source[lstart..lend];

    operator = Operator::map(backend.tokens[backend.cursor].token_type);

    backend.cursor+=1;
    let rstart = backend.tokens[backend.cursor].start;

    while Operator::is_legal(backend.tokens[backend.cursor].token_type){ 
        backend.cursor+=1;
    }

    let rend = backend.tokens[backend.cursor-1].end;
    right = &backend.source[rstart..rend];

    ExpOperator{ left, operator, right }
}

pub fn expression<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let current = backend.tokens[backend.cursor];
    let expression_type = helpers::lookahead(backend).token_type;

    let mut subexpressions: Vec<ExpType<'_>> = Vec::new();

    // recurses back if a subexpression is detected
    // revisit this later
    if current.token_type == VoidstarTokenTypes::OpenBraces{
        backend.cursor += 1;
        subexpressions.push(expression(backend));
    }

    match current.token_type{
        VoidstarTokenTypes::IntLiteral
        |VoidstarTokenTypes::FloatLiteral
        |VoidstarTokenTypes::BoolLiteral
        |VoidstarTokenTypes::CharLiteral
        |VoidstarTokenTypes::StringLiteral => {
            if Operator::is_comparative(expression_type) || Operator::is_arithmetic(expression_type){
                return ExpType::OperativeExp(mount_expression(backend))
            }

            let bytes_value = &backend.source[current.start..current.end];

            ExpType::LiteralExp(ExpLiteral{
                val: Literal::to_literal(bytes_value, current.token_type)
            })
        },

        VoidstarTokenTypes::Plus | VoidstarTokenTypes::Minus => {
            ExpType::LiteralExp(ExpLiteral {
                val: q4r_values::unary_literal(backend).val
            })
        },

        VoidstarTokenTypes::Ident => {
            if Operator::is_comparative(expression_type) || Operator::is_arithmetic(expression_type){
                return ExpType::OperativeExp(mount_expression(backend))
            }

            if expression_type == VoidstarTokenTypes::OpenParents{
                return ExpType::CallExp(q4r_functions::fun_call(backend));
            }

            if expression_type == VoidstarTokenTypes::Equals{
                return q4r_variables::var_callee(backend)
            }

            let bytes_value = &backend.source[current.start..current.end];

            ExpType::LiteralExp(ExpLiteral { 
                val: Literal::to_literal(bytes_value, current.token_type)
            })
        },

        VoidstarTokenTypes::Ampersand => {
            let mut bytes_ident = Vec::from(b"&");

            backend.cursor += 1;
            bytes_ident.extend_from_slice(&backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end]);

            ExpType::AddressExp(ExpLiteral { 
                val: Literal::to_literal(bytes_ident.as_slice(), VoidstarTokenTypes::Ident)
            })
        }

        _ => panic!("illegal expression argument `{:#?}`", current.token_type)
    }
}