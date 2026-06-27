use std::collections::HashMap;

use crate::{errors::interpreter_error::InterpreterError, interpreter::{class::class::Class, value::Value}};

type IResult<T> = Result<T, InterpreterError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
  class: Class,
  fields: HashMap<String, Value>,
}

impl Instance {
  pub fn new(class: Class) -> Self {
    Self {
      class,
      fields: HashMap::new(),
    }
  }

  pub fn get(&self, name: String, line: usize, col: usize) -> IResult<Value> {
    if let Some(value) = self.fields.get(&name) {
      return Ok(value.clone());
    }

    if let Some(method) = self.class.find_method(&name) {
      return Ok(Value::FUNC(method));
    }

    Err(InterpreterError::UndefinedProperty { prop: name, line, col })
  }

  pub fn set(&mut self, name: String, value: &Value) {
    self.fields.insert(name, value.clone());
  }
}