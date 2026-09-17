use crate::signature::{DecKind, Literal, Operator, Type};

pub enum ExpType<'a>{ 
    OperativeExp(ExpOperator<'a>),
    LiteralExp(ExpLiteral),
    VariableExp(ExpVariable<'a>)
}

pub struct ExpOperator<'a>{
    pub left: &'a[u8],
    pub operator: Operator,
    pub right: &'a[u8]
}

pub struct ExpLiteral{
    pub val: Literal
}

pub struct ExpVariable<'a>{
    pub ty: Type,
    pub val: &'a[u8]
}

pub struct ComparisonDeclaration<'a>{
    pub kind: DecKind,
    pub expression: ExpType<'a>
}

pub struct VarDeclaration<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub val: ExpType<'a>,
}

pub struct FunDeclaration<'a>{
    pub returns: Type,
    pub ident: &'a[u8],
    pub params: Vec<VarDeclaration<'a>>
}