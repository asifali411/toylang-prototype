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
            ' ' | '\t' | '\r' => {}
            '\n' => {
                self.line += 1;
                self.column = 1;
            }

            ';' => self.add_token(TokenKind::SEMI),
            ':' => self.add_token(TokenKind::COLON),
            ',' => self.add_token(TokenKind::COMMA),
            '.' => self.add_token(TokenKind::DOT),

            '+' => {
                let kind = if self.match_next('+') {
                    TokenKind::INC
                } else if self.match_next('=') {
                    TokenKind::PLUS_EQ
                } else {
                    TokenKind::PLUS
                };
                self.add_token(kind);
            }
            '-' => {
                let kind = if self.match_next('-') {
                    TokenKind::DEC
                } else if self.match_next('=') {
                    TokenKind::MINUS_EQ
                } else {
                    TokenKind::MINUS
                };
                self.add_token(kind);
            }
            '*' => {
                let kind = if self.match_next('=') {
                    TokenKind::STAR_EQ
                } else {
                    TokenKind::STAR
                };
                self.add_token(kind);
            }
            '%' => {
                let kind = if self.match_next('=') {
                    TokenKind::MOD_EQ
                } else {
                    TokenKind::MOD
                };
                self.add_token(kind);
            }

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.skip_comment()?;
                } else if self.match_next('=') {
                    self.add_token(TokenKind::SLASH_EQ);
                } else {
                    self.add_token(TokenKind::SLASH);
                }
            }

            '(' => self.add_token(TokenKind::OPEN_PAREN),
            ')' => self.add_token(TokenKind::CLOSE_PAREN),
            '{' => self.add_token(TokenKind::OPEN_BRACE),
            '}' => self.add_token(TokenKind::CLOSE_BRACE),
            '[' => self.add_token(TokenKind::OPEN_BRACK),
            ']' => self.add_token(TokenKind::CLOSE_BRACK),

            '"' | '\'' => self.scan_string(c)?,

            '!' => {
                let kind = if self.match_next('=') {
                    TokenKind::NOT_EQ
                } else {
                    TokenKind::NOT
                };
                self.add_token(kind);
            }
            '=' => {
                let kind = if self.match_next('=') {
                    TokenKind::EQ_EQ
                } else {
                    TokenKind::EQUAL
                };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.match_next('=') {
                    TokenKind::GREAT_EQ
                } else {
                    TokenKind::GREAT
                };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.match_next('=') {
                    TokenKind::LESS_EQ
                } else {
                    TokenKind::LESS
                };
                self.add_token(kind);
            }

            '&' => self.add_token(TokenKind::AND),
            '|' => self.add_token(TokenKind::OR),
            
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
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        lexeme
            .parse::<f64>()
            .map(|n| self.add_token(TokenKind::NUM(n)))
            .map_err(|_| LexError::InvalidNumber {
                lexeme,
                line: self.line,
                col: self.column,
            })
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
        let mut value = String::new();

        while !self.is_at_end() {
            match self.peek() {
                '\\' => {
                    self.advance();

                    let escaped = match self.peek() {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\'' => '\'',
                        '\\' => '\\',
                        other => {
                            return Err(LexError::InvalidEscapeCharacter {
                                char: other,
                                line: self.line,
                                col: self.column,
                            });
                        }
                    };

                    value.push(escaped);
                    self.advance();
                }

                c if c == quote => break,

                c => {
                    value.push(c);
                    self.advance();
                }
            }
        }

        if self.is_at_end() {
            return Err(LexError::ExpectedCharacter {
                message: format!("unterminated string, '{quote}' was never closed"),
                line: self.line,
                col: self.column,
            });
        }

        self.advance();
        self.add_token(TokenKind::STRING(value));
        Ok(())
    }

    fn skip_comment(&mut self) -> LResult<()> {
        loop {
            if self.is_at_end() {
                return Err(LexError::ExpectedCharacter { 
                    message: format!("unterminated multiline comment"), 
                    line: self.line, 
                    col: self.column 
                });
            }

            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            }
            let c = self.advance();
            if c == '\n' { self.column = 1; }
        }
        


        Ok(())
    }

    // -------------------------------------------------------------------------

    fn keyword(s: &str) -> Option<TokenKind> {
        match s {
            "var" => Some(TokenKind::VAR),
            "true" => Some(TokenKind::TRUE),
            "false" => Some(TokenKind::FALSE),
            "if" => Some(TokenKind::IF),
            "else" => Some(TokenKind::ELSE),
            "loop" => Some(TokenKind::LOOP),
            "func" => Some(TokenKind::FUNC),
            "return" => Some(TokenKind::RETURN),
            "class" => Some(TokenKind::CLASS),
            "inherit" => Some(TokenKind::INHERIT),
            "in" => Some(TokenKind::IN),
            "break" => Some(TokenKind::BREAK),
            "continue" => Some(TokenKind::CONTINUE),
            "and" => Some(TokenKind::AND),
            "or" => Some(TokenKind::OR),
            _ => None,
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
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            span: Span {
                line: self.line,
                column: self.column,
            },
        });
    }
}

