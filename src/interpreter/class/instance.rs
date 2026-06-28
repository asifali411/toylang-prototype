use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{errors::{interpreter_error::InterpreterError, lang_error::IResult}, interpreter::{class::class::Class, environment::Environment, function::Function, value::Value}};

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

  pub fn get(&self, name: String, line: usize, col: usize, this: Rc<RefCell<Instance>>) -> IResult<Value> {
    if let Some(value) = self.fields.get(&name) {
      return Ok(value.clone());
    }
    if let Some(method) = self.class.find_method(&name) {
      let bound = self.bind(method, this);
      return Ok(Value::FUNC(bound));
    }
    Err(InterpreterError::UndefinedProperty { prop: name, line, col })
  }

  pub fn bind(&self, mut method: Function, this: Rc<RefCell<Instance>>) -> Function {
    let env = Environment::new_enclosed(method.closure.clone());
    env.borrow_mut().define_var("this", Value::OBJECT(this));
    method.closure = env;
    method
  }

  pub fn class_name(&self) -> String {
    self.class.name().to_string()
  }

  pub fn set(&mut self, name: String, value: &Value) {
    self.fields.insert(name, value.clone());
  }
}