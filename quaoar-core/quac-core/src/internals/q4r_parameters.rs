use crate::{
    backend::Backend, errors::error::QError, expdesc::Parameter, internals::helpers::{self, advance}, signature::Type, tokens::VoidstarTokenTypes,
};

pub fn declare_parameter<'a>(backend: &mut Backend<'a>) -> Parameter<'a> {
    let mut current = backend.tokens[backend.cursor];
    match current.token_type {
        VoidstarTokenTypes::Asterisk => {
            helpers::expect_type(backend);

            let ty = Type::map(current.token_type).map_pointer();

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            current = backend.tokens[backend.cursor];
            let ident = &backend.source[current.start()..current.end()];
            advance(1, backend);

            Parameter {
                ty,
                ident,
                is_etc: false,
            }
        }

        VoidstarTokenTypes::Int
        | VoidstarTokenTypes::Float
        | VoidstarTokenTypes::Char
        | VoidstarTokenTypes::Bool => {
            let ty = Type::map(current.token_type);

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            current = backend.tokens[backend.cursor];
            let ident = &backend.source[current.start()..current.end()];
            advance(1, backend);

            Parameter {
                ty,
                ident,
                is_etc: false,
            }
        }

        // ty is ignored here, as typeless ellipsis excludes it.
        VoidstarTokenTypes::Ellipsis => {
            helpers::expect(VoidstarTokenTypes::CloseParents, backend);
            Parameter {
                ty: Type::Void,
                ident: b"...",
                is_etc: true,
            }
        }

        VoidstarTokenTypes::Comma => {
            backend.cursor += 1;
            declare_parameter(backend)
        }

        _ => {
            QError{
                span: current.span,
                msg: "Unknown symbol found here".to_string()
            };
            helpers::goto_safety(backend);
            
            Parameter::default()
        }
    }
}
