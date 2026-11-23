use quac_core::tokens::Span;

use crate::{expressions::BinaryExpression, statements::{BlockStatement, ConditionalStatement, IncrementalStatement, ParamStatement, VarStatement}};

pub mod statements;
pub mod expressions;
pub mod operators;

pub type ExprId = usize;

/// Statements are the base of Quaoar, they are the second level of the AST.
pub enum Statement{
    VarStatement(VarStatement),

    /// e.g. `for i = 0; i < 10; +1`
    IncrementalStatement(IncrementalStatement),
    /// e.g. `for i < 10` or `if i < 10`
    ConditionalStatement(ConditionalStatement),

    ParamStatement(ParamStatement),
    FunctionStatement(BlockStatement),

    BadStatement
}

/// Expressions are the third level of the AST, and they are the base for most operations.
pub enum Expression{
    LiteralExpression(Span),
    BinaryExpression(BinaryExpression),

    BadExpression
}