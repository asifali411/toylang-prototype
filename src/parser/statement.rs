use crate::{lexer::token::Token, parser::expression::Expr};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Box<Stmt>>),
    If {
        condition: Expr,
        if_body: Box<Stmt>, // expect Block
        else_body: Option<Box<Stmt>>, // expect Block
    },
    Loop {
        count: Expr,
        body: Box<Stmt>, // expect Block
    },
    LoopIf {
        condition: Expr,
        body: Box<Stmt>, // expect Block
    },
    Func {
        name: String,
        parameters: Vec<Token>,
        body: Rc<Stmt>, // expect Block
    },
    Return(Expr),
    Class {
        name: String,
        methods: Vec<Stmt>,
        superclass: Option<Expr>, // expect Var
    }
}
