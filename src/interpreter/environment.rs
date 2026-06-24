use crate::{errors::interpreter_error::InterpreterError, interpreter::value::Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    enclosing: Option<Env>,
}

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn new_enclosed(enclosing: Env) -> Env {
        Rc::new(RefCell::new(Self {
            variables: HashMap::new(),
            enclosing: Some(enclosing),
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

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign_var(name, value, line, col);
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

        self.enclosing
            .as_ref()
            .and_then(|env| env.borrow().get_var(name, line, col))
    }

    //--------------------------------------------------------------------------
}
