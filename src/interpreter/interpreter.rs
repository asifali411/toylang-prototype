use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::lang_error::IResult,
    interpreter::{environment::Environment, function::Function, signal::Signal, value::Value},
    lexer::token::Token,
    parser::{expression::Expr, statement::Stmt},
};

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Interpreter {
    pub environment: Env,
    pub globals: Env,
    pub locals: HashMap<*const Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new();
        Self {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statement: &Stmt) -> IResult<Value> {
        match self.execute_stmt(statement) {
            Ok(v) => Ok(v),
            Err(Signal::Error(e)) => Err(e),
            Err(Signal::Return(v)) => Ok(v),
            Err(Signal::Break) | Err(Signal::Continue) => Ok(Value::NULL),
        }
    }

    pub fn eval_var_statement(&mut self, name: &String, expr: &Option<Expr>) -> IResult<Value> {
        let value = match expr {
            Some(e) => self.eval_expression(e)?,
            None => Value::NULL,
        };
        self.environment.borrow_mut().define_var(name, value);
        Ok(Value::NULL)
    }

    pub fn eval_func_statement(
        &mut self,
        name: &String,
        parameters: &Vec<Token>,
        body: &Rc<Stmt>,
    ) -> IResult<Value> {
        let func = Function::new(
            name.to_string(),
            parameters.to_vec(),
            body.clone(),
            &self.environment,
            false,
        );
        self.environment.borrow_mut().define_func(name, func);
        Ok(Value::NULL)
    }
}

