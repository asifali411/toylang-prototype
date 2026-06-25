use crate::{lexer::token::Token, parser::statement::Stmt};

#[derive(Debug, Clone)]
pub struct Function {
  parameters: Vec<Token>,
  body: Box<Stmt>,
}

impl Function {
  pub fn new(parameters: Vec<Token>, body: Box<Stmt>) -> Function {
    Self {
      parameters,
      body,
    }
  }
}