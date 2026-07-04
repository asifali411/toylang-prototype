use crate::{
    errors::interpreter_error::InterpreterError,
    interpreter::{class::class::Class, function::Function, value::Value},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_at(&self, depth: usize, name: &str) -> Option<Value> {
        if depth == 0 {
            return self.variables.get(name).cloned();
        }

        self.enclosing
            .as_ref()
            .and_then(|e| e.borrow().get_at(depth - 1, name))
    }

    pub fn assign_at(&mut self, depth: usize, name: &str, value: Value) -> Option<()> {
        if depth == 0 {
            self.variables.insert(name.to_string(), value);
            return Some(());
        }
        self.enclosing
            .as_ref()
            .and_then(|e| e.borrow_mut().assign_at(depth - 1, name, value))
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
            name: name.into(),
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

    pub fn define_func(&mut self, name: impl Into<String>, func: Function) {
        let name = name.into();
        self.variables.insert(name, Value::FUNC(func));
    }

    pub fn get_func(&self, name: &str) -> Option<Function> {
        if let Some(Value::FUNC(func)) = self.variables.get(name) {
            return Some(func.clone());
        }

        self.enclosing
            .as_ref()
            .and_then(|env| env.borrow().get_func(name))
    }

    //--------------------------------------------------------------------------

    pub fn define_class(&mut self, name: impl Into<String>, class: Class) {
        let name = name.into();
        self.variables.insert(name, Value::CLASS(class));
    }

    pub fn get_class(&self, name: &str) -> Option<Class> {
        if let Some(Value::CLASS(class)) = self.variables.get(name) {
            return Some(class.clone());
        }

        self.enclosing
            .as_ref()
            .and_then(|env: &Rc<RefCell<Environment>>| env.borrow().get_class(name))
    }
}
