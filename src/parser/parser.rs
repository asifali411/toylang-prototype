use crate::errors::parse_error::ParseError::{self, UnexpectedEof};
use crate::parser::statement::Stmt;
use crate::{
    lexer::token::{Token, TokenKind},
    parser::expression::Expr,
};
use std::rc::Rc;

type PResult<T> = Result<T, ParseError>;

const MAX_ARGUMENTS: usize = 255;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
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

    // ---------------------------------------------------------------
    // Declarations
    // ---------------------------------------------------------------

    fn declaration(&mut self) -> PResult<Stmt> {
        match self.peek().cloned().ok_or(ParseError::UnexpectedEof)?.kind {
            TokenKind::VAR => self.var_declaration(),
            TokenKind::FUNC => self.func_declaration(),
            TokenKind::CLASS => self.class_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> PResult<Stmt> {
        self.advance();

        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone(),
                _ => {
                    return Err(ParseError::ExpectedName {
                        keyword: "var",
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
                    return Err(ParseError::ExpectedName {
                        keyword: "func",
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

                if parameters.len() >= MAX_ARGUMENTS {
                    let tok = self.peek().ok_or(ParseError::UnexpectedEof)?;
                    return Err(ParseError::InvalidStatement {
                        message: format!("Can't have more than {MAX_ARGUMENTS} parameters"),
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }

                parameters.push(self.consume_ident("Expect parameter name")?);
            }
        }

        self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after parameters")?;
        let body = self.block()?;

        Ok(Stmt::Func {
            name,
            parameters,
            body: Rc::new(body),
        })
    }

    fn class_declaration(&mut self) -> PResult<Stmt> {
        self.advance();
        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone(),
                _ => {
                    return Err(ParseError::ExpectedName {
                        keyword: "class",
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }
            },
            None => return Err(ParseError::UnexpectedEof),
        };
        self.advance();
        let superclass = match self.peek() {
            Some(tok) => match tok.kind {
                TokenKind::INHERIT => {
                    self.advance();
                    Some(Expr::Var(self.consume_ident(
                        "Expect superclass name after 'inherit' keyword",
                    )?))
                }
                _ => None,
            },
            None => return Err(UnexpectedEof),
        };

        self.consume(TokenKind::OPEN_BRACE, "Expect '{' before class body")?;

        let mut methods: Vec<Stmt> = Vec::new();

        while !self.is_empty() && !self.compare(TokenKind::CLOSE_BRACE) {
            methods.push(self.func_declaration()?);
        }

        self.consume(TokenKind::CLOSE_BRACE, "Expect '}' after class body")?;

        Ok(Stmt::Class {
            name,
            methods,
            superclass,
        })
    }

    // ---------------------------------------------------------------
    // Statements
    // ---------------------------------------------------------------

    fn statement(&mut self) -> PResult<Stmt> {
        match self.peek() {
            Some(tok) => match tok.kind {
                TokenKind::OPEN_BRACE => self.block(),
                TokenKind::IF => self.if_statement(),
                TokenKind::LOOP => match self.peek_next() {
                    Some(tok) => {
                        match &tok.kind {
                            TokenKind::IF => return self.loop_if_statement(),
                            TokenKind::IDENT(_) => {
                                if let Some(in_tok) = self.peek_nth(2) {
                                    if in_tok.kind == TokenKind::IN {
                                        return self.loop_in_statement();
                                    }
                                }
                            },
                            _ => {}
                        };

                        self.loop_statement()
                    }
                    _ => self.loop_statement(),
                },
                TokenKind::RETURN => self.return_statement(),
                TokenKind::BREAK => {
                    self.advance();
                    self.consume(TokenKind::SEMI, "Expect ';' after break statement")?;
                    Ok(Stmt::Break)
                },
                TokenKind::CONTINUE => {
                    self.advance();
                    self.consume(TokenKind::SEMI, "Expect ';' after continue statement")?;
                    Ok(Stmt::Continue)
                }
                _ => self.expression_statement(),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn expression_statement(&mut self) -> PResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::SEMI, "Expect ';' after expression")?;
        Ok(Stmt::Expr(expr))
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
        self.consume(TokenKind::LOOP, "Expect 'loop'")?;
        self.consume(TokenKind::IF, "Expect 'if' after 'loop'")?;
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

    fn loop_in_statement(&mut self) -> PResult<Stmt> {
        self.advance();

        let name = match self.peek() {
            Some(tok) => match &tok.kind {
                TokenKind::IDENT(v) => v.clone(),
                _ => {
                    return Err(ParseError::ExpectedName {
                        keyword: "loop",
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }
            },
            None => return Err(ParseError::UnexpectedEof),
        };
        self.advance();
        self.consume(TokenKind::IN, "Expect 'in' after identifier")?;
        let object = self.expression()?;
        let body = Box::new(self.block()?);

        Ok(Stmt::LoopIn { name, object, body })
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

        self.consume(TokenKind::CLOSE_BRACE, "Expect '}' after block")?;
        Ok(Stmt::Block(statements))
    }

    fn finish_call(&mut self, callee: Expr, line: usize, col: usize) -> PResult<Expr> {
        let mut arguments: Vec<Box<Expr>> = Vec::new();

        if !self.compare(TokenKind::CLOSE_PAREN) {
            arguments.push(Box::new(self.expression()?));

            while self.compare(TokenKind::COMMA) {
                self.advance();

                if arguments.len() >= MAX_ARGUMENTS {
                    let tok = self.peek().ok_or(ParseError::UnexpectedEof)?;
                    return Err(ParseError::InvalidStatement {
                        message: format!("Can't have more than {MAX_ARGUMENTS} arguments"),
                        line: tok.span.line,
                        col: tok.span.column,
                    });
                }

                arguments.push(Box::new(self.expression()?));
            }
        }

        self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after arguments")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
            line,
            col,
        })
    }

    fn create_array(&mut self) -> PResult<Expr> {
        let mut elements = Vec::new();

        if !self.compare(TokenKind::CLOSE_BRACK) {
            loop {
                elements.push(Box::new(self.expression()?));

                if !self.compare(TokenKind::COMMA) {
                    break;
                }

                self.advance();
            }
        }

        self.consume(TokenKind::CLOSE_BRACK, "Expect ']' after array.")?;

        return Ok(Expr::Array { elements });
    }

    fn create_hashmap(&mut self) -> PResult<Expr> {
        let mut fields: Vec<(String, Box<Expr>)> = Vec::new();

        if !self.compare(TokenKind::CLOSE_BRACE) {
            loop {
                let key_tok = self.consume_ident("Expect key name in hashmap")?;
                let key = match key_tok.kind {
                    TokenKind::IDENT(name) => name,
                    _ => unreachable!(),
                };

                self.consume(TokenKind::COLON, "Expect ':' after hashmap key")?;

                let value = self.expression()?;
                fields.push((key, Box::new(value)));

                if self.compare(TokenKind::COMMA) {
                    self.advance();
                    if self.compare(TokenKind::CLOSE_BRACE) {
                        break;
                    } // trailing comma
                } else {
                    break;
                }
            }
        }

        self.consume(TokenKind::CLOSE_BRACE, "Expect '}' after hashmap body")?;

        Ok(Expr::Hashmap { fields })
    }

    // ---------------------------------------------------------------
    // Expressions
    // ---------------------------------------------------------------

    fn expression(&mut self) -> PResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> PResult<Expr> {
        let expr = self.equality()?;

        if self.compare(TokenKind::EQUAL) {
            let equals = self.advance_token()?;
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
            } else if let Expr::Get { object, name, .. } = expr {
                return Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                });
            } else if let Expr::Index { object, index } = expr {
                return Ok(Expr::IndexSet {
                    object,
                    index,
                    value: Box::new(value),
                    line: equals.span.line,
                    col: equals.span.column,
                });
            }

            return Err(ParseError::InvalidStatement {
                message: "Invalid assignment target".to_string(),
                line: equals.span.line,
                col: equals.span.column,
            });
        }

        if self.compare(TokenKind::PLUS_EQ)
            || self.compare(TokenKind::MINUS_EQ)
            || self.compare(TokenKind::STAR_EQ)
            || self.compare(TokenKind::SLASH_EQ)
            || self.compare(TokenKind::MOD_EQ)
        {
            let op = self.advance_token()?;
            let value = self.assignment()?;

            if let Expr::Var(ref tok) = expr {
                if let TokenKind::IDENT(name) = &tok.kind {
                    return Ok(Expr::CompoundAssign {
                        name: name.to_string(),
                        value: Box::new(value),
                        op,
                    });
                }
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> PResult<Expr> {
        let mut expr = self.logical()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::NOT_EQ | TokenKind::EQ_EQ => {
                    let op = self.advance_token()?;
                    let right = self.logical()?;
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

    fn logical(&mut self) -> PResult<Expr> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.peek() {
            match op.kind {
                TokenKind::AND | TokenKind::OR => {
                    let op = self.advance_token()?;
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
                TokenKind::LESS | TokenKind::GREAT 
                | TokenKind::LESS_EQ | TokenKind::GREAT_EQ
                | TokenKind::IN => {
                    let op = self.advance_token()?;
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
                    let op = self.advance_token()?;
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
                TokenKind::STAR | TokenKind::SLASH | TokenKind::MOD => {
                    let op = self.advance_token()?;
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
            if tok.kind == TokenKind::MINUS
                || tok.kind == TokenKind::NOT
                || tok.kind == TokenKind::INC
                || tok.kind == TokenKind::DEC
            {
                let op = self.advance_token()?;
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

        loop {
            let tok = self.peek();
            if tok == None {
                break;
            }
            let tok = tok.unwrap().clone();
            match &tok.kind {
                TokenKind::OPEN_PAREN => {
                    if let Some(tok) = self.advance() {
                        let tok = tok.clone();
                        expr = self.finish_call(expr, tok.span.line, tok.span.column)?;
                    } else {
                        return Err(ParseError::UnexpectedEof);
                    }
                }
                TokenKind::DOT => {
                    self.advance();
                    let tok = self.consume_ident("Expect property name after '.'")?;

                    if let TokenKind::IDENT(name) = tok.kind {
                        let line = tok.span.line;
                        let col = tok.span.column;

                        expr = Expr::Get {
                            object: Box::new(expr),
                            name,
                            line,
                            col,
                        };
                    }
                }
                TokenKind::OPEN_BRACK => {
                    self.advance();
                    let index = self.expression()?;
                    self.consume(TokenKind::CLOSE_BRACK, "Expect ']' after array indexing")?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    }
                }
                TokenKind::INC | TokenKind::DEC => {
                    let op = self.advance_token()?;
                    expr = Expr::PostUnary {
                        operator: op,
                        left: Box::new(expr),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> PResult<Expr> {
        match self.advance() {
            Some(tok) => match &tok.kind {
                TokenKind::NUM(_) | TokenKind::TRUE | TokenKind::FALSE | TokenKind::STRING(_) => {
                    Ok(Expr::Literal(tok.clone()))
                }
                TokenKind::OPEN_PAREN => {
                    let expr = self.expression()?;
                    self.consume(TokenKind::CLOSE_PAREN, "Expect ')' after expression")?;
                    Ok(Expr::Group {
                        expr: Box::new(expr),
                    })
                }
                TokenKind::IDENT(_) => Ok(Expr::Var(tok.clone())),
                TokenKind::OPEN_BRACK => self.create_array(),
                TokenKind::OPEN_BRACE => self.create_hashmap(),
                _ => Err(ParseError::UnexpectedToken {
                    token: tok.to_string(),
                    line: tok.span.line,
                    col: tok.span.column,
                }),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    // ---------------------------------------------------------------
    // Primitives
    // ---------------------------------------------------------------

    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens
            .get(self.current)
            .filter(|t| t.kind != TokenKind::EOF)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens
            .get(self.current + 1)
            .filter(|t| t.kind != TokenKind::EOF)
    }

    fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens
            .get(self.current + n)
            .filter(|t| t.kind != TokenKind::EOF)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_empty() {
            return None;
        }
        let tok = &self.tokens[self.current];
        self.current += 1;
        Some(tok)
    }

    fn advance_token(&mut self) -> PResult<Token> {
        self.advance().cloned().ok_or(ParseError::UnexpectedEof)
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
                TokenKind::IDENT(_) => self.advance_token(),
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
}

