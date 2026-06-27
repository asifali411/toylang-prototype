use std::{cell::RefCell, rc::Rc};

use crate::{errors::interpreter_error::InterpreterError, interpreter::{Interpreter, class::instance::Instance, environment::{self, Environment}, signal::Signal, value::Value}};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
  name: String,
}

impl Class {
  pub fn new(name: String) -> Self {
    Self {
      name,
    }
  }

  pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
    let instance = Instance::new(self.clone());
    Value::OBJECT(instance)
  }
}