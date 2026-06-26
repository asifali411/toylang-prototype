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
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> LResult<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(TokenKind::EOF);
        Ok(&self.tokens)
    }

    // -------------------------------------------------------------------------

    fn scan_token(&mut self) -> LResult<()> {
        let c = self.advance();
        match c {
            ' ' | '\t' => {}
            '\r' => {}
            '\n' => {
                self.line += 1;
                self.column = 1;
            }

            ';' => self.add_token(TokenKind::SEMI),
            ',' => self.add_token(TokenKind::COMMA),

            '+' => self.add_token(TokenKind::PLUS),
            '-' => self.add_token(TokenKind::MINUS),
            '*' => self.add_token(TokenKind::STAR),

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::SLASH);
                }
            }

            '(' => self.add_token(TokenKind::OPEN_PAREN),
            ')' => self.add_token(TokenKind::CLOSE_PAREN),
            '{' => self.add_token(TokenKind::OPEN_BRACE),
            '}' => self.add_token(TokenKind::CLOSE_BRACE),

            '"' | '\'' => self.scan_string(c)?,

            '!' => {
                let kind = if self.match_next('=') { TokenKind::NOT_EQ } else { TokenKind::NOT };
                self.add_token(kind);
            }
            '=' => {
                let kind = if self.match_next('=') { TokenKind::EQ_EQ } else { TokenKind::EQUAL };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.match_next('=') { TokenKind::GREAT_EQ } else { TokenKind::GREAT };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.match_next('=') { TokenKind::LESS_EQ } else { TokenKind::LESS };
                self.add_token(kind);
            }

            c if c.is_ascii_digit() => self.scan_number()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(),

            _ => {
                return Err(LexError::UndefinedCharacter {
                    char: c,
                    line: self.line,
                    col: self.column,
                });
            }
        }
        Ok(())
    }

    fn scan_number(&mut self) -> LResult<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let is_float = self.peek() == '.' && self.peek_next().is_ascii_digit();
        if is_float {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            lexeme.parse::<f32>().map(|n| self.add_token(TokenKind::FLOAT(n))).map_err(|_| {
                LexError::InvalidNumber { lexeme, line: self.line, col: self.column }
            })
        } else {
            lexeme.parse::<i64>().map(|n| self.add_token(TokenKind::INT(n))).map_err(|_| {
                LexError::InvalidNumber { lexeme, line: self.line, col: self.column }
            })
        }
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let kind = Self::keyword(&lexeme).unwrap_or(TokenKind::IDENT(lexeme));
        self.add_token(kind);
    }

    fn scan_string(&mut self, quote: char) -> LResult<()> {
        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::ExpectedCharacter {
                message: format!("unterminated string, '{quote}' was never closed"),
                line: self.line,
                col: self.column,
            });
        }

        let string: String = self.source[self.start + 1..self.current].iter().collect();
        self.advance();
        self.add_token(TokenKind::STRING(string));
        Ok(())
    }

    // -------------------------------------------------------------------------

    fn keyword(s: &str) -> Option<TokenKind> {
        match s {
            "print"  => Some(TokenKind::PRINT),
            "var"    => Some(TokenKind::VAR),
            "true"   => Some(TokenKind::TRUE),
            "false"  => Some(TokenKind::FALSE),
            "if"     => Some(TokenKind::IF),
            "else"   => Some(TokenKind::ELSE),
            "loop"   => Some(TokenKind::LOOP),
            "func"   => Some(TokenKind::FUNC),
            "return" => Some(TokenKind::RETURN),
            "class" => Some(TokenKind::CLASS),
            _        => None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { '\0' } else { self.source[self.current + 1] }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            span: Span { line: self.line, column: self.column },
        });
    }
}