use crate::errors::lex_error::LexError;
use crate::lexer::token::{Span, Token, TokenKind};

type LResult<T> = Result<T, LexError>;

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

    pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
        while !self.is_empty() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(TokenKind::EOF);
        Ok(&self.tokens)
    }

    //---------------------------------------------------------------------------

    fn scan_token(&mut self) -> LResult<()> {
        let c = self.advance().unwrap();
        match c {
            ' ' | '\t' => self.column += 1,
            '\r' | '\n' => {
                self.line += 1;
                self.column = 0;
            }

            ';' => self.add_token(TokenKind::SEMI),
            ',' => self.add_token(TokenKind::COMMA),

            '+' => self.add_token(TokenKind::PLUS),
            '-' => self.add_token(TokenKind::MINUS),
            '*' => self.add_token(TokenKind::STAR),
            '/' => self.add_token(TokenKind::SLASH),

            '(' => self.add_token(TokenKind::OPEN_PAREN),
            ')' => self.add_token(TokenKind::CLOSE_PAREN),
            '{' => self.add_token(TokenKind::OPEN_BRACE),
            '}' => self.add_token(TokenKind::CLOSE_BRACE),

            '"' | '\'' => self.generate_string(c)?,

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
                } else if c.is_ascii_alphabetic() {
                    self.generate_identifier();
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

    fn generate_number(&mut self) -> LResult<()> {
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

    fn generate_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
                continue;
            }

            break;
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        match &lexeme[..] {
            "print" => self.add_token(TokenKind::PRINT),
            "var" => self.add_token(TokenKind::VAR),
            "true" => self.add_token(TokenKind::TRUE),
            "false" => self.add_token(TokenKind::FALSE),
            "if" => self.add_token(TokenKind::IF),
            "else" => self.add_token(TokenKind::ELSE),
            "loop" => self.add_token(TokenKind::LOOP),
            "func" => self.add_token(TokenKind::FUNC),
            "return" => self.add_token(TokenKind::RETURN),
            _ => self.add_token(TokenKind::IDENT(lexeme)),
        }
    }

    fn generate_string(&mut self, punctuation: char) -> LResult<()> {
        self.start = self.current;
        self.advance();
        while let Some(c) = self.peek() {
            if c == punctuation {
                break;
            }
            self.advance();
        }

        if self.is_empty() {
            return Err(LexError::ExpectedCharacter {
                message: format!("'{}' was never closed", punctuation),
                line: self.line,
                col: self.column,
            });
        }

        let string: String = self.source[self.start..self.current].iter().collect();

        self.advance();
        self.add_token(TokenKind::STRING(string));
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
