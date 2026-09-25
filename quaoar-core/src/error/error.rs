use crate::{die, error::{lexer_err::{LexerErr, LexerError}, parser_err::{ParserErr, ParserError}}};

/// Encompasses every and any type of deviation a Q4r program might have.
/// 
/// Those include recoverable states, warnings and errors.
/// 
/// You can consider this as a "global" error handler.
#[derive(PartialEq, Eq)]
pub enum QErrorTypes{
    /// Happens when, at any time of compilation, a
    /// deviation happens, but it is recoverable and requires
    /// no developer interaction whatsoever.
    Recoverable,

    /// Encompasses every lexer error.
    LexerErr(LexerErr),
    /// Encompasses every parser error.
    ParserErr(ParserErr),

    /// Encompasses every lexer warning.
    LexerWarn,
    /// Encompasses every parser warning.
    ParserWarn,
}

pub struct QError{
    pub ty: QErrorTypes,

    pub line: usize,
    pub pos: usize,

    pub panic_mode: bool
}

impl<'a> QError{
    pub fn new(ty: QErrorTypes, line: usize, pos: usize, panic_mode: bool) -> Self{
        Self { ty, line, pos, panic_mode }
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
    pub fn new_recoverable() -> Self{
        Self { ty: QErrorTypes::Recoverable, line: 0, pos: 0, panic_mode: false }
    }

    pub fn handle_new_error(ty: QErrorTypes, line: usize, pos: usize, panic_mode: bool){
        match ty{
            QErrorTypes::LexerErr(lexer_err) => Self::handle_lexer_err(
                LexerError::new(lexer_err, line, pos), panic_mode
            ),
            QErrorTypes::ParserErr(parser_err) => Self::handle_parser_err(
                ParserError::new(parser_err, line, pos), panic_mode
            ),

            QErrorTypes::LexerWarn => todo!("warnings not implemented yet"),
            QErrorTypes::ParserWarn => todo!("warnings not implemented yet"),
            _ => return
        }
    }

    fn handle_lexer_err(error: LexerError, panic_mode: bool){
        let msg = LexerError::handle_new(error.ty, error.line, error.pos).msg;

        if panic_mode{ panic!("{}", msg) }
        else{ die!("{}", msg) }
    }

    fn handle_parser_err(error: ParserError, panic_mode: bool){
        let msg = ParserError::handle_new(error).msg;

        if panic_mode{ panic!("{}", msg) }
        else{ die!("{}", msg); }
    }
}