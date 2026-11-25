use ecow::EcoVec;

use crate::{Expression, Statement};

/// The Abstract Syntax Tree is the tree the parser mounts using the source file and the lexer-generated tokens.
/// 
/// This is the base of the Quaoar backend (for QBEak), used to write scalable and efficient IL
/// 
/// ## Fields
/// - exprs: EcoVec of expressions used for storing the expressions' positions.
/// These are then used for traversing and finding expressions on the QBE compiler and emitter.
/// 
/// - nodes: EcoVec of statements that compose a Quaoar program. Used for posterior
/// traversal by the QBE compiler.
pub struct Ast{
    pub exprs: EcoVec<Expression>,
    pub nodes: EcoVec<Statement>
}