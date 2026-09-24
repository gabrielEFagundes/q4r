use crate::signature::{DecKind, Literal, Operator, Type};

#[derive(Debug)]
pub enum ExpType<'a>{ 
    OperativeExp(ExpOperator<'a>),
    LiteralExp(ExpLiteral),
    CallExp(FunCall<'a>),
    AddressExp(ExpLiteral)
}

pub enum DeclType<'a>{
    ForLoopDecl(ForLoopDeclaration<'a>),
    WhileLoopDecl(ComparisonDeclaration<'a>),
}

#[derive(Debug)]
pub struct ExpOperator<'a>{
    pub left: &'a[u8],
    pub operator: Operator,
    pub right: &'a[u8]
}

#[derive(Debug)]
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

pub struct ForLoopDeclaration<'a>{
    pub kind: DecKind,
    pub iterator: &'a[u8],
    pub initializer: ExpType<'a>,
    pub exp: ExpType<'a>,
    pub incrementer: Literal
}

#[derive(Debug)]
pub struct VarDeclaration<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub val: ExpType<'a>,
}

#[derive(Debug)]
pub struct Parameter<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub is_etc: bool
}

#[derive(Debug)]
pub struct FunDeclaration<'a>{
    pub returns: Type,
    pub ident: &'a[u8],
    pub params: Vec<Parameter<'a>>,
    pub is_extern: bool
}

#[derive(Debug)]
pub struct FunCall<'a>{
    pub ident: &'a[u8],
    pub params: Vec<ExpType<'a>>
}