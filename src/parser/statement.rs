use crate::parser::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Box<Stmt>>),
    IF {
        condition: Expr,
        if_body: Box<Stmt>,
        else_body: Option<Box<Stmt>>,
    },
    LOOP {
        count: Expr,
        body: Box<Stmt>,
    },
    LOOP_IF {
        condition: Expr,
        body: Box<Stmt>,
    }
}
