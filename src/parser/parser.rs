use crate::errors::parse_error::ParseError;
use crate::parser::statement::Stmt;
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
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    //---------------------------------------------------------------

    fn declaration(&mut self) -> PResult<Stmt> {
        match self.peek().cloned().ok_or(ParseError::UnexpectedEof)?.kind {
            TokenKind::VAR => self.var_declaration(),
            TokenKind::FUNC => self.func_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> PResult<Stmt> {
        self.advance();

        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone(),
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

        let initializer = if self.compare(TokenKind::EQUAL) {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenKind::SEMI, "Expect ';' after variable declaration")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn func_declaration(&mut self) -> PResult<Stmt> {
        self.advance();

        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone(),
                _ => {
                    return Err(ParseError::ExpectedFunctionName {
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }
            },
            None => return Err(ParseError::UnexpectedEof),
        };
        self.advance();
        self.consume(TokenKind::OPEN_PAREN, "Expect '(' after function name")?;

        let mut parameters: Vec<Token> = Vec::new();

        if !self.compare(TokenKind::CLOSE_PAREN) {
            parameters.push(self.consume_ident("Expect parameter name")?);

            while self.compare(TokenKind::COMMA) {
                self.advance();
                // TODO: handle max parameter length here

                parameters.push(self.consume_ident("Expect parameter name")?);
            }
        }
        self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after parameters")?;
        let body = self.block()?;

        Ok(Stmt::Func {
            name,
            parameters,
            body: Box::new(body),
        })
    }

    //---------------------------------------------------------------

    fn statement(&mut self) -> PResult<Stmt> {
        match self.peek() {
            Some(tok) => match tok.kind {
                TokenKind::PRINT => self.print_statement(),
                TokenKind::OPEN_BRACE => self.block(),
                TokenKind::IF => self.if_statement(),
                TokenKind::LOOP => match self.peek_next() {
                    Some(tok) if tok.kind == TokenKind::IF => self.loop_if_statement(),
                    _ => self.loop_statement(),
                },
                TokenKind::RETURN => self.return_statement(),
                _ => self.expression_statement(),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn expression_statement(&mut self) -> PResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::SEMI, "Expect ';' after an expression")?;
        Ok(Stmt::Expr(expr))
    }

    fn print_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        let expr = self.expression()?;
        self.consume(TokenKind::SEMI, "Expect ';' after print statement")?;
        Ok(Stmt::Print(expr))
    }

    fn if_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        let condition = self.expression()?;
        let if_body = Box::new(self.block()?);

        let else_body = if self.compare(TokenKind::ELSE) {
            self.advance();
            Some(Box::new(self.block()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            if_body,
            else_body,
        })
    }

    fn loop_if_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        self.advance();
        let condition = self.expression()?;
        let body = Box::new(self.block()?);
        Ok(Stmt::LoopIf { condition, body })
    }

    fn loop_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        let count = self.expression()?;
        let body = Box::new(self.block()?);
        Ok(Stmt::Loop { count, body })
    }

    fn return_statement(&mut self) -> PResult<Stmt> {
        self.advance();
        let expr = self.expression()?;
        self.consume(TokenKind::SEMI, "Expect ';' after return statement")?;
        Ok(Stmt::Return(expr))
    }

    fn block(&mut self) -> PResult<Stmt> {
        self.consume(TokenKind::OPEN_BRACE, "Expect '{' before block")?;

        let mut statements: Vec<Box<Stmt>> = Vec::new();
        while !self.is_empty() && !self.compare(TokenKind::CLOSE_BRACE) {
            statements.push(Box::new(self.declaration()?));
        }

        self.consume(TokenKind::CLOSE_BRACE, "Expect '}' after block.")?;
        Ok(Stmt::Block(statements))
    }
    
    fn finish_call(&mut self, callee: Expr) -> PResult<Expr> {
        let mut arguments = Vec::new();

        if !self.compare(TokenKind::CLOSE_PAREN) {
            arguments.push(Box::new(self.expression()?));

            while self.compare(TokenKind::COMMA) {
                self.advance();

                if arguments.len() >= 255 {
                    panic!("Can't have more than 255 arguments");
                }

                arguments.push(Box::new(self.expression()?));
            }
        }
        
        self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after arguments");
        
        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        })
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
        self.call()
    }

    fn call(&mut self) -> PResult<Expr> {
        let mut expr = self.primary()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::OPEN_PAREN => {
                    self.advance();
                    expr = self.finish_call(expr)?;
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> PResult<Expr> {
        match self.advance() {
            Some(tok) => match &tok.kind {
                TokenKind::INT(_)
                | TokenKind::FLOAT(_)
                | TokenKind::TRUE
                | TokenKind::FALSE
                | TokenKind::STRING(_) => Ok(Expr::Literal(tok.clone())),
                TokenKind::OPEN_PAREN => {
                    let expr = self.expression()?;
                    self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after an expression")?;
                    Ok(Expr::Group {
                        expr: Box::new(expr),
                    })
                }
                TokenKind::IDENT(_) => Ok(Expr::Var(tok.clone())),
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

    fn consume(&mut self, token_kind: TokenKind, message: &str) -> PResult<()> {
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

    fn consume_ident(&mut self, message: &str) -> PResult<Token> {
        match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(_) => Ok(self.advance().unwrap().clone()),
                _ => Err(ParseError::ExpectedToken {
                    message: message.to_string(),
                    line: tok.span.line,
                    col: tok.span.column,
                }),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn compare(&self, token_kind: TokenKind) -> bool {
        matches!(self.peek(), Some(tok) if tok.kind == token_kind)
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
