use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::interpreter_error::InterpreterError,
    interpreter::{environment::Environment, signal::Signal, value::Value},
    lexer::token::{Token, TokenKind},
    parser::statement::Stmt,
};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    parameters: Vec<Token>,
    body: Vec<Box<Stmt>>,
    closure: Env,
}

impl Function {
    pub fn new(parameters: Vec<Token>, body: Box<Stmt>, environment: &Env) -> Self {
        let stmts = match *body {
            Stmt::Block(stmts) => stmts,
            other => panic!(
                "Function body must be a Block statement, got: {:?}",
                other
            ),
        };
        Self {
            parameters,
            body: stmts,
            closure: environment.clone(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        args: Vec<Value>,
    ) -> IResult<Value> {
        if args.len() != self.parameters.len() {
            return Err(InterpreterError::ArityMismatch {
                expected: self.parameters.len(),
                got: args.len(),
            });
        }

        let env = Environment::new_enclosed(self.closure.clone());
        for (param, arg) in self.parameters.iter().zip(args) {
            match &param.kind {
                TokenKind::IDENT(name) => env.borrow_mut().define_var(name, arg),
                _ => {
                    return Err(InterpreterError::InvalidParameter {
                        kind: format!("{:?}", param.kind.clone()),
                    })
                }
            }
        }

        interpreter
            .execute_block(&self.body, env)
            .or_else(|signal| match signal {
                Signal::Return(value) => Ok(value),
                Signal::Error(e) => Err(e),
            })
    }
}