use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::interpreter_error::InterpreterError, interpreter::{
        environment::Environment, function::Function, signal::Signal, value::Value,
    }, lexer::token::{Token, TokenKind}, parser::{expression::Expr, statement::Stmt},
};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

pub struct Interpreter {
    pub environment: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn execute(&mut self, statement: &Stmt) -> IResult<Value> {
        match self.execute_stmt(statement) {
            Ok(v) => Ok(v),
            Err(Signal::Error(e)) => Err(e),
            Err(Signal::Return(v)) => Ok(v),
        }
    }

    fn execute_stmt(&mut self, statement: &Stmt) -> Result<Value, Signal> {
        match statement {
            Stmt::Expr(expr) => self.eval_expression(expr).map_err(Signal::Error),
            Stmt::Print(expr) => self.execute_print_statement(expr).map_err(Signal::Error),
            Stmt::Var { name, initializer } => {
                self.eval_var_statement(name, initializer).map_err(Signal::Error)
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts, Environment::new_enclosed(self.environment.clone()))
            }
            Stmt::If { condition, if_body, else_body } => {
                self.execute_if_statement(condition, if_body, else_body)
            }
            Stmt::Loop { count, body } => {
                self.execute_loop_statement(count, body)
            }
            Stmt::LoopIf { condition, body } => {
                self.execute_loop_if_statement(condition, body)
            }
            Stmt::Func { name, parameters, body } => {
                self.eval_func_statement(name, parameters, body).map_err(Signal::Error)
            }
            Stmt::Return(expr) => {
                let value = self.eval_expression(expr).map_err(Signal::Error)?;
                Err(Signal::Return(value))
            }
        }
    }

    pub fn eval_expression(&mut self, expr: &Expr) -> IResult<Value> {
        match expr {
            Expr::Literal(literal) => match &literal.kind {
                TokenKind::INT(n) => Ok(Value::INT(*n)),
                TokenKind::FLOAT(n) => Ok(Value::FLOAT(*n)),
                TokenKind::TRUE => Ok(Value::TRUE),
                TokenKind::FALSE => Ok(Value::FALSE),
                TokenKind::STRING(s) => Ok(Value::STRING(s.to_string())),
                _ => Err(InterpreterError::UnexpectedLiteral {
                    kind: literal.kind.clone(),
                }),
            },
            Expr::Var(identifier) => {
                if let TokenKind::IDENT(name) = &identifier.kind {
                    self.lookup_var(name, identifier.span.line, identifier.span.column)
                } else {
                    panic!(
                        "Expected variable identifier, but found: {:?}",
                        identifier
                    )
                }
            }
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::Binary { left, operator, right } => self.eval_binary(left, operator, right),
            Expr::Group { expr } => self.eval_expression(expr),
            Expr::Assign { name, value, line, col } => {
                let value = self.eval_expression(value)?;
                self.environment
                    .borrow_mut()
                    .assign_var(name, value.clone(), *line, *col)?;
                Ok(value)
            }
            Expr::Call { callee, arguments } => self.eval_call(callee, arguments),
            _ => Err(InterpreterError::UnexpectedExpr),
        }
    }

    pub fn eval_var_statement(
        &mut self,
        name: &String,
        expr: &Option<Expr>,
    ) -> IResult<Value> {
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
        body: &Box<Stmt>,
    ) -> IResult<Value> {
        let func = Function::new(parameters.to_vec(), body.clone(), &self.environment);
        self.environment.borrow_mut().define_func(name, func);
        Ok(Value::NULL)
    }

    fn execute_print_statement(&mut self, expr: &Expr) -> IResult<Value> {
        let val = self.eval_expression(expr)?;
        match &val {
            Value::INT(n) => println!("{}", n),
            Value::FLOAT(n) => println!("{}", n),
            Value::NULL => println!("null"),
            Value::TRUE => println!("true"),
            Value::FALSE => println!("false"),
            Value::STRING(v) => println!("{}", v),
            other => println!("{:?}", other),
        }
        Ok(val)
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

    fn execute_if_statement(
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

    fn execute_loop_statement(
        &mut self,
        count: &Expr,
        body: &Box<Stmt>,
    ) -> Result<Value, Signal> {
        let count = match self.eval_expression(count).map_err(Signal::Error)? {
            Value::INT(c) => c,
            _ => return Err(Signal::Error(InterpreterError::UnexpectedExpr)),
        };

        for _ in 0..count {
            self.execute_stmt(body)?;
        }

        Ok(Value::NULL)
    }

    fn execute_loop_if_statement(
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

    //-----------------------------------------------------------------------------

    fn lookup_var(&self, name: &str, line: usize, col: usize) -> IResult<Value> {
        self.environment
            .borrow()
            .get_var(name, line, col)
            .ok_or(InterpreterError::UndefinedVariable {
                var: name.to_string(),
                line,
                col,
            })
    }

    fn eval_unary(&mut self, op: &Token, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr)?;
        match op.kind {
            TokenKind::MINUS => Ok(-value),
            TokenKind::NOT => Ok(!value),
            ref kind => Err(InterpreterError::UnsupportedUnaryOp { op: kind.clone() }),
        }
    }

    fn eval_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> IResult<Value> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match op.kind {
            TokenKind::SLASH => {
                if matches!(right, Value::INT(0)) {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(left / right)
            }
            TokenKind::PLUS => Ok(left + right),
            TokenKind::MINUS => Ok(left - right),
            TokenKind::STAR => Ok(left * right),
            TokenKind::LESS => Ok(left.lt(&right)),
            TokenKind::LESS_EQ => Ok(left.lt_eq(&right)),
            TokenKind::GREAT => Ok(left.gt(&right)),
            TokenKind::GREAT_EQ => Ok(left.gt_eq(&right)),
            TokenKind::EQ_EQ => Ok(left.eq(&right)),
            TokenKind::NOT_EQ => Ok(left.not_eq(&right)),
            ref kind => Err(InterpreterError::UnsupportedBinaryOp { op: kind.clone() }),
        }
    }

    fn eval_call(&mut self, callee: &Expr, arguments: &Vec<Box<Expr>>) -> IResult<Value> {
        let name = match callee {
            Expr::Var(token) => {
                if let TokenKind::IDENT(n) = &token.kind {
                    n.clone()
                } else {
                    return Err(InterpreterError::UnexpectedExpr);
                }
            }
            _ => return Err(InterpreterError::UnexpectedExpr),
        };

        let func = self
            .environment
            .borrow()
            .get_func(&name)
            .ok_or_else(|| InterpreterError::UndefinedVariable {
                var: name.clone(),
                line: 0,
                col: 0,
            })?;

        let args: Vec<Value> = arguments
            .iter()
            .map(|a| self.eval_expression(a))
            .collect::<IResult<_>>()?;

        func.call(self, args)
    }
}