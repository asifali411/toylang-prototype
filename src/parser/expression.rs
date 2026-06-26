use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Token),
    Var(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Group {
        expr: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
        line: usize,
        col: usize,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Box<Expr>>,
        line: usize,
        col: usize,
    }
}
