use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult},
    interpreter::{Interpreter, value::Value},
    lexer::token::{Token, TokenKind},
    parser::expression::Expr,
};

impl Interpreter {
    pub(crate) fn eval_unary(&mut self, op: &Token, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr)?;
        match op.kind {
            TokenKind::MINUS => Ok((-value)?),
            TokenKind::NOT => Ok((!value)?),
            TokenKind::INC => {
                let new_val = match value {
                    Value::NUM(n) => Value::NUM(n + 1.0),
                    _ => {
                        return Err(InterpreterError::InvalidStatement {
                            message: "Increment operator requires a number".into(),
                        });
                    }
                };
                self.assign_back(expr, new_val.clone())?;
                Ok(new_val)
            }
            TokenKind::DEC => {
                let new_val = match value {
                    Value::NUM(n) => Value::NUM(n - 1.0),
                    _ => {
                        return Err(InterpreterError::InvalidStatement {
                            message: "Decrement operator requires a number".into(),
                        });
                    }
                };
                self.assign_back(expr, new_val.clone())?;
                Ok(new_val)
            }
            ref kind => Err(InterpreterError::UnsupportedUnaryOp { op: kind.clone() }),
        }
    }

    pub(crate) fn eval_post_unary(&mut self, op: &Token, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr)?;

        match op.kind {
            TokenKind::INC => {
                let new_val = match value {
                    Value::NUM(n) => Value::NUM(n + 1.0),
                    _ => {
                        return Err(InterpreterError::InvalidStatement {
                            message: "Increment operator requires a number".into(),
                        });
                    }
                };
                self.assign_back(expr, new_val.clone())?;
                Ok(value)
            }
            TokenKind::DEC => {
                let new_val = match value {
                    Value::NUM(n) => Value::NUM(n - 1.0),
                    _ => {
                        return Err(InterpreterError::InvalidStatement {
                            message: "Decrement operator requires a number".into(),
                        });
                    }
                };
                self.assign_back(expr, new_val.clone())?;
                Ok(value)
            }
            ref kind => Err(InterpreterError::UnsupportedUnaryOp { op: kind.clone() }),
        }
    }

    pub(crate) fn eval_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> IResult<Value> {
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
            TokenKind::MOD => Ok(left.modulo(&right)?),
            TokenKind::LESS => Ok(left.lt(&right)),
            TokenKind::LESS_EQ => Ok(left.lt_eq(&right)),
            TokenKind::GREAT => Ok(left.gt(&right)),
            TokenKind::GREAT_EQ => Ok(left.gt_eq(&right)),
            TokenKind::EQ_EQ => Ok(left.eq(&right)),
            TokenKind::NOT_EQ => Ok(left.not_eq(&right)),
            TokenKind::IN => Ok(self.eval_is_in_expr(&left, &right)?),
            TokenKind::AND => Ok(left.and(&right)?),
            TokenKind::OR => Ok(left.or(&right)?),
            ref kind => return Err(InterpreterError::UnsupportedBinaryOp { op: kind.clone() }),
        };

        Ok(res?)
    }

    pub(crate) fn eval_compound_assign(
        &mut self,
        expr: &Expr,
        name: &String,
        value: &Box<Expr>,
        op: &Token,
    ) -> IResult<Value> {
        let change = self.eval_expression(value)?;

        let current = {
            let env = self.environment.borrow();
            if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
                env.get_at(depth, name)
                    .or_else(|| env.get_var(name, op.span.line, op.span.column))
            } else {
                env.get_var(name, op.span.line, op.span.column)
            }
        }
        .ok_or_else(|| InterpreterError::UndefinedVariable {
            name: name.into(),
            line: op.span.line,
            col: op.span.column,
        })?;

        let new_val = match op.kind {
            TokenKind::PLUS_EQ => (current + change)?,
            TokenKind::MINUS_EQ => (current - change)?,
            TokenKind::STAR_EQ => (current * change)?,
            TokenKind::MOD_EQ => (current.modulo(&change))?,
            TokenKind::SLASH_EQ => {
                if matches!(change, Value::NUM(0.0)) {
                    return Err(InterpreterError::DivisionByZero);
                }
                (current / change)?
            }
            ref kind => return Err(InterpreterError::UnsupportedBinaryOp { op: kind.clone() }),
        };

        if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
            let mut env = self.environment.borrow_mut();
            if env.assign_at(depth, name, new_val.clone()).is_none() {
                env.assign_var(name, new_val.clone(), op.span.line, op.span.column)?;
            }
        } else {
            self.environment.borrow_mut().assign_var(
                name,
                new_val.clone(),
                op.span.line,
                op.span.column,
            )?;
        }

        Ok(new_val)
    }
}

