use crate::{ExprId, operators::Operator};

/// Binary Expressions can be one of two things:
/// 
/// - A literal binary expression, like `2+2`
/// - Conditionals, like `2>1`
/// 
/// Quaoar's AST doesn't differ them because there's no need to!
/// 
/// Using a single expression type for both boolean and algebraic expressions
/// helps organize the ast, and lowers overhead of nodes.
pub struct BinaryExpression{
    pub left: ExprId,
    pub operator: Operator,
    pub right: ExprId
}