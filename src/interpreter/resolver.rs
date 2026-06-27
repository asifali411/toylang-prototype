use std::collections::HashMap;

use crate::{lexer::token::TokenKind, parser::{expression::Expr, statement::Stmt}};

pub struct Resolver {
  scopes: Vec<HashMap<String, bool>>,
  pub locals: HashMap<*const Expr, usize>,
}

impl Resolver {
  pub fn new() -> Self {
    Self { scopes: Vec::new(), locals: HashMap::new() }
  }

  fn begin_scope(&mut self) { self.scopes.push(HashMap::new()); }
  fn end_scope(&mut self) { self.scopes.pop(); }

  fn declare(&mut self, name: &str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.to_string(), false);
    }
  }

  fn define(&mut self, name: &str) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.to_string(), true);
    }
  }

  fn resolve_local(&mut self, expr: *const Expr, name: &str) {
    for (depth, scope) in self.scopes.iter().rev().enumerate() {
      if scope.contains_key(name) {
        self.locals.insert(expr, depth);
        return;
      }
    }
  }

  pub fn resolve_stmt(&mut self, stmt: &Stmt) {
    match stmt {
      Stmt::Var { name, initializer} => {
        self.declare(name);
        if let Some(init) = initializer {
          self.resolve_expr(init);
        }

        self.define(name);
      }

      Stmt::Block(stmts) => {
        self.begin_scope();
        for s in stmts { self.resolve_stmt(s); }
        self.end_scope();
      }

      Stmt::Func { name, parameters, body } => {
        self.declare(name);
        self.define(name);
        self.begin_scope();
        for param in parameters {
          if let TokenKind::IDENT(n) = &param.kind {
            self.declare(n);
            self.define(n);
          }
        }
        self.resolve_stmt(body);
        self.end_scope();
      }

      Stmt::Class { name, methods } => {
        self.declare(name);
        self.define(name);

        self.begin_scope();
        self.scopes.last_mut().unwrap().insert("this".to_string(), true);
        for method in methods {
          self.resolve_stmt(method);
        }
        self.end_scope();
      }

      Stmt::Return(expr) => self.resolve_expr(expr),

      Stmt::If { condition, if_body, else_body } => {
        self.resolve_expr(condition);
        self.resolve_stmt(if_body);
        if let Some(e) = else_body { self.resolve_stmt(e); }
      }

      Stmt::Loop { count, body } => {
        self.resolve_expr(count);
        self.resolve_stmt(body);
      }

      Stmt::LoopIf { condition, body } => {
        self.resolve_expr(condition);
        self.resolve_stmt(body);
      }

      Stmt::Expr(e) | Stmt::Print(e) => self.resolve_expr(e),
    }
  }

  pub fn resolve_expr(&mut self, expr: &Expr) {
    match expr {
        Expr::Var(token) => {
            if let TokenKind::IDENT(name) = &token.kind {
                if self.scopes.last()
                    .and_then(|s| s.get(name.as_str()))
                    == Some(&false)
                {
                    panic!("Cannot read '{}' in its own initializer", name);
                }
                self.resolve_local(expr as *const Expr, name);
            }
        }
        Expr::Assign { name, value, .. } => {
            self.resolve_expr(value);
            self.resolve_local(expr as *const Expr, name);
        }
        Expr::Binary { left, right, .. } => {
            self.resolve_expr(left);
            self.resolve_expr(right);
        }
        Expr::Unary { right, .. } | Expr::Group { expr: right } => {
            self.resolve_expr(right);
        }
        Expr::Call { callee, arguments, .. } => {
            self.resolve_expr(callee);
            for a in arguments { self.resolve_expr(a); }
        }
        Expr::Get { object, .. } => {
          self.resolve_expr(object);
        }
        Expr::Set { object, value , ..} => {
          self.resolve_expr(object);
          self.resolve_expr(value);
        }
        _ => {}
    }
}

}