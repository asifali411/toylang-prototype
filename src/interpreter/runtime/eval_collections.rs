use std::collections::HashMap;

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult},
    interpreter::{Interpreter, value::Value},
    lexer::token::TokenKind,
    parser::expression::Expr,
};

impl Interpreter {
    pub(crate) fn eval_array(&mut self, elements: &Vec<Box<Expr>>) -> IResult<Value> {
        let mut items: Vec<Box<Value>> = Vec::new();
        for elem in elements {
            let item = self.eval_expression(elem)?;
            items.push(Box::new(item));
        }

        Ok(Value::ARRAY(items))
    }

    pub(crate) fn eval_hashmap(&mut self, fields: &Vec<(String, Box<Expr>)>) -> IResult<Value> {
        let mut hashmap: HashMap<String, Box<Value>> = HashMap::new();
        for (key, value) in fields {
            let value = self.eval_expression(value)?;
            hashmap.insert(key.clone(), Box::new(value));
        }
        Ok(Value::HASHMAP(hashmap))
    }

    pub(crate) fn eval_index(&mut self, object: &Box<Expr>, index: &Box<Expr>) -> IResult<Value> {
        let index = self.eval_expression(index)?;
        let object = self.eval_expression(object)?;

        match object {
            Value::ARRAY(arr) => match index {
                Value::NUM(n) => {
                    if n.is_finite() && n.fract() == 0.0 {
                        let len_i = arr.len() as i64;
                        let n_i = n as i64;

                        let target_index = if n_i < 0 { len_i + n_i } else { n_i };

                        if target_index >= 0 && target_index < len_i {
                            return Ok((*arr[target_index as usize]).clone());
                        } else {
                            return Err(InterpreterError::InvalidStatement {
                                message: format!(
                                    "Array index {} out of bounds for length {}",
                                    n,
                                    arr.len()
                                ),
                            });
                        }
                    } else {
                        return Err(InterpreterError::InvalidStatement {
                            message: "Array index must be a finite integer".into(),
                        });
                    }
                }
                _ => {
                    return Err(InterpreterError::InvalidStatement {
                        message: "Array index must be a finite integer".into(),
                    });
                }
            },
            _ => {}
        }

        Ok(Value::NULL)
    }

    pub(crate) fn eval_index_set(
        &mut self,
        object: &Expr,
        index: &Expr,
        value: &Expr,
        line: usize,
        col: usize,
    ) -> IResult<Value> {
        let index = self.eval_expression(index)?;
        let value = self.eval_expression(value)?;

        let var_name = match object {
            Expr::Var(tok) => match &tok.kind {
                TokenKind::IDENT(name) => name.clone(),
                _ => {
                    return Err(InterpreterError::InvalidStatement {
                        message: "Invalid assignment target".into(),
                    });
                }
            },
            _ => {
                return Err(InterpreterError::InvalidStatement {
                    message: "Invalid assignment target".into(),
                });
            }
        };

        let mut arr = match self.environment.borrow().get_var(&var_name, line, col) {
            Some(Value::ARRAY(arr)) => arr,
            _ => {
                return Err(InterpreterError::InvalidStatement {
                    message: "Index assignment target must be an array".into(),
                });
            }
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
                        message: format!(
                            "Array index {} out of bounds for length {}",
                            n_i,
                            arr.len()
                        ),
                    });
                }
            }
            _ => {
                return Err(InterpreterError::InvalidStatement {
                    message: "Array index must be a finite integer".into(),
                });
            }
        }

        self.environment
            .borrow_mut()
            .assign_var(&var_name, Value::ARRAY(arr), line, col)?;
        Ok(Value::NULL)
    }
}

