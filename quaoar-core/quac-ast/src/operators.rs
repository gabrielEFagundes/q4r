use quac_core::tokens::Span;

/// The type of the operator, be it boolean or algebraic. Used on the `Operator` node.
/// 
#[derive(Clone)]
pub enum OperatorType{
    Plus, Minus, Multiplication, Div, 
    Lesser, Greater, LesserEquals, GreaterEquals, Equals
}

/// Operators are used mostly for expressions (especifically BinaryExpressions, like `1+1`, `2>1`, etc.)
/// 
/// The type and its span are what's stored as a node on the AST.
/// 
#[derive(Clone)]
pub struct Operator{
    pub ty: OperatorType,
    pub span: Span
}