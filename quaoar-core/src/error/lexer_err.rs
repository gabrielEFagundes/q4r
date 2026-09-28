use core::fmt;

#[derive(PartialEq, Eq)]
pub enum LexerErrOpts{
    ImpossibleState,
    UnknownSymbol,
    BadDotSlice,
}

#[derive(PartialEq, Eq)]
pub struct LexerError{
    pub ty: LexerErrOpts,
    pub msg: String,
    pub line: usize,
    pub pos: usize
}

impl<'a> LexerError{
    pub fn new(ty: LexerErrOpts, line: usize, pos: usize) -> Self{
        Self { ty, msg: String::new(), line, pos }
    }

    pub fn handle_new(ty: LexerErrOpts, line: usize, pos: usize) -> Self{
        match ty{
            LexerErrOpts::ImpossibleState => Self::throw_lexer_err(
                ty, line, pos,
                format_args!("lexer reached an impossible state on line {}:{}", line, pos)
            ),
            LexerErrOpts::UnknownSymbol => Self::throw_lexer_err(
                ty, line, pos,
                format_args!("unknown symbol found on line {}:{}", line, pos)
            ),
            LexerErrOpts::BadDotSlice => Self::throw_lexer_err(
                ty, line, pos, 
                format_args!("found too many dots representing something on line {}:{}", line, pos)
            ),
        }
    }

    fn throw_lexer_err(ty: LexerErrOpts, line: usize, pos: usize, msg: fmt::Arguments) -> Self{
        Self { ty, msg: msg.to_string(), line, pos }
    }
}