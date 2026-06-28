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
    name: String,
    parameters: Vec<Token>,
    body: Rc<Stmt>,
    pub closure: Env,
    is_init: bool,
}

impl Function {
    pub fn new(name: String, parameters: Vec<Token>, body: Rc<Stmt>, environment: &Env, is_init: bool) -> Self {
        if let Stmt::Block(_) = body.as_ref() {
        } else {
            panic!("Function body must be a Block statement");
        }
        Self {
            name,
            parameters,
            body,
            closure: environment.clone(),
            is_init,
        }
    }

    pub fn func_name(&self) -> &str {
        &self.name
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
            };
        }

        let previous = std::mem::replace(&mut interpreter.environment, env);
        let result = interpreter.execute_stmt(self.body.as_ref());
        interpreter.environment = previous;

        match result {
            Ok(_) => Ok(Value::NULL),
            Err(Signal::Return(_)) if self.is_init => Ok(Value::NULL),
            Err(Signal::Return(value)) => Ok(value),
            Err(Signal::Error(e)) => Err(e),
        }
    }
}