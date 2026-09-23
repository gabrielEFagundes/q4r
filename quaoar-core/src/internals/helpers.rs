use crate::{backend::Backend, tokens::{VoidstarToken, VoidstarTokenTypes}};

pub fn lookahead<'a>(backend: &mut Backend<'a>) -> VoidstarToken{
    if backend.cursor+1 < backend.tokens.len(){
        return backend.tokens[backend.cursor+1]
    }
    backend.tokens[backend.cursor]
}

pub fn expect<'a>(expected: VoidstarTokenTypes, backend: &mut Backend<'a>){
    backend.cursor += 1;
    if backend.cursor < backend.tokens.len() &&
        backend.tokens[backend.cursor].token_type != expected{ 
        panic!("found {:#?} instead of {:#?}", backend.tokens[backend.cursor].token_type, expected) 
    }
}

pub fn expect_any<'a>(expected: &[VoidstarTokenTypes], backend: &mut Backend<'a>){
    backend.cursor += 1;
    if !expected.iter().any(|t| *t == backend.tokens[backend.cursor].token_type){
        panic!("found {:#?} instead of {:#?}", backend.tokens[backend.cursor].token_type, expected)
    }
}

#[inline]
pub fn expect_type<'a>(backend: &mut Backend<'a>){
    expect_any(
        &[
            VoidstarTokenTypes::Int, VoidstarTokenTypes::Float, 
            VoidstarTokenTypes::Char, VoidstarTokenTypes::Bool
        ],
        backend
    );
}

pub fn end<'a>(backend: &mut Backend<'a>) -> bool{
    backend.cursor >= backend.tokens.len()
}