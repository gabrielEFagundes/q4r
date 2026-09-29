use crate::{backend::Backend, error::{error::{QError, QErrorTypes}, parser_err::ParserErrOpts}, expdesc::{ExpType, StmtType}, internals::{helpers::{self, advance}, q4r_expressions, q4r_values}, signature::DecKind, tokens::VoidstarTokenTypes};

pub fn relational_statement<'a>(backend: &mut Backend<'a>) -> StmtType<'a>{
    backend.cursor+=1;
    let expression = q4r_expressions::parse_expr(backend);
    
    StmtType::RelationalStmt { kind: DecKind::If, expression }
}

pub fn loop_statement<'a>(backend: &mut Backend<'a>) -> StmtType<'a>{
    backend.cursor += 1;
    let iterator = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];

    let init_expression = q4r_expressions::parse_expr(backend);

    backend.cursor += 1;
    if backend.tokens[backend.cursor].token_type == VoidstarTokenTypes::SemiColon{
        backend.cursor += 1;
        let exp = q4r_expressions::parse_expr(backend);

        backend.cursor += 1;
        let incrementer = q4r_values::unary_literal(backend);

        StmtType::ForLoopStmt { 
            kind: DecKind::For, iterator, initializer: init_expression, exp, incrementer 
        }

    }else{
        StmtType::RelationalStmt { kind: DecKind::While, expression: init_expression }
    }
}

pub fn assign_statement<'a>(backend: &mut Backend<'a>) -> ExpType<'a>{
    let next = helpers::lookahead(backend).token_type;
    match next{
        // var assignment
        VoidstarTokenTypes::Equals => {
            advance(2, backend);
            q4r_expressions::parse_expr(backend)
        },

        // i = i + /* whatever expression */
        VoidstarTokenTypes::Increment
        | VoidstarTokenTypes::Decrement => {
            advance(2, backend);

            
            todo!()
        }

        _ => QError::handle_new_error(
            QErrorTypes::ParserErr(ParserErrOpts::UnknownSymbol), 
            backend.tokens[backend.cursor].line, backend.tokens[backend.cursor].start, backend.debug
        )
    }
}