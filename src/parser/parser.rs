use crate::errors::parse_error::ParseError;
use crate::parser::statement::{self, Stmt};
use crate::{
    lexer::token::{Token, TokenKind},
    parser::expression::Expr,
};

type PResult<T> = Result<T, ParseError>;

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

    pub fn parse(&mut self) -> PResult<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_empty() {
            match self.declaration() {
                Err(e) => return Err(e),
                Ok(statement) => statements.push(statement),
            }
        }

        Ok(statements)
    }

    //---------------------------------------------------------------

    fn declaration(&mut self) -> PResult<Stmt> {
        match self.peek().cloned().ok_or(ParseError::UnexpectedEof)?.kind {
            TokenKind::VAR => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> PResult<Stmt> {
        self.advance();
        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone().to_string(),
                _ => {
                    return Err(ParseError::ExpectedVariableName {
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }
            },
            None => return Err(ParseError::UnexpectedEof),
        };
        self.advance();

        let has_initializer = match self.peek() {
            Some(tok) if tok.kind == TokenKind::EQUAL => {
                self.advance();
                true
            }
            _ => false,
        };

        if has_initializer {
            match self.expression() {
                Err(e) => Err(e),
                Ok(initializer) => {
                    self.consume(TokenKind::SEMI, "Expect ';' after variable declaration");
                    Ok(Stmt::Var {
                        name,
                        initializer: Some(initializer),
                    })
                }
            }
        } else {
            self.consume(TokenKind::SEMI, "Expect ';' after variable declaration");
            Ok(Stmt::Var {
                name,
                initializer: None,
            })
        }
    }

    //---------------------------------------------------------------

    fn statement(&mut self) -> PResult<Stmt> {
        match self.peek() {
            Some(tok) => match tok.kind {
                TokenKind::PRINT => self.print_statement(),
                TokenKind::OPEN_BRACE => self.block(),
                TokenKind::IF => self.if_statement(),
                TokenKind::LOOP => self.loop_statement(),
                _ => self.expression_statement(),
            },
            _ => Err(ParseError::UnexpectedEof),
        }
    }

    fn expression_statement(&mut self) -> PResult<Stmt> {
        let expr: Expr = match self.expression() {
            Err(e) => return Err(e),
            Ok(e) => e,
        };

        match self.consume(TokenKind::SEMI, "Expect ';' after an expression") {
            Err(e) => Err(e),
            _ => Ok(Stmt::Expr(expr)),
        }
    }

    fn print_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        match self.expression() {
            Err(e) => Err(e),
            Ok(expr) => match self.consume(TokenKind::SEMI, "Expect ';' after print statement") {
                Err(e) => Err(e),
                _ => Ok(Stmt::Print(expr)),
            },
        }
    }

    fn if_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        match self.expression() {
            Err(e) => Err(e),
            Ok(condition) => match self.block() {
                Err(e) => Err(e),
                Ok(if_body) => {
                    let has_else = match self.peek() {
                        Some(tok) => true,
                        _ => false,
                    };

                    if has_else {
                        self.advance();
                        let else_body = self.block()?;

                        return Ok(Stmt::IF {
                            condition,
                            if_body: Box::new(if_body),
                            else_body: Some(Box::new(else_body)),
                        });
                    } else {
                         return Ok(Stmt::IF {
                            condition,
                            if_body: Box::new(if_body),
                            else_body: None,
                        });
                    }
                }
            },
        }
    }

    fn loop_statement(&mut self) -> PResult<Stmt> {
        self.advance();

        let count = match self.expression() {
            Err(e) => return Err(e),
            Ok(count) => count,
        };
        let body = match self.block() {
            Err(e) => return Err(e),
            Ok(body) => body,
        };

        Ok(Stmt::LOOP { count, body: Box::new(body) })
    }

    fn block(&mut self) -> PResult<Stmt> {
        self.consume(TokenKind::OPEN_BRACE, "Expect '{' before block");

        let mut statements: Vec<Box<Stmt>> = Vec::new();

        while !self.is_empty() && !self.compare(TokenKind::CLOSE_BRACE) {
            match self.declaration() {
                Ok(statement) => statements.push(Box::new(statement)),
                Err(e) => return Err(e),
            }
        }

        self.consume(TokenKind::CLOSE_BRACE, "Expect '}' after block.");
        Ok(Stmt::Block(statements))
    }

    //---------------------------------------------------------------

    fn expression(&mut self) -> PResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> PResult<Expr> {
        let expr = self.equality()?;

        if self.compare(TokenKind::EQUAL) {
            let equals = self.advance().cloned().ok_or(ParseError::UnexpectedEof)?;

            let value = self.assignment()?;

            if let Expr::Var(tok) = expr {
                if let TokenKind::IDENT(name) = tok.kind {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }
            }

            return Err(ParseError::InvalidStatement {
                message: "Invalid assignment target".to_string(),
                line: equals.span.line,
                col: equals.span.column,
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> PResult<Expr> {
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

    fn comparison(&mut self) -> PResult<Expr> {
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

    fn term(&mut self) -> PResult<Expr> {
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

    fn factor(&mut self) -> PResult<Expr> {
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

    fn unary(&mut self) -> PResult<Expr> {
        if let Some(tok) = self.peek() {
            if tok.kind == TokenKind::MINUS || tok.kind == TokenKind::NOT {
                let op = self.advance().unwrap().clone();
                return Ok(Expr::Unary {
                    operator: op,
                    right: Box::new(self.unary()?),
                });
            }
        }
        self.primary()
    }

    fn primary(&mut self) -> PResult<Expr> {
        match self.advance() {
            Some(tok) => match &tok.kind {
                TokenKind::INT(_) | TokenKind::FLOAT(_) => Ok(Expr::Literal(tok.clone())),
                TokenKind::TRUE | TokenKind::FALSE => Ok(Expr::Literal(tok.clone())),
                TokenKind::STRING(v) => Ok(Expr::Literal(tok.clone())),
                TokenKind::OPEN_PAREN => {
                    let expr = self.expression()?;

                    match self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after an expression") {
                        Err(err) => Err(err),
                        _ => Ok(Expr::Group {
                            expr: Box::new(expr),
                        }),
                    }
                }
                TokenKind::IDENT(_) => {
                    // for now consider every identifier as a variable
                    Ok(Expr::Var(tok.clone()))
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

    fn consume(&mut self, token_kind: TokenKind, message: &str) -> Result<(), ParseError> {
        match self.peek() {
            Some(tok) if tok.kind == token_kind => {
                self.current += 1;
                Ok(())
            }
            Some(tok) => Err(ParseError::ExpectedToken {
                message: message.to_string(),
                line: tok.span.line,
                col: tok.span.column,
            }),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn compare(&self, token_kind: TokenKind) -> bool {
        match self.peek() {
            Some(tok) if tok.kind == token_kind => true,
            _ => false,
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
