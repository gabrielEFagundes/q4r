use core::panic;

use crate::{tokens::{VoidstarToken, VoidstarTokenTypes}, *};

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

    fn end(&self) -> bool{
        self.cursor >= self.source.len()
    }

    fn advance(&mut self) -> u8{
        self.cursor+=1;
        if self.cursor < self.source.len(){ self.current = self.source[self.cursor]; }
        self.current
    }

    fn advance_this(&mut self, amount: usize) -> u8{
        self.cursor+=amount;
        if self.cursor < self.source.len(){ self.current = self.source[self.cursor]; }
        self.current
    }

    fn symbol(&mut self) -> VoidstarToken{
        let start = self.cursor; let mut end = self.cursor;
        match self.current{
            QUOTES => {
                todo!("strings are not implemented yet.");
            },

            APOSTROPHE => {
                self.advance();
                let v = self.cursor;
                self.advance_this(2);
                return VoidstarToken::new(VoidstarTokenTypes::CharLiteral, v, v+1);
            },

            GREATER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::GreaterEq, start, end)
                    },
                    _ => {
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::Greater, start, end)
                    }
                }
            },
            LESSER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::LesserEq, start, end)
                    },
                    _ => {
                        VoidstarToken::new(VoidstarTokenTypes::Lesser, start, self.cursor)
                    }
                }
            },
            PLUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::Increment, start, end)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Plus, start, self.cursor)
                }
            }
            MINUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::Decrement, start, end)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Minus, start, self.cursor)
                }
            }

            _ => {
                match SYMBOLS.binary_search_by(|&(k, _)| k.cmp(&(self.current)))
                        .ok()
                        .and_then(|i| SYMBOLS.get(i)){
                            Some(token) => {
                                self.advance();
                                let end = self.cursor;
                                VoidstarToken::new(token.1, start, end)
                            },
                            None => panic!("unknown symbol at {}, line {}", self.current, self.line)
                }
            }
        }
    }

    fn keyword(&mut self) -> VoidstarToken{
        let start = self.cursor;
        while !self.end() && (self.current.is_ascii_alphanumeric() || self.current == UNDERSCORE){
            self.advance();
        }
        let end = self.cursor;
        
        match KEYWORDS.binary_search_by(|&(k, _)| k.cmp(&&self.source[start..end]))
                .ok()
                .and_then(|i| KEYWORDS.get(i)){
                    Some(token) => VoidstarToken::new(token.1, start, end),
                    None => VoidstarToken::new(VoidstarTokenTypes::Ident, start, end),
        }
    }

    fn digit(&mut self) -> VoidstarToken{
        let start = self.cursor;
        let end;

        'integer: {
            while !self.end() && self.current.is_ascii_digit(){
                self.advance();

                if self.current == DOT{
                    break 'integer;
                }
            }
            end = self.cursor;
            return VoidstarToken::new(VoidstarTokenTypes::IntLiteral, start, end);
        }
        // honestly, it's an overhead, but it keeps things organized
        #[allow(unused)]
        'float: {
            self.advance();

            while !self.end() && self.current.is_ascii_digit(){
                self.advance();
            }
            end = self.cursor;
            return VoidstarToken::new(VoidstarTokenTypes::FloatLiteral, start, end);
        }
    }

    pub fn lexerize(&mut self) -> Vec<VoidstarToken>{
        let mut tokens: Vec<VoidstarToken> = Vec::new();
        self.current = self.source[self.cursor];

        while !self.end(){
            match self.current{
                OPEN_PAREN | CLOSE_PAREN | OPEN_BRACE | CLOSE_BRACE | OPEN_BRACKET | CLOSE_BRACKET |
                DOT | COMMA | COLON | SEMICOL | EQ | NOT | ASTRSK | QUOTES | APOSTROPHE | PLUS | MINUS |
                GREATER | LESSER => {
                    tokens.push(self.symbol());
                },

                val if val.is_ascii_alphabetic() => tokens.push(self.keyword()),
                val if val.is_ascii_digit() => tokens.push(self.digit()),

                SLASH => {
                    self.advance();
                    match self.current{
                        SLASH => {
                            loop{
                                self.advance();
                                if self.end() || self.current == LINE_FEED || self.current == HALT{
                                    break;
                                }
                            }
                        }

                        ASTRSK => {
                            loop{
                                self.advance();
                                if self.current == ASTRSK && self.advance() == SLASH{
                                    break;
                                }
                            }
                            self.advance();
                        }
                        _ => tokens.push(VoidstarToken::new(VoidstarTokenTypes::Slash, self.cursor, self.cursor))
                    }
                }

                LINE_FEED => {
                    self.line+=1;
                    self.advance();
                    continue;
                },

                TABULATION | SPACE => {
                    self.advance();
                    continue;
                },

                _ => panic!("lexer reached an impossible state on line {}, byte `{}`", self.line, self.source[self.cursor])
            }
        }

        println!("{:#?}", tokens);
        tokens
    }
}