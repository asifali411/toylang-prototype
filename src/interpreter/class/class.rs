use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::{interpreter_error::InterpreterError, lang_error::IResult}, interpreter::{Interpreter, class::instance::Instance, function::Function, value::Value}};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
  name: String,
  methods: HashMap<String, Function>,
}

impl Class {
  pub fn new(name: String, methods: HashMap<String, Function>) -> Self {
    Self {
      name,
      methods,
    }
  }

  pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> IResult<Value> {
    let instance = Rc::new(RefCell::new(Instance::new(self.clone())));
    if let Some(init) = self.find_method(&self.name) {
      let func = instance.borrow().bind(init, Rc::clone(&instance));
      let ret = func.call(interpreter, arguments)?;

      if ret != Value::NULL {
        return Err(InterpreterError::InvalidStatement { message: "Cannot return value from an initializer".to_string() })
      }
    }
    Ok(Value::OBJECT(instance))
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn find_method(&self, name: &str) -> Option<Function> {
    if let Some(method) = self.methods.get(name) {
      return Some(method.clone());
    }

    None
  }
}