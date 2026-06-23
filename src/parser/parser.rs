use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::expression::{self, Expr},
};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    //---------------------------------------------------------------

    fn expression(&mut self) -> Result<Expr, String> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {

        let tok = self.advance().unwrap().clone();

        match tok.kind {
            TokenKind::INT(n) => Ok(Expr::Literal(tok)),
            TokenKind::FLOAT(n) => Ok(Expr::Literal(tok)),
           _ => Err(Error::parse_error(&format!("Expected expression but found '{tok}'"), tok.span.line, tok.span.column)) 
        }
    }

    //---------------------------------------------------------------

    fn is_empty(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].kind == TokenKind::EOF
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_empty() {
            return None;
        }

        let c = &self.tokens[self.current];
        self.current += 1;
        Some(c)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
}
