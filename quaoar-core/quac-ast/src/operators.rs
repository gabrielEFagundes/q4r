use quac_core::tokens::Span;

/// The type of the operator, be it boolean or algebraic. Used on the `Operator` node.
/// 
#[derive(Clone)]
pub enum OperatorType{
    Plus, Minus, Multiplication, Div, 
    Lesser, Greater, LesserEquals, GreaterEquals, Equals
}

impl OperatorType{
    /// Takes the operator's precedence based on its type.
    /// 
    /// - Plus, Minus and all the boolean operators are of precedence 10
    /// - Multiplication and Division are of precedence 20
    pub fn get_operator_precedence(&self) -> usize{
        match self{
            OperatorType::Plus => 10,
            OperatorType::Minus => 10,
            OperatorType::Multiplication => 20,
            OperatorType::Div => 20,
            OperatorType::Lesser => 10,
            OperatorType::Greater => 10,
            OperatorType::LesserEquals => 10,
            OperatorType::GreaterEquals => 10,
            OperatorType::Equals => 10,
        }
    }
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