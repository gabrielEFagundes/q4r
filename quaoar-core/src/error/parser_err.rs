use core::fmt;

#[derive(PartialEq, Eq)]
pub enum ParserErrOpts{
    ImpossibleState,
    UnknownSymbol,
    UnknownType,
    IllegalArgument,
    UnexpectedToken,
}

#[derive(PartialEq, Eq)]
pub struct ParserError{
    pub ty: ParserErrOpts,
    pub msg: String,
    pub line: usize,
    pub pos: usize
}

impl<'a> ParserError{
    pub fn new(ty: ParserErrOpts, line: usize, pos: usize) -> Self{
        Self { ty, msg: String::new(), line, pos }
    }

    pub fn handle_new(self) -> Self{
        match self.ty {
            ParserErrOpts::ImpossibleState => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("parser reached an impossible state on line {}:{}", self.line, self.pos)
            ),
            ParserErrOpts::UnknownSymbol => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("unknown symbol found on line {}:{}", self.line, self.pos)
            ),
            ParserErrOpts::IllegalArgument => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("illegal argument found on line {}:{}", self.line, self.pos)
            ),
            ParserErrOpts::UnknownType => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("unknown type found on line {}:{}", self.line, self.pos)
            ),
            ParserErrOpts::UnexpectedToken => Self::throw_parser_err(
                self.ty, self.line, self.pos, 
                format_args!("unexpected token on line: {}:{}", self.line, self.pos)
            ),
        }
    }

    pub fn throw_parser_err(ty: ParserErrOpts, line: usize, pos: usize, msg: fmt::Arguments) -> Self{
        Self { ty, msg: msg.to_string(), line, pos }
    }
}