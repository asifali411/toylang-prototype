use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{errors::interpreter_error::InterpreterError, interpreter::{self, environment::{self, Environment}, value::Value}, lexer::token::{Token, TokenKind}, parser::statement::Stmt};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Function {
  parameters: Vec<Token>,
  body: Box<Stmt>,
  closure: Env,
}

impl Function {
  pub fn new(parameters: Vec<Token>, body: Box<Stmt>, environment: &Env) -> Function {
    Self {
      parameters,
      body,
      closure: environment.clone(),
    }
  }

  pub fn call(
    &self,
    interpreter: &mut crate::interpreter::Interpreter,
    args: Vec<Value>
  ) -> IResult<Value> {
    
    let env = Environment::new_enclosed(self.closure.clone());
    for (param, arg) in self.parameters.iter().zip(args) {
      match &param.kind {
        TokenKind::IDENT(name) => env.borrow_mut().define_var(name, arg),
        _ => panic!("Parameter should be an identifier"),
      }
    }

    let result = match &*self.body {
      Stmt::Block(stmts) => interpreter.execute_block(stmts, env),
      _ => panic!("Unexpected behaviour"),
    }?;

    Ok(result)
  }
}