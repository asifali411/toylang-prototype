use crate::{
    errors::interpreter_error::InterpreterError,
    interpreter::value::Value,
    lexer::token::{Token, TokenKind},
    parser::{expression::Expr, statement::Stmt},
};

type IResult<T> = Result<T, InterpreterError>;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, statement: &Stmt) -> IResult<Value> {
        match statement {
            Stmt::Expr(expr) => self.eval_expression(expr),
        }
    }

    pub fn eval_expression(&mut self, expr: &Expr) -> IResult<Value> {
        match expr {
            Expr::Literal(literal) => match literal.kind {
                TokenKind::INT(n) => Ok(Value::INT(n)),
                TokenKind::FLOAT(n) => Ok(Value::FLOAT(n)),
                ref kind => Err(InterpreterError::UnexpectedLiteral { kind: kind.clone() }),
            },
            Expr::Unary { operator, right } => self.eval_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(left, operator, right),
            Expr::Group { expr } => self.eval_expression(expr),
            _ => Err(InterpreterError::UnexpectedExpr),
        }
    }

    //-----------------------------------------------------------------------------

    fn eval_unary(&mut self, op: &Token, expr: &Expr) -> IResult<Value> {
        let value = self.eval_expression(expr)?;
        match op.kind {
            TokenKind::MINUS => Ok(-value),
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
