use crate::{errors::interpreter_error::InterpreterError, interpreter::value::Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
        }))
    }

    //--------------------------------------------------------------------------

    pub fn define_var(&mut self, name: impl Into<String>, value: Value) {
        self.variables.insert(name.into(), value);
    }

    pub fn assign_var(
        &mut self,
        name: &str,
        value: Value,
        line: usize,
        col: usize,
    ) -> Result<(), InterpreterError> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            return Ok(());
        }

        Err(InterpreterError::UndefinedVariable {
            var: name.to_string(),
            line,
            col,
        })
    }

    pub fn get_var(&self, name: &str, line: usize, col: usize) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            return Some(value.clone());
        }
        None
    }

    //--------------------------------------------------------------------------
}
