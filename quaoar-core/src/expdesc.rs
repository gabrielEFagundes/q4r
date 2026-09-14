use crate::signatures::{DecKind, Operator, Type};

pub struct Expression<'a>{
    pub left: &'a[u8],
    pub operator: Operator,
    pub right: &'a[u8]
}

pub struct ComparisonDeclaration<'a>{
    pub kind: DecKind,
    pub expression: Expression<'a>
}

pub struct VarDeclaration<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub val: &'a[u8],
}

pub struct FunDeclaration<'a>{
    pub returns: Type,
    pub ident: &'a[u8],
    pub params: Vec<&'a[u8]>
}