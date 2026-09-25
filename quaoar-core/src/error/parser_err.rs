use core::fmt;

#[derive(PartialEq, Eq)]
pub enum ParserErr{
    ImpossibleState,
    UnknownSymbol,
}

#[derive(PartialEq, Eq)]
pub struct ParserError{
    pub ty: ParserErr,
    pub msg: String,
    pub line: usize,
    pub pos: usize
}

impl<'a> ParserError{
    pub fn new(ty: ParserErr, line: usize, pos: usize) -> Self{
        Self { ty, msg: String::new(), line, pos }
    }

    pub fn handle_new(self) -> Self{
        match self.ty {
            ParserErr::ImpossibleState => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("parser reached an impossible state on line {}:{}", self.line, self.pos)
            ),
            ParserErr::UnknownSymbol => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("unknown symbol found on line {}:{}", self.line, self.pos)
            ),
        }
    }

    pub fn throw_parser_err(ty: ParserErr, line: usize, pos: usize, msg: fmt::Arguments) -> Self{
        Self { ty, msg: msg.to_string(), line, pos }
    }
}