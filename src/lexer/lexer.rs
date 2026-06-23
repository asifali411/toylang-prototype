use crate::{
    error::Error,
    lexer::token::{Span, Token, TokenKind},
};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,

    max_len: usize,
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

            max_len: source.len(),
        }
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, String> {
        while !self.is_empty() {
            self.start = self.current;

            if let Some(err) = self.scan_token() {
                return Err(Error::syntax_error(&err, self.line, self.column));
            }
        }

        self.add_token(TokenKind::EOF);
        Ok(&self.tokens)
    }

    //---------------------------------------------------------------------------

    fn scan_token(&mut self) -> Option<String> {
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

            _ => {
                if c.is_ascii_digit() {
                    self.generate_number();
                } else {
                    return Some(format!("Undefined character: {}", c));
                }
            }
        };

        return None;
    }

    fn generate_number(&mut self) {
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
            self.add_token(TokenKind::FLOAT(
                lexeme.parse::<f32>().expect("Failed to parse float"),
            ));
        } else {
            self.add_token(TokenKind::INT(lexeme.parse().expect("Failed to parse integer")));
        }
    }

    //---------------------------------------------------------------------------

    fn is_empty(&self) -> bool {
        self.current >= self.max_len
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }

        let c = self.source[self.current];
        self.current += 1;
        Some(c)
    }

    fn peek(&self) -> Option<char> {
        if self.is_empty() {
            return None;
        }

        Some(self.source[self.current])
    }

    fn add_token(&mut self, token_kind: TokenKind) {
        let token = Token {
            kind: token_kind,
            span: Span {
                line: self.line,
                column: self.column,
            },
        };
        self.tokens.push(token);
    }
}
