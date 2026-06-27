use std::{cell::{Ref, RefCell}, collections::HashMap, rc::Rc};

use crate::{errors::interpreter_error::InterpreterError, interpreter::{Interpreter, class::instance::Instance, environment::{self, Environment}, function::Function, signal::Signal, value::Value}};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

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

  pub fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
    let instance = Instance::new(self.clone());
    if let Some(init) = self.find_method(&self.name) {
      let func = instance.bind(init);
      func.call(interpreter, arguments);
    }

    Value::OBJECT(Rc::new(RefCell::new(instance)))
  }

  pub fn arity(&self) -> usize {
    if let Some(init) = self.find_method(&self.name) {
      init.arity()
    } else {
      0
    }
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