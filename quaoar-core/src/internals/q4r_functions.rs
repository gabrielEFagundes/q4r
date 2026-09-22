use crate::{backend::Backend, expdesc::{FunCall, FunDeclaration, VarDeclaration}, internals::{helpers, q4r_variables}, signature::Type, tokens::VoidstarTokenTypes};

pub fn fun_declaration<'a>(backend: &mut Backend<'a>) -> FunDeclaration<'a>{
    helpers::expect(VoidstarTokenTypes::Ident, backend);
    let ident = &backend.source[
        backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end
    ];

    helpers::expect(VoidstarTokenTypes::OpenParents, backend);
    let mut params: Vec<VarDeclaration> = Vec::new();

    backend.cursor+=1;
    while backend.tokens[backend.cursor].token_type != VoidstarTokenTypes::CloseParents{
        match backend.tokens[backend.cursor].token_type{
            VoidstarTokenTypes::Int
            | VoidstarTokenTypes::Float
            | VoidstarTokenTypes::Char
            | VoidstarTokenTypes::Bool => {
                let declaration: VarDeclaration<'a> = q4r_variables::var_declaration(backend, false);

                params.push(declaration);
                backend.cursor += 1;
            },

            VoidstarTokenTypes::Comma | VoidstarTokenTypes::Void => backend.cursor+=1,
            
            _ => panic!("unknown symbol as a parameter of `{}`: `{:#?}`", str::from_utf8(ident).unwrap_or("unknown"), backend.tokens[backend.cursor].token_type)
        }
    }

    let returns = if Type::has(helpers::lookahead(backend).token_type){
        backend.cursor += 1;
        Type::map(backend.tokens[backend.cursor].token_type)
    } else {
        Type::Void
    };

    helpers::expect(VoidstarTokenTypes::OpenBraces, backend);
    
    FunDeclaration { returns, ident, params }
}

pub fn fun_call<'a>(backend: &mut Backend<'a>) -> FunCall<'a>{
    let ident = &backend.source[backend.tokens[backend.cursor].start..backend.tokens[backend.cursor].end];
    let mut params: Vec<u8> = Vec::new();
    backend.cursor += 2;

    while backend.tokens[backend.cursor].token_type() != VoidstarTokenTypes::CloseParents{
            let current = backend.tokens[backend.cursor];
            if current.token_type() == VoidstarTokenTypes::CharLiteral{
                params.push(b'\'');
                params.extend_from_slice(&backend.source[current.start()..current.end()]);
                params.push(b'\'');
                
            } else { // we need the commas inside the array while we don't have a "parameter" struct
                params.extend_from_slice(&backend.source[current.start()..current.end()]);
            }

            backend.cursor += 1;
    }

    FunCall { ident, params }
}