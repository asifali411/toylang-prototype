use crate::errors::lex_error::LexError;
use crate::lexer::token::{Span, Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, LexError> {
        while !self.is_empty() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(TokenKind::EOF);
        Ok(&self.tokens)
    }

    //---------------------------------------------------------------------------

    fn scan_token(&mut self) -> Result<(), LexError> {
        let c = self.advance().unwrap();
        match c {
            ' ' | '\t' => self.column += 1,
            '\r' | '\n' => {
                self.line += 1;
                self.column = 0;
            }

            '+' => self.add_token(TokenKind::PLUS),
            '-' => self.add_token(TokenKind::MINUS),
            '*' => self.add_token(TokenKind::STAR),
            '/' => self.add_token(TokenKind::SLASH),

            '(' => self.add_token(TokenKind::OPEN_PAREN),
            ')' => self.add_token(TokenKind::CLOSE_PAREN),

            '!' => {
                if let Some(c) = self.peek() {
                    if c == '=' {
                        self.advance();
                        self.add_token(TokenKind::NOT_EQ);
                    } else {
                        self.add_token(TokenKind::NOT);
                    }
                }
            }
            
            '=' => {
                if let Some(c) = self.peek() {
                    if c == '=' {
                        self.advance();
                        self.add_token(TokenKind::EQ_EQ);
                    } else {
                        self.add_token(TokenKind::EQUAL);
                    }
                }
            }
            
            '>' => {
                if let Some(c) = self.peek() {
                    if c == '=' {
                        self.advance();
                        self.add_token(TokenKind::GREAT_EQ);
                    } else {
                        self.add_token(TokenKind::GREAT);
                    }
                }
            }
            
            '<' => {
                if let Some(c) = self.peek() {
                    if c == '=' {
                        self.advance();
                        self.add_token(TokenKind::LESS_EQ);
                    } else {
                        self.add_token(TokenKind::LESS);
                    }
                }
            }

            _ => {
                if c.is_ascii_digit() {
                    self.generate_number()?;
                } else {
                    return Err(LexError::UndefinedCharacter {
                        char: c,
                        line: self.line,
                        col: self.column,
                    });
                }
            }
        }
        Ok(())
    }

    fn generate_number(&mut self) -> Result<(), LexError> {
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c == '.' {
                self.advance();
                is_float = true;
                continue;
            }
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            match lexeme.parse::<f32>() {
                Ok(n) => self.add_token(TokenKind::FLOAT(n)),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: self.line,
                        col: self.column,
                    });
                }
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(n) => self.add_token(TokenKind::INT(n)),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: self.line,
                        col: self.column,
                    });
                }
            }
        }

        Ok(())
    }

    //---------------------------------------------------------------------------

    fn is_empty(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        if self.is_empty() {
            return None;
        }
        Some(self.source[self.current])
    }

    // unused function
    // fn peek_next(&self) -> Option<char> {
    //     if self.current >= self.source.len() - 1 {
    //         None
    //     } else {
    //         Some(self.source[self.current + 1])
    //     }
    // }

    fn add_token(&mut self, token_kind: TokenKind) {
        self.tokens.push(Token {
            kind: token_kind,
            span: Span {
                line: self.line,
                column: self.column,
            },
        });
    }
}
