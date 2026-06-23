use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::expression::Expr,
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

    fn expression(&mut self) -> Result<Expr, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, String> {
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

    fn factor(&mut self) -> Result<Expr, String> {
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

    fn unary(&mut self) -> Result<Expr, String> {
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

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(tok) => match tok.kind {
                TokenKind::INT(_) | TokenKind::FLOAT(_) => Ok(Expr::Literal(tok.clone())),
                _ => Err(Error::parse_error(
                    &format!("Expected expression but found '{tok}'"),
                    tok.span.line,
                    tok.span.column,
                )),
            },
            None => Err(Error::parse_error("Unexpected end of input", 0, 0)),
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
        if self.is_empty() {
            return None;
        }
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }
}
