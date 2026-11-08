#[derive(PartialEq, Eq)]
pub enum LexerErrOpts {
    ImpossibleState,
    UnknownSymbol,
    BadDotSlice,
}
