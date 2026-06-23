use crate::{
    lexer::token::{Token, TokenKind},
    parser::expression::Expr,
};
use crate::errors::parse_error::ParseError;

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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    //---------------------------------------------------------------

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::LESS | TokenKind::GREAT |
                TokenKind::LESS_EQ | TokenKind::GREAT_EQ => {
                    let op = self.advance().unwrap().clone();
                    let right = self.term()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::PLUS | TokenKind::MINUS => {
                    let op = self.advance().unwrap().clone();
                    let right = self.factor()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::STAR | TokenKind::SLASH => {
                    let op = self.advance().unwrap().clone();
                    let right = self.unary()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        operator: op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(tok) = self.peek() {
            if tok.kind == TokenKind::MINUS {
                let op = self.advance().unwrap().clone();
                return Ok(Expr::Unary {
                    operator: op,
                    right: Box::new(self.unary()?),
                });
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Some(tok) => match tok.kind {
                TokenKind::INT(_) | TokenKind::FLOAT(_) => Ok(Expr::Literal(tok.clone())),
                _ => Err(ParseError::UnexpectedToken {
                    token: tok.to_string(),
                    line: tok.span.line,
                    col: tok.span.column,
                }),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    //---------------------------------------------------------------

    fn is_empty(&self) -> bool {
        self.current >= self.tokens.len()
            || self.tokens[self.current].kind == TokenKind::EOF
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
        if self.is_empty() {
            return None;
        }
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
}
