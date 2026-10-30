use crate::{
    backend::Backend, 
    expdesc::Parameter, 
    internals::helpers::{self, advance, current}, 
    signature::Type, 
    tokens::VoidstarTokenTypes,
};

pub fn declare_parameter<'a>(backend: &mut Backend<'a>) -> Parameter<'a> {
    match current(backend).token_type {
        VoidstarTokenTypes::Asterisk => {
            helpers::expect_type(backend);

            let ty = Type::map(current(backend).token_type).map_pointer();

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            let ident = &backend.source[current(backend).start()..current(backend).end()];
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
            let ty = Type::map(current(backend).token_type);

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            let ident = &backend.source[current(backend).start()..current(backend).end()];
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
            helpers::add_error("unknown symbol found here", backend);
            Parameter::default()
        }
    }
}
