use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::interpreter_error::InterpreterError,
    interpreter::{
        environment::{self, Environment},
        value::Value,
    },
    lexer::token::{Token, TokenKind},
    parser::{expression::Expr, statement::Stmt},
};

type IResult<T> = Result<T, InterpreterError>;
type Env = Rc<RefCell<Environment>>;

pub struct Interpreter {
    environment: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn execute(&mut self, statement: &Stmt) -> IResult<Value> {
        match statement {
            Stmt::Expr(expr) => self.eval_expression(expr),
            Stmt::Print(expr) => self.execute_print_statement(expr),
            Stmt::Var { name, initializer } => self.eval_var_statement(name, initializer),
            Stmt::Block(expr) => {
                self.execute_block(expr, Environment::new_enclosed(self.environment.clone()))
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
                TokenKind::IDENT(var) => {
                    match self.environment.borrow_mut().get_var(
                        &var,
                        literal.span.line,
                        literal.span.column,
                    ) {
                        Some(value) => Ok(value),
                        None => Err(InterpreterError::UndefinedVariable {
                            var: var.to_string(),
                            line: literal.span.line,
                            col: literal.span.column,
                        }),
                    }
                }
                _ => Err(InterpreterError::UnexpectedLiteral {
                    kind: literal.kind.clone(),
                }),
            },
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(left, operator, right),
            Expr::Group { expr } => self.eval_expression(expr),
            Expr::Var(identifier) => match &identifier.kind {
                TokenKind::IDENT(name) => {
                    (match self.environment.borrow_mut().get_var(
                        &name,
                        identifier.span.line,
                        identifier.span.column,
                    ) {
                        Some(value) => Ok(value),
                        None => {
                            return Err(InterpreterError::UndefinedVariable {
                                var: name.to_string(),
                                line: identifier.span.line,
                                col: identifier.span.column,
                            });
                        }
                    })
                }
                _ => panic!("Expected variable identifier, but found: {:?}", identifier),
            },
            Expr::Assign {
                name,
                value,
                line,
                col,
            } => {
                let value = self.eval_expression(value)?;

                self.environment
                    .borrow_mut()
                    .assign_var(name, value.clone(), *line, *col);

                Ok(value)
            }
            _ => Err(InterpreterError::UnexpectedExpr),
        }
    }

    pub fn eval_var_statement(&mut self, name: &String, expr: &Option<Expr>) -> IResult<Value> {
        let value = match expr {
            Some(e) => self.eval_expression(e),
            None => Ok(Value::NULL),
        };

        match value {
            Err(e) => Err(e),
            Ok(val) => {
                self.environment.borrow_mut().define_var(name, val);

                Ok(Value::NULL)
            }
        }
    }

    fn execute_print_statement(&mut self, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr);
        match value {
            Err(e) => return Err(e),
            Ok(val) => {
                match val {
                    Value::INT(n) => println!("{}", n),
                    Value::FLOAT(n) => println!("{}", n),
                    Value::NULL => println!("null"),
                    Value::TRUE => println!("true"),
                    Value::FALSE => println!("false"),
                    _ => println!("{:?}", val),
                };
                return Ok(val);
            }
        };
    }

    fn execute_block(&mut self, statements: &Vec<Box<Stmt>>, environment: Env) -> IResult<Value> {
        let previous = std::mem::replace(&mut self.environment, environment);
        let mut return_value: Value = Value::NULL;

        for statement in statements {
            let value = self.execute(&statement);
            match value {
                _ => {}
            };
        }

        self.environment = previous;
        Ok(return_value)
    }

    //-----------------------------------------------------------------------------

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
            TokenKind::PLUS => Ok(left + right),
            TokenKind::MINUS => Ok(left - right),
            TokenKind::STAR => Ok(left * right),
            TokenKind::SLASH => Ok(left / right),
            TokenKind::LESS => Ok(left.lt(&right)),
            TokenKind::LESS_EQ => Ok(left.lt_eq(&right)),
            TokenKind::GREAT => Ok(left.gt(&right)),
            TokenKind::GREAT_EQ => Ok(left.gt_eq(&right)),
            TokenKind::EQ_EQ => Ok(left.eq(&right)),
            TokenKind::NOT_EQ => Ok(left.not_eq(&right)),
            ref kind => Err(InterpreterError::UnsupportedBinaryOp { op: kind.clone() }),
        }
    }
}
