use crate::signature::{DecKind, Literal, Operator, Type};

#[derive(Debug)]
pub enum ExpType<'a> {
    BinaryExp {
        left: Box<ExpType<'a>>,
        operator: Operator,
        right: Box<ExpType<'a>>,
    },

    LiteralExp {
        val: Literal,
    },

    CallExp {
        ident: &'a [u8],
        params: Vec<ExpType<'a>>,
    },

    AddressExp {
        val: Box<ExpType<'a>>,
    },

    SignedExp {
        op: Operator,
        val: Box<ExpType<'a>>,
    },
}

pub enum StmtType<'a> {
    VarStmt(VarStmt<'a>),

    AssignStmt(AssignStmt<'a>),

    AssignOpStmt(AssignOpStmt<'a>),

    FunStmt(FunStmt<'a>),

    LoopStmt(LoopStmt<'a>),

    RelationalStmt(RelationalStmt<'a>),
}

impl<'a> StmtType<'a> {
    pub fn var_stmt(ty: Type, ident: &'a [u8], val: ExpType<'a>) -> Self {
        Self::VarStmt(VarStmt { ty, ident, val })
    }

    pub fn assign_stmt(ident: &'a [u8], val: ExpType<'a>) -> Self {
        Self::AssignStmt(AssignStmt { ident, val })
    }

    pub fn assign_op_stmt(ident: &'a [u8], op: Operator, val: ExpType<'a>) -> Self {
        Self::AssignOpStmt(AssignOpStmt { ident, op, val })
    }

    pub fn fun_stmt(
        returns: Type,
        ident: &'a [u8],
        params: Vec<Parameter<'a>>,
        is_extern: bool,
    ) -> Self {
        Self::FunStmt(FunStmt {
            returns,
            ident,
            params,
            is_extern,
        })
    }

    pub fn loop_stmt(
        kind: DecKind,
        initializer: AssignStmt<'a>,
        exp: ExpType<'a>,
        incrementer: Literal,
    ) -> Self {
        Self::LoopStmt(LoopStmt {
            kind,
            initializer,
            exp,
            incrementer,
        })
    }

    pub fn relational_stmt(kind: DecKind, exp: ExpType<'a>) -> Self {
        Self::RelationalStmt(RelationalStmt { kind, exp })
    }
}

pub struct VarStmt<'a> {
    pub ty: Type,
    pub ident: &'a [u8],
    pub val: ExpType<'a>,
}

pub struct AssignStmt<'a> {
    pub ident: &'a [u8],
    pub val: ExpType<'a>,
}

pub struct AssignOpStmt<'a> {
    pub ident: &'a [u8],
    pub op: Operator,
    pub val: ExpType<'a>,
}

pub struct FunStmt<'a> {
    pub returns: Type,
    pub ident: &'a [u8],
    pub params: Vec<Parameter<'a>>,
    pub is_extern: bool,
}

pub struct Parameter<'a> {
    pub ty: Type,
    pub ident: &'a [u8],
    pub is_etc: bool,
}

pub struct LoopStmt<'a> {
    pub kind: DecKind,
    pub initializer: AssignStmt<'a>,
    pub exp: ExpType<'a>,
    pub incrementer: Literal,
}

pub struct RelationalStmt<'a> {
    pub kind: DecKind,
    pub exp: ExpType<'a>,
}
