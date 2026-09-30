use crate::{backend::Backend, expdesc::{AssignOpStmt, AssignStmt, StmtType}, internals::{helpers::{advance, current, expect, lookahead, matches}, q4r_expressions, q4r_values}, signature::{DecKind, Operator}, tokens::VoidstarTokenTypes};

pub fn relational_statement<'a>(backend: &mut Backend<'a>) -> StmtType<'a>{
    advance(1, backend);
    let expression = q4r_expressions::parse_expr(backend);
    
    StmtType::relational_stmt(DecKind::If, expression)
}

pub fn loop_statement<'a>(backend: &mut Backend<'a>) -> StmtType<'a>{    
    expect(VoidstarTokenTypes::Ident, backend);

    let next = lookahead(backend);

    // while-styled loop
    if Operator::is_comparative(next.token_type){
        StmtType::relational_stmt(DecKind::While, q4r_expressions::parse_expr(backend))

    // cfor-styled loop
    } else if next.token_type == VoidstarTokenTypes::Equals{
        let initializer = assign_statement(backend);

        matches(VoidstarTokenTypes::SemiColon, backend);
        let exp = q4r_expressions::parse_expr(backend);

        matches(VoidstarTokenTypes::SemiColon, backend);
        let incrementer: crate::signature::Literal = q4r_values::unary_literal(backend);

        StmtType::loop_stmt(DecKind::For, initializer, exp, incrementer)

    } else {
        panic!()
    }
}

pub fn assign_statement<'a>(backend: &mut Backend<'a>) -> AssignStmt<'a>{
    let ident = &backend.source[current(backend).start..current(backend).end];

    advance(2, backend);
    let val = q4r_expressions::parse_expr(backend);

    AssignStmt { ident, val }
}

// i += /* whatever expression */
pub fn assign_op_statement<'a>(backend: &mut Backend<'a>) -> AssignOpStmt<'a>{
    let ident = &backend.source[current(backend).start..current(backend).end];

    advance(1, backend);
    let op = Operator::map(current(backend).token_type);

    advance(1, backend);
    let val = q4r_expressions::parse_expr(backend);

    AssignOpStmt { ident, op, val }
}