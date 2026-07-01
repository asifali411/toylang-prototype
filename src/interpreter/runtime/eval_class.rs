use std::{collections::HashMap, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult},
    interpreter::{Interpreter, class::class::Class, function::Function, value::Value},
    lexer::token::{Token, TokenKind},
    parser::{expression::Expr, statement::Stmt},
};

impl Interpreter {
    pub(crate) fn eval_class_statement(
        &mut self,
        class_name: &str,
        methods: &Vec<Stmt>,
        superclass: &Option<Expr>,
    ) -> IResult<Value> {
        let mut functions: HashMap<String, Function> = HashMap::new();
        for method in methods {
            match method {
                Stmt::Func {
                    name,
                    parameters,
                    body,
                } => {
                    let func = Function::new(
                        name.to_string(),
                        parameters.clone(),
                        body.clone(),
                        &self.environment,
                        name == class_name,
                    );
                    functions.insert(name.to_string(), func);
                }
                _ => {}
            }
        }

        if let Some(Expr::Var(Token {
            kind: TokenKind::IDENT(superclass_name),
            ..
        })) = superclass
        {
            if class_name == superclass_name {
                return Err(InterpreterError::InvalidStatement {
                    message: "A class cannot inherit from itself".into(),
                });
            }

            let superclass = self.eval_expression(&superclass.as_ref().unwrap())?;

            if let Value::CLASS(superclass) = superclass {
                let class = Class::new(
                    class_name.to_string(),
                    functions,
                    Some(Box::new(superclass)),
                );
                self.environment
                    .borrow_mut()
                    .define_class(class_name, class);
            } else {
                return Err(InterpreterError::InvalidStatement {
                    message: "Superclass must be a class".into(),
                });
            }
        } else {
            let class = Class::new(class_name.to_string(), functions, None);
            self.environment
                .borrow_mut()
                .define_class(class_name, class);
        }

        Ok(Value::NULL)
    }

    pub(crate) fn eval_get(
        &mut self,
        object: &Expr,
        name: &String,
        line: &usize,
        col: &usize,
    ) -> IResult<Value> {
        match self.eval_expression(object)? {
            Value::HASHMAP(hashmap) => {
                if let Some(res) = hashmap.borrow().get(name) {
                    Ok(*res.clone())
                } else {
                    Err(InterpreterError::UndefinedProperty { 
                        name: name.into(), 
                        line: *line, 
                        col: *col 
                    })
                }
            }
            Value::OBJECT(obj) => {
                let rc = Rc::clone(&obj);
                obj.borrow().get(name.clone(), *line, *col, rc)
            }
            Value::CLASS(class) => {
                let method =
                    class
                        .find_method(name)
                        .ok_or_else(|| InterpreterError::UndefinedProperty {
                            name: name.into(),
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
            Value::SUPER { class, instance } => instance
                .borrow()
                .get_super(&class, name.clone(), *line, *col, Rc::clone(&instance)),
            _ => Err(InterpreterError::InvalidStatement {
                message: "Only objects have properties".to_string(),
            }),
        }
    }

    pub(crate) fn eval_set(
        &mut self,
        object: &Expr,
        name: &String,
        value: &Expr,
    ) -> IResult<Value> {
        match self.eval_expression(object)? {
            Value::HASHMAP(hashmap) => {
                let val = self.eval_expression(value)?;
                hashmap.borrow_mut().insert(name.into(), Box::new(val.clone()));
                Ok(val)
            }
            Value::OBJECT(obj) => {
                let val = self.eval_expression(value)?;
                obj.borrow_mut().set(name.clone(), &val);
                Ok(val)
            }
            Value::SUPER { instance, .. } => {
                let val = self.eval_expression(value)?;
                instance.borrow_mut().set(name.clone(), &val);
                Ok(val)
            }
            _ => Err(InterpreterError::InvalidStatement {
                message: "Only objects have properties".to_string(),
            }),
        }
    }
}

