use quac_core::{errors::error::QError, tokens::VoidstarToken};

pub struct QuacParser<'a>{
    pub src: &'a[u8],
    pub tokens: &'a[VoidstarToken],
    pub errors: Vec<QError>,
}

impl<'a> QuacParser<'a>{
    pub fn new(src: &'a[u8], tokens: &'a[VoidstarToken]) -> Self{
        QuacParser { src, tokens, errors: Vec::new() }
    }
}