use crate::{backend::Backend, error::{error::{self, QError}, parser_err::ParserErr}, expdesc::Parameter, internals::helpers, signature::Type, tokens::VoidstarTokenTypes};

pub fn declare_parameter<'a>(backend: &mut Backend<'a>) -> Result<Parameter<'a>, error::QError>{
    let mut current = backend.tokens[backend.cursor];
    match current.token_type{
        VoidstarTokenTypes::Asterisk => {
            helpers::expect_type(backend);

            let mut ty = Type::map(backend.tokens[backend.cursor].token_type);
            ty = ty.map_pointer();

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            current = backend.tokens[backend.cursor];
            let ident = &backend.source[current.start..current.end];
            backend.cursor += 1;

            Ok(Parameter { ty, ident, is_etc: false })
        },

        VoidstarTokenTypes::Int
        | VoidstarTokenTypes::Float
        | VoidstarTokenTypes::Char
        | VoidstarTokenTypes::Bool => {
            let ty = Type::map(current.token_type);

            helpers::expect(VoidstarTokenTypes::Ident, backend);

            current = backend.tokens[backend.cursor];
            let ident = &backend.source[current.start..current.end];
            backend.cursor += 1;

            Ok(Parameter { ty, ident, is_etc: false })
        },

        // ty is ignored here, as typeless ellipsis excludes it.
        VoidstarTokenTypes::Ellipsis => {
            helpers::expect(VoidstarTokenTypes::CloseParents, backend);
            Ok(Parameter { ty: Type::Void, ident: b"...", is_etc: true })
        },

        VoidstarTokenTypes::Comma => {
            backend.cursor += 1;
            Err(error::QError::new_recoverable())
        }

        _ => Err(QError::new(
            error::QErrorTypes::ParserErr(ParserErr::UnknownSymbol), 
            /*todo*/ 0, backend.tokens[backend.cursor].start, backend.debug
        ))
    }
}