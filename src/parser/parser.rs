use crate::errors::parse_error::ParseError;
use crate::{
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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    //---------------------------------------------------------------

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::NOT_EQ | TokenKind::EQ_EQ => {
                    let op = self.advance().unwrap().clone();
                    let right = self.comparison()?;
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

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::LESS | TokenKind::GREAT | TokenKind::LESS_EQ | TokenKind::GREAT_EQ => {
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
                TokenKind::OPEN_BRACE => {
                    let expr = self.expression()?;

                    match self.consume(TokenKind::CLOSE_BRACE, "Expect ')' after an expression") {
                        Some(err) => Err(err),
                        None => Ok(Expr::Group {
                            expr: Box::new(expr),
                        }),
                    }
                }
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

    fn consume(&mut self, token_kind: TokenKind, message: &str) -> Option<ParseError> {
        let tok = &self.tokens[self.current];

        if tok.kind == token_kind {
            None
        } else {
            Some(ParseError::ExpectedToken {
                message: message.to_string(),
                line: tok.span.line,
                col: tok.span.column,
            })
        }
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
