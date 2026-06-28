use crate::{lexer::token::Token, parser::expression::Expr};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Box<Stmt>>),
    If {
        condition: Expr,
        if_body: Box<Stmt>,
        else_body: Option<Box<Stmt>>,
    },
    Loop {
        count: Expr,
        body: Box<Stmt>,
    },
    LoopIf {
        condition: Expr,
        body: Box<Stmt>,
    },
    Func {
        name: String,
        parameters: Vec<Token>,
        body: Rc<Stmt>,
    },
    Return(Expr),
    Class {
        name: String,
        methods: Vec<Stmt>,
    }
}
