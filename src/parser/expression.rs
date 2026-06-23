use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}
