use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::IResult},
    interpreter::{Interpreter, value::Value},
    lexer::token::TokenKind,
    parser::expression::Expr,
};

impl Interpreter {
    pub(crate) fn eval_expression(&mut self, expr: &Expr) -> IResult<Value> {
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
                    panic!("Expected variable identifier, but found: {:?}", identifier)
                }
            }
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::PostUnary { operator, left } => self.eval_post_unary(operator, left),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(left, operator, right),
            Expr::Group { expr } => self.eval_expression(expr),
            Expr::Assign {
                name,
                value,
                line,
                col,
            } => self.eval_assign(expr, name, value, line, col),
            Expr::CompoundAssign { name, value, op } => {
                self.eval_compound_assign(expr, name, value, op)
            }
            Expr::Call {
                callee,
                arguments,
                line,
                col,
            } => self.eval_call(callee, arguments, line, col),
            Expr::Get {
                object,
                name,
                line,
                col,
            } => self.eval_get(object, name, line, col),
            Expr::Set {
                object,
                name,
                value,
            } => self.eval_set(object, name, value),
            Expr::Array { elements } => self.eval_array(elements),
            Expr::Hashmap { fields } => self.eval_hashmap(fields),
            Expr::Index { object, index } => self.eval_index(object, index),
            Expr::IndexSet {
                object,
                index,
                value,
                line,
                col,
            } => self.eval_index_set(object, index, value, *line, *col),
        }
    }

    pub(crate) fn eval_call(
        &mut self,
        callee: &Expr,
        arguments: &Vec<Box<Expr>>,
        line: &usize,
        col: &usize,
    ) -> IResult<Value> {
        let callee_value = self.eval_expression(callee)?;

        let args: Vec<Value> = arguments
            .iter()
            .map(|a| self.eval_expression(a))
            .collect::<IResult<_>>()?;

        match callee_value {
            Value::FUNC(func) => Ok(func.call(self, args)?),
            Value::CLASS(class) => Ok(class.call(self, args)?),
            Value::NativeFunction { func, .. } => func(self, args),
            _ => Err(InterpreterError::UndefinedFunction {
                name: format!("{:?}", callee),
                line: *line,
                col: *col,
            }),
        }
    }
}

