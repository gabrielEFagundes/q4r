use std::collections::HashMap;

use crate::{signature::{Signature, Type}, tokens::VoidstarToken};

pub struct Backend<'a, B>{
    pub source: &'a[u8],
    pub tokens: &'a[VoidstarToken],
    pub cursor: usize,

    pub signatures: HashMap<String, Signature>, // identifier & signature
    pub scopes: Vec<HashMap<String, Type>>,     // Identifier and type
    pub backend: B
}

impl<'a, B> Backend<'a, B>{
    pub fn new(source: &'a[u8], tokens: &'a[VoidstarToken], signatures: HashMap<String, Signature>, backend: B) -> Self{
        Self { source, tokens, cursor: 0, signatures, scopes: Vec::new(), backend }
    }

    pub fn enter_scope(&mut self){
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self){
        self.scopes.pop();
    }

    pub fn declare_in_scope(&mut self, ident: String, ty: Type){
        self.scopes.last_mut().unwrap().insert(ident, ty);
    }

    pub fn resolve_in_scope(&mut self, ident: &str) -> Option<&Type>{
        self.scopes.iter().rev()
            .find_map(|s| s.get(ident))
            .or_else(|| {
                let sign = self.signatures.get(ident).unwrap();
                Some(&Signature::destructure_glob(sign).0).map(|v| &**v)
            })
    }
}