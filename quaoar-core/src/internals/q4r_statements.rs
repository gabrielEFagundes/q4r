use crate::{backend::Backend, expdesc::{ComparisonDeclaration, DeclType, ForLoopDeclaration}, internals::{helpers, q4r_expressions, q4r_values}, signature::DecKind, tokens::VoidstarTokenTypes};

pub fn relational_statement<'a>(backend: &mut Backend<'a>) -> ComparisonDeclaration<'a>{
    backend.cursor+=1;
    let expression = q4r_expressions::expression(backend);
    
    ComparisonDeclaration { kind: DecKind::If, expression }
}

pub fn loop_statement<'a>(backend: &mut Backend<'a>) -> DeclType<'a>{
    backend.cursor += 1;
    let iterator = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];

    let mut init_expression = q4r_expressions::expression(backend);
    backend.cursor += 2;

    if helpers::lookahead(backend).token_type == VoidstarTokenTypes::SemiColon{
        init_expression = q4r_expressions::expression(backend);

        backend.cursor += 2;
        let exp = q4r_expressions::expression(backend);

        backend.cursor += 1;
        let incrementer = q4r_values::signed_literal(backend).val;

        DeclType::ForLoopDecl(ForLoopDeclaration { 
            kind: DecKind::For, iterator, initializer: init_expression, exp, incrementer 
        })

    }else{
        DeclType::WhileLoopDecl(ComparisonDeclaration { kind: DecKind::While, expression: init_expression })
    }
}