use crate::signatures::Type;

// pub struct Expression{
//     pub left: Literal,
//     pub right: Literal
// }

pub struct VarDeclaration<'a>{
    pub ty: Type,
    pub ident: &'a[u8],
    pub val: &'a[u8],
}

pub struct FunDeclaration<'a>{
    pub returns: Type,
    pub ident: &'a[u8],
    pub params: Vec<&'a[u8],>
}