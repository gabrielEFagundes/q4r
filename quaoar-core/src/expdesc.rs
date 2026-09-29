use crate::signature::{DecKind, Literal, Operator, Type};

#[derive(Debug)]
pub enum ExpType<'a>{ 
    BinaryExp{
        left: Box<ExpType<'a>>,
        operator: Operator,
        right: Box<ExpType<'a>>
    },

    LiteralExp{
        val: Literal
    },

    CallExp{
        ident: &'a[u8],
        params: Vec<ExpType<'a>>
    },

    AddressExp{
        val: Literal
    }
}

pub enum StmtType<'a>{
    VarStatement{
        ty: Type,
        ident: &'a[u8],
        val: ExpType<'a>,
    },

    FunStatement{
        returns: Type,
        ident: &'a[u8],
        params: Vec<Parameter<'a>>,
        is_extern: bool
    },

    ForLoopStmt{
        kind: DecKind,
        iterator: &'a[u8],
        initializer: ExpType<'a>,
        exp: ExpType<'a>,
        incrementer: Literal
    },

    RelationalStmt{
        kind: DecKind,
        expression: ExpType<'a>
    },
}

#[derive(Debug)]
pub struct Parameter<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub is_etc: bool
}