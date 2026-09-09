use core::panic;

use crate::{tokens::{VoidstarToken, VoidstarTokenTypes}, *};

pub struct Lexer{
    source: Vec<u8>,
    line: usize,
    cursor: usize,
    current: u8
}

// store bytes instead of actual full strings, fuck readability
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
        match self.current{
            QUOTES => {
                todo!("strings are not implemented yet.");
            },

            APOSTROPHE => {
                self.advance();
                let v = self.current as char;
                self.advance_this(2);
                return VoidstarToken::new(VoidstarTokenTypes::Char, v.to_string());
            },

            GREATER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        VoidstarToken::new(VoidstarTokenTypes::GreaterEq, ">=".to_string())
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Greater, ">".to_string())
                }
            },
            LESSER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        VoidstarToken::new(VoidstarTokenTypes::LesserEq, "<=".to_string())
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Lesser, "<".to_string())
                }
            },
            PLUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        VoidstarToken::new(VoidstarTokenTypes::Increment, "+=".to_string())
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Plus, "+".to_string())
                }
            }
            MINUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        VoidstarToken::new(VoidstarTokenTypes::Decrement, "-=".to_string())
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Minus, "-".to_string())
                }
            }

            _ => {
                match SYMBOLS.binary_search_by(|&(k, _)| k.cmp(&(self.current as char)))
                        .ok()
                        .and_then(|i| SYMBOLS.get(i)){
                            Some(token) => {
                                self.advance();
                                VoidstarToken::new(token.1, token.0.to_string())
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
                    Some(token) => VoidstarToken::new(token.1, token.0.to_string()),
                    None => VoidstarToken::new(VoidstarTokenTypes::Ident, value),
        }
    }

    fn digit(&mut self) -> VoidstarToken{
        let mut value = String::new();

        'integer: {
            while !self.end() && self.current.is_ascii_digit(){
                value.push(self.current as char);
                self.advance();

                if self.current == DOT{
                    break 'integer;
                }
            }
            return VoidstarToken::new(VoidstarTokenTypes::IntLiteral, value);
        }
        // honestly, it's an overhead, but it keeps things organized
        #[allow(unused)]
        'float: {
            value.push(self.current as char);
            self.advance();

            while !self.end() && self.current.is_ascii_digit(){
                value.push(self.current as char);
                self.advance();
            }
            return VoidstarToken::new(VoidstarTokenTypes::FloatLiteral, value);
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
                    println!("{}", self.current as char);
                    match self.current{
                        SLASH => {
                            loop{
                                self.advance();
                                if self.current == LINE_FEED || self.current == HALT{
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
                        _ => tokens.push(VoidstarToken::new(VoidstarTokenTypes::Slash, "/".to_string())),
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

                HALT => tokens.push(VoidstarToken::new(VoidstarTokenTypes::Halt, String::new())),
                _ => panic!("lexer reached an impossible state on byte `{}`!", self.source[self.cursor])
            }
        }

        println!("{:#?}", tokens);
        tokens
    }
}