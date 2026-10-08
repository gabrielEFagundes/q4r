use ecow::EcoString;
use rustc_hash::FxHashMap;

use crate::{
    signature::{Signature, Type},
    tokens::VoidstarToken,
};

pub struct Backend<'a> {
    pub source: &'a [u8],
    pub tokens: &'a [VoidstarToken],
    pub cursor: usize,

    pub signatures: FxHashMap<EcoString, Signature<'a>>, // identifier & signature
    pub scopes: Vec<FxHashMap<EcoString, Type>>,         // Identifier and type

    pub debug: bool,
}

impl<'a> Backend<'a> {
    pub fn new(
        source: &'a [u8],
        tokens: &'a [VoidstarToken],
        signatures: FxHashMap<EcoString, Signature<'a>>,
        debug: bool,
    ) -> Self {
        Self {
            source,
            tokens,
            cursor: 0,
            signatures,
            scopes: Vec::new(),
            debug,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(FxHashMap::default());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_in_scope(&mut self, ident: EcoString, ty: Type) {
        self.scopes.last_mut().unwrap().insert(ident, ty);
    }

    pub fn resolve_in_scope(&mut self, ident: &str) -> Option<&Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.get(ident))
            .or_else(|| {
                let sign = self.signatures.get(ident).unwrap();
                Some(&Signature::destructure_glob(sign).0).map(|v| &**v)
            })
    }
}
