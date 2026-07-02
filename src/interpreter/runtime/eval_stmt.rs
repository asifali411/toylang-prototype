use crate::{
    errors::{interpreter_error::InterpreterError}, interpreter::{
        Interpreter,
        environment::{Env, Environment},
        signal::Signal,
        value::Value,
    }, parser::{expression::Expr, statement::Stmt},
};

impl Interpreter {
    pub(crate) fn execute_stmt(&mut self, statement: &Stmt) -> Result<Value, Signal> {
        match statement {
            Stmt::Expr(expr) => self.eval_expression(expr).map_err(Signal::Error),
            Stmt::Var { name, initializer } => self
                .eval_var_statement(name, initializer)
                .map_err(Signal::Error),
            Stmt::Block(stmts) => {
                self.execute_block(stmts, Environment::new_enclosed(self.environment.clone()))
            }
            Stmt::If {
                condition,
                if_body,
                else_body,
            } => self.execute_if_statement(condition, if_body, else_body),
            Stmt::Loop { count, body } => self.execute_loop_statement(count, body),
            Stmt::LoopIf { condition, body } => self.execute_loop_if_statement(condition, body),
            Stmt::LoopIn { name, object, body } => self.execute_loop_in_statement(name, object, body),
            Stmt::Func {
                name,
                parameters,
                body,
            } => self
                .eval_func_statement(name, parameters, body)
                .map_err(Signal::Error),
            Stmt::Return(expr) => {
                let value = self.eval_expression(expr).map_err(Signal::Error)?;
                Err(Signal::Return(value))
            }
            Stmt::Class {
                name,
                methods,
                superclass,
            } => self
                .eval_class_statement(name, methods, superclass)
                .map_err(Signal::Error),
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Box<Stmt>>,
        environment: Env,
    ) -> Result<Value, Signal> {
        let previous = std::mem::replace(&mut self.environment, environment);
        let mut result: Result<Value, Signal> = Ok(Value::NULL);

        for statement in statements {
            result = self.execute_stmt(statement);
            if result.is_err() {
                break;
            }
        }

        self.environment = previous;
        result
    }

    pub(crate) fn execute_if_statement(
        &mut self,
        condition: &Expr,
        if_body: &Box<Stmt>,
        else_body: &Option<Box<Stmt>>,
    ) -> Result<Value, Signal> {
        if self
            .eval_expression(condition)
            .map_err(Signal::Error)?
            .is_true()
        {
            self.execute_stmt(if_body)
        } else if let Some(else_body) = else_body {
            self.execute_stmt(else_body)
        } else {
            Ok(Value::NULL)
        }
    }

    pub(crate) fn execute_loop_statement(
        &mut self,
        count: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, Signal> {
        let count = match self.eval_expression(count).map_err(Signal::Error)? {
            Value::NUM(c) => {
                //TODO: handle negative and floating point numbers
                c as usize
            }
            _ => return Err(Signal::Error(InterpreterError::UnexpectedExpr)),
        };

        for _ in 0..count {
            self.execute_stmt(body)?;
        }

        Ok(Value::NULL)
    }

    pub(crate) fn execute_loop_if_statement(
        &mut self,
        condition: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, Signal> {
        while self
            .eval_expression(condition)
            .map_err(Signal::Error)?
            .is_true()
        {
            self.execute_stmt(body)?;
        }
        Ok(Value::NULL)
    }

    pub(crate) fn execute_loop_in_statement(
        &mut self,
        name: &String,
        object: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, Signal> {
        let value = self.eval_expression(object).map_err(Signal::Error)?;
    
        match value {
            Value::ARRAY(arr) => {
                let env = Environment::new_enclosed(self.environment.clone());
                let previous = std::mem::replace(&mut self.environment, env);

                self.environment.borrow_mut().define_var(name, Value::NULL);
    
                let len = arr.borrow().len();
                let mut result = Ok(Value::NULL);
    
                for i in 0..len {
                    let item = arr.borrow()[i].clone();
                    if let Err(e) = self.environment.borrow_mut().assign_var(name, *item, 0, 0) {
                        result = Err(Signal::Error(e));
                        break;
                    }
                    result = self.execute_stmt(body);
                    if result.is_err() {
                        break;
                    }
                }
    
                self.environment = previous;
                Ok(result?)
            }
            _ => Err(Signal::Error(InterpreterError::UnexpectedExpr)),
        }
    }
}

