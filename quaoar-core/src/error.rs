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
    LexerErr,
    /// Encompasses every parser error.
    ParserErr,
    /// Encompasses every compiler error.
    CompilerErr,

    /// Encompasses every lexer warning.
    LexerWarn,
    /// Encompasses every parser warning.
    ParserWarn,
    /// Encompasses every compiler warning.
    CompilerWarn
}

pub struct QError<'a>{
    pub ty: QErrorTypes,
    pub msg: &'a str
}

impl<'a> QError<'a>{
    /// Creates a new `QError`
    pub fn new(ty: QErrorTypes, msg: &'a str) -> Self{
        Self { ty, msg }
    }

    /// Creates a new recoverable type `QError`.
    pub fn new_recoverable() -> Self{
        Self { ty: QErrorTypes::Recoverable, msg: "" }
    }
}