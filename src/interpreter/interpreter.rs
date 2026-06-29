use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult}, interpreter::{
        class::class::Class, environment::Environment, function::Function, signal::Signal, value::Value,
    }, lexer::token::{Token, TokenKind}, parser::{expression::Expr, statement::Stmt},
};

type Env = Rc<RefCell<Environment>>;

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
        }
    }

    pub(crate) fn execute_stmt(&mut self, statement: &Stmt) -> Result<Value, Signal> {
        match statement {
            Stmt::Expr(expr) => self.eval_expression(expr).map_err(Signal::Error),
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
            Stmt::Class { name, methods, superclass } => {
                self.eval_class_statement(name, methods, superclass).map_err(Signal::Error)
            }
        }
    }

    pub fn eval_expression(&mut self, expr: &Expr) -> IResult<Value> {
        match expr {
            Expr::Literal(literal) => match &literal.kind {
                TokenKind::NUM(n) => Ok(Value::NUM(*n)),
                TokenKind::TRUE => Ok(Value::TRUE),
                TokenKind::FALSE => Ok(Value::FALSE),
                TokenKind::STRING(s) => Ok(Value::STRING(s.to_string())),
                _ => Err(InterpreterError::UnexpectedLiteral {
                    kind: literal.kind.clone(),
                }),
            },
            Expr::Var(identifier) => {
                if let TokenKind::IDENT(name) = &identifier.kind {
                    self.lookup_var(name, expr, identifier.span.line, identifier.span.column)
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
                if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
                    self.environment
                        .borrow_mut()
                        .assign_at(depth, name, value.clone());
                } else {
                    self.environment
                        .borrow_mut()
                        .assign_var(name, value.clone(), *line, *col)?;
                }
                Ok(value)
            }
            Expr::Call { callee, arguments, line, col } => self.eval_call(callee, arguments, line, col),
            Expr::Get { object, name, line, col } => self.eval_get(object, name, line, col),
            Expr::Set { object, name, value } => self.eval_set(object, name, value),
            Expr::Array { elements } => self.eval_array(elements),
            Expr::Hashmap { fields } => self.eval_hashmap(fields),
            Expr::Index { object, index } => self.eval_index(object, index),
            Expr::IndexSet { object, index, value, line, col } => self.eval_index_set(object, index, value, *line, *col),
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
        body: &Rc<Stmt>,
    ) -> IResult<Value> {
        let func = Function::new(name.to_string(), parameters.to_vec(), body.clone(), &self.environment, false);
        self.environment.borrow_mut().define_func(name, func);
        Ok(Value::NULL)
    }

    pub fn eval_class_statement(&mut self, class_name: &str, methods: &Vec<Stmt>, superclass: &Option<Expr>) -> IResult<Value> {
        let mut functions: HashMap<String, Function> = HashMap::new();
        for method in methods {
            match method {
                Stmt::Func { name, parameters, body } => {
                    let func = Function::new(name.to_string(), parameters.clone(), body.clone(), &self.environment, name == class_name);
                    functions.insert(name.to_string(), func);
                },
                _ => {},
            }
        }

        if let Some(Expr::Var(Token { kind: TokenKind::IDENT(superclass_name), .. })) = superclass {
            if class_name == superclass_name {
                return Err(InterpreterError::InvalidStatement {
                    message: "A class cannot inherit from itself".into(),
                });
            }

            let superclass = self.eval_expression(&superclass.as_ref().unwrap())?;
            
            if let Value::CLASS(superclass) = superclass {
                let class = Class::new(class_name.to_string(), functions, Some(Box::new(superclass)));
                self.environment.borrow_mut().define_class(class_name, class);
            } else {
                return Err(InterpreterError::InvalidStatement { message: "Superclass must be a class".into() });
            }
        } else {
            let class = Class::new(class_name.to_string(), functions, None);
            self.environment.borrow_mut().define_class(class_name, class);
        }

        Ok(Value::NULL)
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
            Value::NUM(c) =>  {
                //TODO: handle negative and floating point numbers
                c as usize
            },
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

    fn lookup_var(&self, name: &str, expr: &Expr, line: usize, col: usize) -> IResult<Value> {
        if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {

            // the reason why we find the variable at "depth + 1" instead of "depth" is because of a bug that i couldnt find the source of.
            if let Some(value) = self.environment.borrow().get_at(depth + 1, name) {
                return Ok(value);
            }
        }
    
        if let Some(value) = self.globals.borrow().get_var(name, line, col) {
            return Ok(value);
        }
    
        if let Some(func) = self.environment.borrow().get_func(name) {
            return Ok(Value::FUNC(func));
        }
    
        if let Some(class) = self.environment.borrow().get_class(name) {
            return Ok(Value::CLASS(class));
        }
    
        Err(InterpreterError::UndefinedVariable { var: name.into(), line, col })
    }

    fn eval_unary(&mut self, op: &Token, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr)?;
        match op.kind {
            TokenKind::MINUS => Ok((-value)?),
            TokenKind::NOT => Ok((!value)?),
            ref kind => Err(InterpreterError::UnsupportedUnaryOp { op: kind.clone() }),
        }
    }

    fn eval_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> IResult<Value> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        let res = match op.kind {
            TokenKind::SLASH => {
                if matches!(right, Value::NUM(0.0)) {
                    return Err(InterpreterError::DivisionByZero);
                }
                left / right
            }
            TokenKind::PLUS => left + right,
            TokenKind::MINUS => left - right,
            TokenKind::STAR => left * right,
            TokenKind::LESS => Ok(left.lt(&right)),
            TokenKind::LESS_EQ => Ok(left.lt_eq(&right)),
            TokenKind::GREAT => Ok(left.gt(&right)),
            TokenKind::GREAT_EQ => Ok(left.gt_eq(&right)),
            TokenKind::EQ_EQ => Ok(left.eq(&right)),
            TokenKind::NOT_EQ => Ok(left.not_eq(&right)),
            ref kind => return Err(InterpreterError::UnsupportedBinaryOp { op: kind.clone() }),
        };

        Ok(res?)

    }

    fn eval_call(&mut self, callee: &Expr, arguments: &Vec<Box<Expr>>, line: &usize, col: &usize) -> IResult<Value> {
        let callee_value = self.eval_expression(callee)?;
    
        let args: Vec<Value> = arguments
            .iter()
            .map(|a| self.eval_expression(a))
            .collect::<IResult<_>>()?;
    
        match callee_value {
            Value::FUNC(func) => Ok(func.call(self, args)?),
            Value::CLASS(class) => Ok(class.call(self, args)?),
            Value::NativeFunction { func , ..} => {
                func(self, args)
            },
            _ => Err(InterpreterError::UndefinedFunction {
                func: format!("{:?}", callee),
                line: *line,
                col: *col,
            }),
        }
    }

    fn eval_get(&mut self, object: &Expr, name: &String, line: &usize, col: &usize) -> IResult<Value> {
        match self.eval_expression(object)? {
            Value::OBJECT(obj) => {
                let rc = Rc::clone(&obj);
                obj.borrow().get(name.clone(), *line, *col, rc)
            }
            Value::CLASS(class) => {
                let method = class.find_method(name)
                    .ok_or_else(|| InterpreterError::UndefinedProperty {
                        prop: name.clone(),
                        line: *line,
                        col: *col,
                    })?;
    
                let this = self.environment.borrow().get_var("this", *line, *col);
                if let Some(Value::OBJECT(instance)) = this {
                    let bound = instance.borrow().bind(method, Rc::clone(&instance));
                    Ok(Value::FUNC(bound))
                } else {
                    Err(InterpreterError::InvalidStatement {
                        message: "Cannot use 'super' outside of a class instance".to_string(),
                    })
                }
            }
            _ => Err(InterpreterError::InvalidStatement {
                message: "Only objects have properties".to_string(),
            })
        }
    }
    
    fn eval_set(&mut self, object: &Expr, name: &String, value: &Expr) -> IResult<Value> {
        match self.eval_expression(object)? {
            Value::OBJECT(obj) => {
                let val = self.eval_expression(value)?;
                obj.borrow_mut().set(name.clone(), &val);
                Ok(val)
            }
            _ => Err(InterpreterError::InvalidStatement {
                message: "Only objects have properties".to_string(),
            }),
        }
    }

    fn eval_array(&mut self, elements: &Vec<Box<Expr>>) -> IResult<Value> {
        let mut items: Vec<Box<Value>> = Vec::new();
        for elem in elements {
            let item = self.eval_expression(elem)?;
            items.push(Box::new(item));
        }

        Ok(Value::ARRAY(items))
    }

    fn eval_hashmap(&mut self, fields: &Vec<(String, Box<Expr>)>) -> IResult<Value> {
        let mut hashmap: HashMap<String, Box<Value>> = HashMap::new();
        for (key, value) in fields {
            let value = self.eval_expression(value)?;
            hashmap.insert(key.clone(), Box::new(value));
        }
        Ok(Value::HASHMAP(hashmap))
    }

    fn eval_index(&mut self, object: &Box<Expr>, index: &Box<Expr>) -> IResult<Value> {
        let index = self.eval_expression(index)?;
        let object = self.eval_expression(object)?;

        match object {
            Value::ARRAY(arr) => {
                match index {
                    Value::NUM(n) => {
                        if n.is_finite() && n.fract() == 0.0 {
                            let len_i = arr.len() as i64;
                            let n_i = n as i64;
                            
                            let target_index = if n_i < 0 { len_i + n_i } else { n_i };
                        
                            if target_index >= 0 && target_index < len_i {
                                return Ok((*arr[target_index as usize]).clone());
                            } else {
                                return Err(InterpreterError::InvalidStatement { 
                                    message: format!("Array index {} out of bounds for length {}", n, arr.len()) 
                                });
                            }
                        } else {
                            return Err(InterpreterError::InvalidStatement { 
                                message: "Array index must be a finite integer".into() 
                            });
                        }                        
                    },
                    _ => return Err(InterpreterError::InvalidStatement { message: "Array index must be a finite integer".into() }),
                }
            },
            _ => {}
        }

        Ok(Value::NULL)
    }

    fn eval_index_set(&mut self, object: &Expr, index: &Expr, value: &Expr, line: usize, col: usize) -> IResult<Value> {
        let index = self.eval_expression(index)?;
        let value = self.eval_expression(value)?;
    
        let var_name = match object {
            Expr::Var(tok) => match &tok.kind {
                TokenKind::IDENT(name) => name.clone(),
                _ => return Err(InterpreterError::InvalidStatement { 
                    message: "Invalid assignment target".into() 
                }),
            },
            _ => return Err(InterpreterError::InvalidStatement { 
                message: "Invalid assignment target".into() 
            }),
        };
    
        let mut arr = match self.environment.borrow().get_var(&var_name, line, col) {
            Some(Value::ARRAY(arr)) => arr,
            _ => return Err(InterpreterError::InvalidStatement { 
                message: "Index assignment target must be an array".into() 
            }),
        };
    
        match index {
            Value::NUM(n) if n.is_finite() && n.fract() == 0.0 => {
                let len_i = arr.len() as i64;
                let n_i = n as i64;
                let target_index = if n_i < 0 { len_i + n_i } else { n_i };
    
                if target_index >= 0 && target_index < len_i {
                    *arr[target_index as usize] = value;
                } else {
                    return Err(InterpreterError::InvalidStatement {
                        message: format!("Array index {} out of bounds for length {}", n_i, arr.len()),
                    });
                }
            }
            _ => return Err(InterpreterError::InvalidStatement {
                message: "Array index must be a finite integer".into(),
            }),
        }
    
        self.environment.borrow_mut().assign_var(&var_name, Value::ARRAY(arr), line, col)?;
        Ok(Value::NULL)
    }

}