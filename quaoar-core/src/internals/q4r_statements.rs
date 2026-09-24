use crate::{backend::Backend, expdesc::{ComparisonDeclaration, DeclType, ForLoopDeclaration}, internals::{q4r_expressions, q4r_values}, signature::DecKind, tokens::VoidstarTokenTypes};

pub fn relational_statement<'a>(backend: &mut Backend<'a>) -> ComparisonDeclaration<'a>{
    backend.cursor+=1;
    let expression = q4r_expressions::expression(backend);
    
    ComparisonDeclaration { kind: DecKind::If, expression }
}

pub fn loop_statement<'a>(backend: &mut Backend<'a>) -> DeclType<'a>{
    backend.cursor += 1;
    let iterator = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];

    let init_expression = q4r_expressions::expression(backend);

    backend.cursor += 1;
    if backend.tokens[backend.cursor].token_type == VoidstarTokenTypes::SemiColon{
        backend.cursor += 1;
        let exp = q4r_expressions::expression(backend);

        backend.cursor += 1;
        let incrementer = q4r_values::unary_literal(backend).val;

        DeclType::ForLoopDecl(ForLoopDeclaration { 
            kind: DecKind::For, iterator, initializer: init_expression, exp, incrementer 
        })

    }else{
        DeclType::WhileLoopDecl(ComparisonDeclaration { kind: DecKind::While, expression: init_expression })
    }
}