use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult},
    interpreter::{Interpreter, value::Value},
    lexer::token::TokenKind,
    parser::expression::Expr,
};

impl Interpreter {
    pub(crate) fn eval_assign(
        &mut self,
        expr: &Expr,
        name: &String,
        value: &Box<Expr>,
        line: &usize,
        col: &usize,
    ) -> IResult<Value> {
        let value = self.eval_expression(value)?;
        if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
            let mut env = self.environment.borrow_mut();
            if env.assign_at(depth, name, value.clone()).is_none() {
                env.assign_var(name, value.clone(), *line, *col)?;
            }
        } else {
            self.environment
                .borrow_mut()
                .assign_var(name, value.clone(), *line, *col)?;
        }
        Ok(value)
    }

    pub(crate) fn assign_back(&mut self, expr: &Expr, value: Value) -> IResult<()> {
        let (name, line, col) = match expr {
            Expr::Var(tok) => match &tok.kind {
                TokenKind::IDENT(name) => (name.clone(), tok.span.line, tok.span.column),
                _ => {
                    return Err(InterpreterError::InvalidStatement {
                        message: "Increment/decrement target must be a variable".into(),
                    });
                }
            },
            _ => {
                return Err(InterpreterError::InvalidStatement {
                    message: "Increment/decrement target must be a variable".into(),
                });
            }
        };

        if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
            let mut env = self.environment.borrow_mut();
            if env.assign_at(depth, &name, value.clone()).is_none() {
                env.assign_var(&name, value, line, col)?;
            }
        } else {
            self.environment
                .borrow_mut()
                .assign_var(&name, value, line, col)?;
        }

        Ok(())
    }

    pub(crate) fn lookup_var(
        &self,
        name: &str,
        expr: &Expr,
        line: usize,
        col: usize,
    ) -> IResult<Value> {
        if let Some(depth) = self.locals.get(&(expr as *const Expr)).copied() {
            let env = self.environment.borrow();
            if let Some(value) = env.get_at(depth, name) {
                return Ok(value);
            }
            if let Some(value) = env.get_var(name, line, col) {
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

        Err(InterpreterError::UndefinedVariable {
            name: name.into(),
            line,
            col,
        })
    }
}

