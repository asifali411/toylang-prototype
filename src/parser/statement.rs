use crate::parser::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: String, initializer: Option<Expr> },
    Block(Vec<Box<Stmt>>),
}
