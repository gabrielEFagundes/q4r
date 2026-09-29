use crate::{backend::Backend, tokens::{VoidstarToken, VoidstarTokenTypes}};

pub fn lookahead<'a>(backend: &mut Backend<'a>) -> VoidstarToken{
    if backend.cursor+1 < backend.tokens.len(){
        return backend.tokens[backend.cursor+1]
    }
    backend.tokens[backend.cursor]
}

pub fn current<'a>(backend: &mut Backend<'a>) -> VoidstarToken{
    backend.tokens[backend.cursor]
}

pub fn advance<'a>(amount: usize, backend: &mut Backend<'a>) -> VoidstarToken{
    backend.cursor += amount;
    current(backend)
}

/// Advances past the current token and compares with the expected.
pub fn expect<'a>(expected: VoidstarTokenTypes, backend: &mut Backend<'a>){
    backend.cursor += 1;
    if backend.cursor < backend.tokens.len() &&
        backend.tokens[backend.cursor].token_type != expected{ 
        panic!("found {:#?} instead of {:#?}", backend.tokens[backend.cursor].token_type, expected) 
    }
}

/// Compares the current token with the expected and, if the token types match, advances past it.
pub fn matches<'a>(expected: VoidstarTokenTypes, backend: &mut Backend<'a>){
    if backend.cursor < backend.tokens.len() &&
        backend.tokens[backend.cursor].token_type != expected{ 
        panic!("found {:#?} instead of {:#?}", backend.tokens[backend.cursor].token_type, expected) 
    }
    backend.cursor += 1;
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

pub fn skip_if_comma<'a>(backend: &mut Backend<'a>){
    if backend.tokens[backend.cursor].token_type == VoidstarTokenTypes::Comma{
        backend.cursor += 1;
    }
}

pub fn end<'a>(backend: &mut Backend<'a>) -> bool{
    backend.cursor >= backend.tokens.len()
}