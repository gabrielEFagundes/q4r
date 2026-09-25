use core::fmt;

/// Compiler errors are treated differently from the core errors, because
/// each compiler have their own errors which are handled differently depending
/// on the backend. 
pub enum CompilerErr{
    
}

pub struct CompilerError{
    pub ty: CompilerErr,
    pub msg: String,
    pub line: usize,
    pub pos: usize
}

/// The `CompilerErrorTrait` is a trait implemented by the compiler backends that
/// hands over a few useful methods to define and throw errors.
/// 
/// Not implemented yet as of version 0.1, which is why it's incomplete.
#[allow(unused)]
pub trait CompilerErrorTrait{
    fn new(ty: CompilerErr, line: usize, pos: usize) -> CompilerError{
        match ty{
            
        }
    }

    fn throw_compiler_err(ty: CompilerErr, line: usize, pos: usize, msg: fmt::Arguments) -> CompilerError{
        CompilerError { ty, msg: msg.to_string(), line, pos }
    }
}