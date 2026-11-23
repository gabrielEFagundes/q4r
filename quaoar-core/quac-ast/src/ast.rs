use crate::Statement;

/// The Abstract Syntax Tree the parser mounts using the source file and the lexer-generated tokens.
/// 
/// This is the base of the Quaoar backend (for QBEak), used to write scalable and efficient IL
pub struct Ast{
    pub nodes: Vec<Statement>
}