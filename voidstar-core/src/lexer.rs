use core::panic;
use std::ops::Add;

use crate::{tokens::{VoidstarToken, VoidstarTokenTypes}, *};

#[allow(unused)]
pub struct Lexer{
    source: Vec<u8>,
    line: usize,
    cursor: usize,
    current: u8
}

impl Lexer{
    pub fn new(source: Vec<u8>) -> Self{
        Self { source, line: 1, cursor: 0, current: 0 }
    }

    fn advance(&mut self) -> u8{
        if self.source.len() > self.cursor + 1{ self.cursor += 1; }
        self.current = self.source[self.cursor];
        self.source[self.cursor]
    }

    fn next(&self) -> u8{
        if self.cursor+1 > self.source.len(){ return self.source[self.cursor]; }
        self.source[self.cursor+1]
    }

    fn symbol(&mut self) -> VoidstarToken{
        match self.current{
            QUOTES => {
                todo!("strings are not implemented yet.");
            },

            APOSTROPHE => {
                self.advance();
                let v = self.current as char;
                return VoidstarTokenTypes::new(VoidstarTokenTypes::Char, v.to_string());
            },
            _ => {
                match SYMBOLS.binary_search_by(|&(k, _)| k.cmp(&(self.current as char)))
                        .ok()
                        .and_then(|i| SYMBOLS.get(i)){
                            Some(token) => {
                                self.advance();
                                VoidstarTokenTypes::new(token.1, token.0.to_string())
                            },
                            None => panic!("unknown symbol at {}, line {}", self.current, self.line)
                        }
            }
        }
    }

    fn keyword(&mut self) -> VoidstarToken{
        let mut value: String = String::new();
        while self.current.is_ascii_alphabetic(){
            value.push(self.current as char);
            self.advance();
        }
        
        match KEYWORDS.binary_search_by(|&(k, _)| k.cmp(&value))
                .ok()
                .and_then(|i| KEYWORDS.get(i)){
                    Some(token) => VoidstarTokenTypes::new(token.1, token.0.to_string()),
                    None => VoidstarTokenTypes::new(VoidstarTokenTypes::Ident, value),
        }
    }

    fn digit(&mut self) -> VoidstarToken{
        let mut value = String::new();

        'integer: {
            while self.current.is_ascii_digit(){
                value.push(self.current as char);
                self.advance();

                if self.current == DOT{
                    break 'integer;
                }
            }
            return VoidstarTokenTypes::new(VoidstarTokenTypes::IntLiteral, value);
        }
        'float: {
            if self.next() == SPACE || self.next() == LINE_FEED || self.next() == TABULATION{
                break 'float;
            }
            value.push(self.current as char);
            self.advance();

            while self.current.is_ascii_digit(){
                value.push(self.current as char);
                self.advance();
            }
            return VoidstarTokenTypes::new(VoidstarTokenTypes::FloatLiteral, value);
        }

        panic!("invalid literal {}", value);
    }

    pub fn lexerize(&mut self) -> Vec<VoidstarToken>{
        let mut tokens: Vec<VoidstarToken> = Vec::new();
        self.current = self.source[self.cursor];

        while self.cursor < self.source.len()-1{
            match self.current{
                OPEN_PAREN | CLOSE_PAREN | OPEN_BRACE | CLOSE_BRACE | OPEN_BRACKET | CLOSE_BRACKET |
                DOT | COMMA | COLON | SEMICOL | EQ | NOT | ASTRSK | QUOTES | APOSTROPHE => {
                    tokens.push(self.symbol());
                },

                val if val.is_ascii_alphabetic() => tokens.push(self.keyword()),
                val if val.is_ascii_digit() => tokens.push(self.digit()),

                LINE_FEED => {
                    self.line+=1;
                    self.advance();
                    continue;
                },
                TABULATION | SPACE => {
                    self.advance();
                    continue;
                },

                HALT => tokens.push(VoidstarTokenTypes::new(VoidstarTokenTypes::Halt, String::new())),
                _ => panic!("lexer reached an impossible state on byte `{}`!", self.source[self.cursor])
            }
            self.advance();
            println!("{:#?}", tokens);
        }

        tokens
    }
}