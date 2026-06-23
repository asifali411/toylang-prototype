use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Token),
}
