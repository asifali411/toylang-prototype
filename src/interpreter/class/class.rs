use crate::interpreter::{Interpreter, class::instance::Instance, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
  name: String
}

impl Class {
  pub fn new(name: String) -> Self {
    Self {
      name,
    }
  }

  pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Instance {
    let instance = Instance::new(self.clone());
    instance
  }
}