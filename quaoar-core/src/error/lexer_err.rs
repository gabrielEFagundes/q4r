use core::fmt;

#[derive(PartialEq, Eq)]
pub enum LexerErr{
    ImpossibleState,
    UnknownSymbol,
    BadDotSlice,
}

#[derive(PartialEq, Eq)]
pub struct LexerError{
    pub ty: LexerErr,
    pub msg: String,
    pub line: usize,
    pub pos: usize
}

impl<'a> LexerError{
    pub fn new(ty: LexerErr, line: usize, pos: usize) -> Self{
        Self { ty, msg: String::new(), line, pos }
    }

    pub fn handle_new(ty: LexerErr, line: usize, pos: usize) -> Self{
        match ty{
            LexerErr::ImpossibleState => Self::throw_lexer_err(
                ty, line, pos,
                format_args!("lexer reached an impossible state on line {}:{}", line, pos)
            ),
            LexerErr::UnknownSymbol => Self::throw_lexer_err(
                ty, line, pos,
                format_args!("unknown symbol found on line {}:{}", line, pos)
            ),
            LexerErr::BadDotSlice => Self::throw_lexer_err(
                ty, line, pos, 
                format_args!("found too many dots representing something on line {}:{}", line, pos)
            ),
        }
    }

    fn throw_lexer_err(ty: LexerErr, line: usize, pos: usize, msg: fmt::Arguments) -> Self{
        Self { ty, msg: msg.to_string(), line, pos }
    }
}