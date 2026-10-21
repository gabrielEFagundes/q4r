use crate::{errors::{lexer_err::LexerErrOpts, parser_err::ParserErrOpts}, tokens::Span};

/// Encompasses every and any type of deviation a Q4r program might have.
///
/// Those include recoverable states, warnings and errors.
///
/// You can consider this as a "global" error handler.
#[derive(PartialEq, Eq)]
pub enum QErrorTypes {
    /// Happens when, at any time of compilation, a
    /// deviation happens, but it is recoverable and requires
    /// no developer interaction whatsoever.
    Recoverable,

    /// Encompasses every lexer error.
    LexerErr(LexerErrOpts),
    /// Encompasses every parser error.
    ParserErr(ParserErrOpts),

    /// Encompasses every lexer warning.
    LexerWarn,
    /// Encompasses every parser warning.
    ParserWarn,
}

pub struct QError {
    pub span: Span,
    pub msg: String
}
