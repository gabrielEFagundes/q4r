use core::panic;

use crate::{
    errors::error::QError,
    tokens::{VoidstarToken, VoidstarTokenTypes},
    *,
};

pub struct Lexer {
    source: Vec<u8>,
    line: usize,
    cursor: usize,
    current: u8,
    errors: Vec<QError>
}

impl Lexer {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            line: 1,
            cursor: 0,
            current: 0,
            errors: Vec::new()
        }
    }

    fn end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.cursor += 1;
        if self.cursor < self.source.len() {
            self.current = self.source[self.cursor];
        }
        self.current
    }

    fn advance_this(&mut self, amount: usize) -> u8 {
        self.cursor += amount;
        if self.cursor < self.source.len() {
            self.current = self.source[self.cursor];
        }
        self.current
    }

    fn symbol(&mut self) -> VoidstarToken {
        let start = self.cursor;
        let end;
        match self.current {
            QUOTES => {
                self.advance();
                let start = self.cursor;

                while self.current != QUOTES {
                    if self.current == QUOTES && self.source[self.cursor - 1] == BACKSLSH {
                        self.advance();
                    }
                    self.advance();
                }

                let end = self.cursor;
                self.advance();
                VoidstarToken::new(
                    VoidstarTokenTypes::StringLiteral,
                    tokens::Span { start, end, line: self.line },
                    true,
                )
            }

            APOSTROPHE => {
                self.advance();
                let v = self.cursor;
                self.advance_this(2);
                VoidstarToken::new(VoidstarTokenTypes::CharLiteral, tokens::Span { start: v, end: v+1, line: self.line }, true)
            }

            GREATER => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::GreaterEq, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Greater,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },
            LESSER => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::LesserEq, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Lesser,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },
            EQ => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::CompEquals, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Equals,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },

            NOT => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::NotEquals, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Not,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },

            PLUS => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::Increment, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Plus,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },
            MINUS => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::Decrement, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Minus,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },
            ASTRSK => match self.advance() {
                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::Multiply, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Asterisk,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },
            SLASH => match self.advance() {
                SLASH => loop {
                    self.advance();
                    if self.end() || self.current == LINE_FEED || self.current == HALT {
                        return VoidstarToken::default();
                    }
                },

                ASTRSK => loop {
                    self.advance();
                    if self.current == LINE_FEED {
                        self.line += 1;
                    }
                    if self.current == ASTRSK && self.advance() == SLASH {
                        self.advance();
                        return VoidstarToken::default();
                    }
                },

                EQ => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::Divide, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Slash,
                    tokens::Span { start: self.cursor, end: self.cursor, line: self.line },
                    false,
                ),
            },

            CLOSE_BRACE => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(VoidstarTokenTypes::CloseBraces, tokens::Span { start, end, line: self.line }, true)
            }
            CLOSE_BRACKET => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(
                    VoidstarTokenTypes::CloseBrackets,
                    tokens::Span { start, end, line: self.line },
                    true,
                )
            }
            CLOSE_PAREN => {
                self.advance();
                end = self.cursor;
                VoidstarToken::new(
                    VoidstarTokenTypes::CloseParents,
                    tokens::Span { start, end, line: self.line },
                    true,
                )
            }

            DOT => {
                let mut count = 0;
                while self.current == DOT {
                    count += 1;
                    self.advance();
                }

                end = self.cursor;
                match count {
                    1 => VoidstarToken::new(VoidstarTokenTypes::Dot, tokens::Span { start, end, line: self.line }, false),
                    2 => {
                        VoidstarToken::new(VoidstarTokenTypes::DotDot, tokens::Span { start, end, line: self.line }, false)
                    }
                    3 => VoidstarToken::new(
                        VoidstarTokenTypes::Ellipsis,
                        tokens::Span { start, end, line: self.line },
                        false,
                    ),
                    _ => {
                        self.errors.push(QError { 
                            span: tokens::Span { start, end, line: self.line }, 
                            msg: "too many dots here".to_string()
                        });
                        VoidstarToken::new(VoidstarTokenTypes::Halt, tokens::Span { start, end, line: self.line }, false)
                    }
                }
            }

            AMPERSND => match self.advance() {
                AMPERSND => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::And, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Ampersand,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },

            PIPE => match self.advance() {
                PIPE => {
                    self.advance();
                    end = self.cursor;
                    VoidstarToken::new(VoidstarTokenTypes::Or, tokens::Span { start, end, line: self.line }, false)
                }
                _ => VoidstarToken::new(
                    VoidstarTokenTypes::Pipe,
                    tokens::Span { start, end: self.cursor, line: self.line },
                    false,
                ),
            },

            _ => {
                match SYMBOLS
                    .binary_search_by(|&(k, _)| k.cmp(&(self.current)))
                    .ok()
                    .and_then(|i| SYMBOLS.get(i))
                {
                    Some(token) => {
                        self.advance();
                        let end = self.cursor;
                        VoidstarToken::new(token.1, tokens::Span { start, end, line: self.line }, false)
                    }
                    None => panic!("unknown symbol at {}, line {}", self.current, self.line),
                }
            }
        }
    }

    fn keyword(&mut self) -> VoidstarToken {
        let start = self.cursor;
        while !self.end() && (self.current.is_ascii_alphanumeric() || self.current == UNDERSCORE) {
            self.advance();
        }
        let end = self.cursor;

        match KEYWORDS
            .binary_search_by(|&(k, _)| k.cmp(&self.source[start..end]))
            .ok()
            .and_then(|i| KEYWORDS.get(i))
        {
            Some(token) => {
                if token.1 == VoidstarTokenTypes::BoolLiteral {
                    return VoidstarToken::new(token.1, tokens::Span { start, end, line: self.line }, true); // bool literals can return a semicolon
                }
                VoidstarToken::new(token.1, tokens::Span { start, end, line: self.line }, false)
            }
            None => VoidstarToken::new(VoidstarTokenTypes::Ident, tokens::Span { start, end, line: self.line }, true),
        }
    }

    fn digit(&mut self) -> VoidstarToken {
        let start = self.cursor;
        let end;

        'integer: {
            while !self.end() && self.current.is_ascii_digit() {
                self.advance();

                if self.current == DOT {
                    break 'integer;
                }
            }
            end = self.cursor;
            return VoidstarToken::new(VoidstarTokenTypes::IntLiteral, tokens::Span { start, end, line: self.line }, true);
        }
        // honestly, it's an overhead, but it keeps things organized
        #[allow(unused)]
        'float: {
            self.advance();

            while !self.end() && self.current.is_ascii_digit() {
                self.advance();
            }
            end = self.cursor;
            VoidstarToken::new(
                VoidstarTokenTypes::FloatLiteral,
                tokens::Span { start, end, line: self.line },
                true,
            )
        }
    }

    pub fn lexerize(&mut self) -> Vec<VoidstarToken> {
        let mut tokens: Vec<VoidstarToken> = Vec::new();
        self.current = self.source[self.cursor];

        while !self.end() {
            match self.current {
                OPEN_PAREN | CLOSE_PAREN | OPEN_BRACE | CLOSE_BRACE | OPEN_BRACKET
                | CLOSE_BRACKET | DOT | COMMA | COLON | SEMICOL | EQ | NOT | SLASH | ASTRSK
                | AMPERSND | QUOTES | APOSTROPHE | PIPE | PLUS | MINUS | GREATER | LESSER => {
                    let symbol = self.symbol();

                    if symbol.token_type == VoidstarTokenTypes::default() {
                        continue;
                    }

                    tokens.push(symbol);
                }

                val if val.is_ascii_alphabetic() => tokens.push(self.keyword()),
                val if val.is_ascii_digit() => tokens.push(self.digit()),

                LINE_FEED => {
                    self.line += 1;

                    if !tokens.is_empty() && tokens[tokens.len() - 1].last_could_end_stmt {
                        tokens.push(VoidstarToken::new(
                            VoidstarTokenTypes::SemiColon,
                            tokens::Span { start: self.cursor, end: self.cursor, line: self.line },
                            false,
                        ));
                    }

                    self.advance();
                    continue;
                }

                TABULATION | SPACE | CARRIAGE => {
                    self.advance();
                    continue;
                }

                _ => self.errors.push(QError{
                    span: tokens::Span { start: self.cursor, end: self.cursor, line: self.line },
                    msg: "lexer reached an impossible state here".to_string()
                }),
            }
        }

        tokens
    }
}
