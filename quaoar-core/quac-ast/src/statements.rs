use ecow::EcoVec;
use quac_core::tokens::Span;

use crate::{Expression, operators::Operator};

/// Variables are used mainly to store values inside an identifier, for later usage.
/// 
/// When a variable is declared, it's type, identifier and value are stored as spans, which are then resolved by the semantic checker.
pub struct VarStatement{
    pub ty: Span,
    pub ident: Span,
    pub value: Span
}

/// Conditional Statements are used mainly for conditional operations, at moments you need to decide a path to take, based on a value, type, etc.
/// 
/// Those can be either `if` stataments or conditional `for` statements.
/// 
/// When a conditional statement is declared, the left and right expressions are stored, as well as the operator's type and span.
pub struct ConditionalStatement{
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression
}

/// Incremental Statements are used mainly for `for` loops.
/// 
/// When an incremental statement is declared, the initializer (a variable), its condition (ConditionalStatement) and its incrementer are stored,
/// essentially as Spans.
pub struct IncrementalStatement{
    pub initializer: VarStatement,
    pub condition: ConditionalStatement,
    pub incrementer: Span
}

/// Parameters are used mainly on function statements and calls, though this specific node is for function statements.
/// 
/// When a parameter is declared, its identifier and type are stored.
pub struct ParamStatement{
    pub ident: Span,
    pub ty: Span
}

/// Functions are reusable blocks of code used so you don't have to rewrite repetitive code.
/// 
/// Quaoar is a general purpose language, and it uses functions as its main basis to compile correctly.
/// The only time you won't use a function in Quaoar is on the `main` function and for structs, enums, globals, etc,
/// but most of the time, you will be writing logic blocks, as that's the purpose of a programming language.
/// 
/// When a function is declared, it's identifier and type are stored as spans, while its parameters are stored inside an EcoVec (as of v0.2)
pub struct BlockStatement{
    pub ident: Span,
    pub params: EcoVec<ParamStatement>,
    pub ty: Span
}