use ecow::EcoString;
use rustc_hash::FxHashMap;

use crate::{
    errors::error::QError,
    signature::{Signature, Type},
    tokens::VoidstarToken,
};

/// Used by the second stage of each backend (expdesc for C and the parser for QBE)
/// 
/// ## For the C backend
/// The C backend is a single-pass compiler, which means it does not mount a
/// traditional AST like the QBE backend does, it mounts "expression descriptions" (or `expdesc`),
/// which is what's used by the emitter to emit C code.
/// 
/// ## For the QBE backend
/// The QBE backend is a traditional compiler, with an AST and a parser,
/// the Backend struct is mostly used to access the source, tokens, cursor and errors.
pub struct Backend<'a> {
    pub source: &'a [u8],
    pub tokens: &'a [VoidstarToken],
    pub cursor: usize,

    pub signatures: FxHashMap<EcoString, Signature<'a>>, // identifier & signature
    pub scopes: Vec<FxHashMap<EcoString, Type>>,         // Identifier and type

    pub errors: Vec<QError>,
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
            errors: Vec::new(),
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
