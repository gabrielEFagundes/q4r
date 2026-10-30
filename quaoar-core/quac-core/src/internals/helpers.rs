use crate::{
    backend::Backend, 
    errors::error::QError, 
    tokens::{
        VoidstarToken, 
        VoidstarTokenTypes
    },
};

pub fn lookahead<'a>(backend: &mut Backend<'a>) -> VoidstarToken {
    if backend.cursor + 1 < backend.tokens.len() {
        return backend.tokens[backend.cursor + 1];
    }
    backend.tokens[backend.cursor]
}

#[inline]
pub fn current<'a>(backend: &mut Backend<'a>) -> VoidstarToken {
    backend.tokens[backend.cursor]
}

#[inline]
pub fn advance<'a>(amount: usize, backend: &mut Backend<'a>) {
    backend.cursor += amount;
}

/// Advances past the current token and compares with the expected.
pub fn expect<'a>(expected: VoidstarTokenTypes, backend: &mut Backend<'a>) {
    backend.cursor += 1;
    if backend.cursor < backend.tokens.len()
        && current(backend).token_type != expected
    {
        add_error(format!("found {:#?} instead of {:#?}", current(backend).token_type, expected).as_str(), backend);
    }
}

/// Compares the current token with the expected and, if the token types match, advances past it.
pub fn matches<'a>(expected: VoidstarTokenTypes, backend: &mut Backend<'a>) {
    if backend.cursor < backend.tokens.len()
        && backend.tokens[backend.cursor].token_type != expected
    {
        panic!(
            "found {:#?} instead of {:#?}",
            backend.tokens[backend.cursor].token_type, expected
        )
    }
    advance(1, backend);
}

pub fn expect_any<'a>(expected: &[VoidstarTokenTypes], backend: &mut Backend<'a>) {
    advance(1, backend);
    if !expected
        .iter()
        .any(|t| *t == backend.tokens[backend.cursor].token_type)
    {
        let current = current(backend);
        backend.errors.push(QError{
            span: current.span,
            msg: format_args!("next is found to be {:#?} instead of {:#?}", current.token_type, expected).to_string()
        });
    }
}

pub fn matches_any<'a>(expected: &[VoidstarTokenTypes], backend: &mut Backend<'a>) {
    if !expected
        .iter()
        .any(|t| *t == backend.tokens[backend.cursor].token_type)
    {
        let current = current(backend);
        backend.errors.push(QError{
            span: current.span,
            msg: format_args!("current is found to be {:#?} instead of {:#?}", current.token_type, expected).to_string()
        });
    }
    advance(1, backend);
}

#[inline]
pub fn expect_type<'a>(backend: &mut Backend<'a>) {
    expect_any(
        &[
            VoidstarTokenTypes::Int,
            VoidstarTokenTypes::Float,
            VoidstarTokenTypes::Char,
            VoidstarTokenTypes::Bool,
        ],
        backend,
    );
}

pub fn skip_if_comma<'a>(backend: &mut Backend<'a>) {
    if current(backend).token_type == VoidstarTokenTypes::Comma {
        backend.cursor += 1;
    }
}

// maybe use closures here instead of if statements?
#[inline]
pub fn add_error<'a>(msg: &str, backend: &mut Backend<'a>){
    let current = current(backend);
    backend.errors.push(QError { span: current.span, msg: msg.to_string() });
    goto_safety(backend);
}

#[inline]
pub fn ok_or_report<'a, T>(backend: &mut Backend<'a>, r: Result<T, &str>) -> Option<T>{
    r.map_err(|msg| add_error(msg, backend)).ok()
}

pub fn goto_safety<'a>(backend: &mut Backend<'a>){
    while !end(backend)
        && current(backend).token_type != VoidstarTokenTypes::SemiColon
        && current(backend).token_type != VoidstarTokenTypes::CloseParents
        && current(backend).token_type != VoidstarTokenTypes::CloseBraces
    {
        advance(1, backend);
    }
}

pub fn end<'a>(backend: &mut Backend<'a>) -> bool {
    backend.cursor >= backend.tokens.len()
}
