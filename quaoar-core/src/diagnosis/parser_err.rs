#[derive(PartialEq, Eq)]
pub enum ParserErrOpts{
    ImpossibleState,
    UnknownSymbol,
    UnknownType,
    IllegalArgument,
    UnexpectedToken,
}