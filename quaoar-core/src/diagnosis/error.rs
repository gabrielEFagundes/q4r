use core::fmt;

use crate::{
    diagnosis::{lexer_err::LexerErrOpts, parser_err::ParserErrOpts},
    die,
};

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
    pub ty: QErrorTypes,

    pub line: usize,
    pub pos: usize,

    pub panic_mode: bool,
}

impl QError {
    pub fn new(ty: QErrorTypes, line: usize, pos: usize, panic_mode: bool) -> Self {
        Self {
            ty,
            line,
            pos,
            panic_mode,
        }
    }

    /// Creates a new recoverable type `QError`.
    ///
    /// This is only used on specific cases.
    ///
    /// # Example
    /// ```
    /// /* ... */
    /// if current.token_type == VoidstarTokenTypes::Comma{
    ///     // This is used as a signal that the specific case is NOT an error,
    ///     // but rather a state that doesn't necessarily affect the program.
    ///
    ///     // e.g. the Comma is just a separator, which is ignored. This is the signal
    ///     // that tells the current token can be skipped safely.
    ///     QError::new_recoverable()
    /// }
    /// ```
    pub fn new_recoverable() -> Self {
        Self {
            ty: QErrorTypes::Recoverable,
            line: 0,
            pos: 0,
            panic_mode: false,
        }
    }

    pub fn evaluate_new_err(msg: fmt::Arguments, panic_mode: bool) -> ! {
        if panic_mode {
            panic!("{}", msg)
        } else {
            die!("{}", msg)
        }
    }
}
