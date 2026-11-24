use quac_core::{backend::{self, Backend}, internals::helpers::{advance, expect, expect_any}, tokens::Span};

use crate::prelude::{VoidstarTokenTypes, helpers::{current}};

pub fn parse_statement(backend: &mut Backend){
    match current(backend).token_type(){
        VoidstarTokenTypes::Int
        | VoidstarTokenTypes::Float
        | VoidstarTokenTypes::Bool
        | VoidstarTokenTypes::Char => {
            let ty = current(backend).span();
            expect(VoidstarTokenTypes::Ident, backend);

            let ident = current(backend).span();
            let mut value: Span;

            if expect_any(&[VoidstarTokenTypes::Equals, VoidstarTokenTypes::SemiColon], backend) == VoidstarTokenTypes::Equals{
                advance(1, backend);
                // TODO value
            }
        },

        _ => panic!("rust:")
    }
}