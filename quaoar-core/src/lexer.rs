use core::panic;

use crate::{error::{error::self, lexer_err::LexerErr}, tokens::{VoidstarToken, VoidstarTokenTypes}, *};

pub struct Lexer{
    source: Vec<u8>,
    line: usize,
    cursor: usize,
    current: u8,
    debug: bool
}

impl Lexer{
    pub fn new(source: Vec<u8>, debug: bool) -> Self{
        Self { source, line: 1, cursor: 0, current: 0, debug }
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
        let start = self.cursor; let end;
        match self.current{
            QUOTES => {
                self.advance();
                let start = self.cursor;

                while self.current != QUOTES{
                    self.advance();
                }

                let end = self.cursor;
                self.advance();
                VoidstarToken::new(VoidstarTokenTypes::StringLiteral, start, end, true)
            },

            APOSTROPHE => {
                self.advance();
                let v = self.cursor;
                self.advance_this(2);
                return VoidstarToken::new(VoidstarTokenTypes::CharLiteral, v, v+1, true);
            },

            GREATER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::GreaterEq, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Greater, start, self.cursor, false)
                }
            },
            LESSER => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::LesserEq, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Lesser, start, self.cursor, false)
                    
                }
            },
            EQ => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::CompEquals, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Equals, start, self.cursor, false)
                }
            },

            NOT => {
                match self.advance(){
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::NotEquals, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Not, start, self.cursor, false)
                }
            },

            PLUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::Increment, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Plus, start, self.cursor, false)
                }
            }
            MINUS => {
                match self.advance() {
                    EQ => {
                        self.advance();
                        end = self.cursor;
                        VoidstarToken::new(VoidstarTokenTypes::Decrement, start, end, false)
                    },
                    _ => VoidstarToken::new(VoidstarTokenTypes::Minus, start, self.cursor, false)
                }
            }

            CLOSE_BRACE => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(VoidstarTokenTypes::CloseBraces, start, end, true)
            },
            CLOSE_BRACKET => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(VoidstarTokenTypes::CloseBrackets, start, end, true)
            },
            CLOSE_PAREN => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(VoidstarTokenTypes::CloseParents, start, end, true)
            },

            DOT => {
                let mut count = 0;
                while self.current == DOT{
                    count += 1;
                    self.advance();
                }
                
                end = self.cursor;
                match count{
                    1 => VoidstarToken::new(VoidstarTokenTypes::Dot, start, end, false),
                    2 => VoidstarToken::new(VoidstarTokenTypes::DotDot, start, end, false),
                    3 => VoidstarToken::new(VoidstarTokenTypes::Ellipsis, start, end, false),
                    _ => panic!("too many dots at line {}: there are {} dots!", self.line, count)
                }
            }

            _ => {
                match SYMBOLS.binary_search_by(|&(k, _)| k.cmp(&(self.current)))
                        .ok()
                        .and_then(|i| SYMBOLS.get(i)){
                            Some(token) => {
                                self.advance();
                                let end = self.cursor;
                                VoidstarToken::new(token.1, start, end, false)
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
                    Some(token) => {
                        if token.1 == VoidstarTokenTypes::BoolLiteral{ 
                            return VoidstarToken::new(token.1, start, end, true) // bool literals can return a semicolon
                        }
                        VoidstarToken::new(token.1, start, end, false)
                    },
                    None => VoidstarToken::new(VoidstarTokenTypes::Ident, start, end, true),
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
            return VoidstarToken::new(VoidstarTokenTypes::IntLiteral, start, end, true);
        }
        // honestly, it's an overhead, but it keeps things organized
        #[allow(unused)]
        'float: {
            self.advance();

            while !self.end() && self.current.is_ascii_digit(){
                self.advance();
            }
            end = self.cursor;
            return VoidstarToken::new(VoidstarTokenTypes::FloatLiteral, start, end, true);
        }
    }

    pub fn lexerize(&mut self) -> Vec<VoidstarToken>{
        let mut tokens: Vec<VoidstarToken> = Vec::new();
        self.current = self.source[self.cursor];

        while !self.end(){
            match self.current{
                OPEN_PAREN | CLOSE_PAREN | OPEN_BRACE | CLOSE_BRACE | OPEN_BRACKET | CLOSE_BRACKET |
                DOT | COMMA | COLON | SEMICOL | EQ | NOT | ASTRSK | AMPERSND | QUOTES | APOSTROPHE | PLUS | MINUS |
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
                        _ => tokens.push(VoidstarToken::new(VoidstarTokenTypes::Slash, self.cursor, self.cursor, false))
                    }
                }

                LINE_FEED => {
                    self.line+=1;

                    if tokens.len() > 0 && tokens[tokens.len()-1].last_could_end_stmt{
                        tokens.push(VoidstarToken::new(VoidstarTokenTypes::SemiColon, self.cursor, self.cursor, false));
                    }

                    self.advance();
                    continue;
                },

                TABULATION | SPACE | CARRIAGE => {
                    self.advance();
                    continue;
                },

                _ => {
                    error::QError::handle_new_error(
                        error::QErrorTypes::LexerErr(LexerErr::ImpossibleState), 
                        self.line, self.cursor, self.debug);
                }
            }
        }

        tokens
    }
}