use crate::lexer::token::TokenKind;
use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Unsupported unary operator '{op:?}'")]
    UnsupportedUnaryOp { op: TokenKind },

    #[error("Unsupported binary operator '{op:?}'")]
    UnsupportedBinaryOp { op: TokenKind },

    #[error("Unexpected literal kind '{kind:?}'")]
    UnexpectedLiteral { kind: TokenKind },

    #[error("Unexpected expression")]
    UnexpectedExpr,

    #[error("Undefined variable '{var}'")]
    UndefinedVariable {
        var: String,
        line: usize,
        col: usize,
    },

    #[error("Undefined func '{func}' at {line}:{col}")]
    UndefinedFunction {
        func: String,
        line: usize,
        col: usize,
    },

    #[error("Undefined property '{prop}' at {line}:{col}")]
    UndefinedProperty {
        prop: String,
        line: usize,
        col: usize,
    },

    #[error("Expected {expected} arguments but recieved {got} arguments")]
    ArityMismatch {
        expected: usize,
        got: usize,
    },

    #[error("Cannot divide by zero")]
    DivisionByZero,

    #[error("Invalid parameter '{:?}'", kind)]
    InvalidParameter { kind: String },

    #[error("Invalid statement {message}")]
    InvalidStatement { message: String }
}

impl InterpreterError {
    pub fn display(&self) {
        let prefix = "Runtime error".red().bold();
        match self {
            InterpreterError::UnsupportedUnaryOp { op } => {
                eprintln!("{}: unsupported unary operator '{:?}'", prefix, op);
            }
            InterpreterError::UnsupportedBinaryOp { op } => {
                eprintln!("{}: unsupported binary operator '{:?}'", prefix, op);
            }
            InterpreterError::UnexpectedLiteral { kind } => {
                eprintln!("{}: unexpected literal kind '{:?}'", prefix, kind);
            }
            InterpreterError::UnexpectedExpr => {
                eprintln!("{}: unexpected expression", prefix);
            }
            InterpreterError::UndefinedVariable { var, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!("{}: Undefined variable '{}'\n{}\n", prefix, var, loc);
            }
            InterpreterError::UndefinedFunction { func, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!("{}: Undefined function '{}'\n{}\n", prefix, func, loc);
            }
            InterpreterError::UndefinedProperty { prop, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!("{}: Undefined property '{}'\n{}\n", prefix, prop, loc);
            }
            InterpreterError::ArityMismatch { expected, got } => {
                eprintln!("Expected {expected} arguments but recieved {got} arguments");
            },
            InterpreterError::DivisionByZero => {
                eprintln!("{}: Cannot divide by zero", prefix);
            }
            InterpreterError::InvalidParameter { kind} => {
                eprintln!("Invalid parameter: '{:?}'", kind.yellow());
            }
            InterpreterError::InvalidStatement { message } => {
                eprintln!("Invalid statement: {message}");
            }
        }
    }
}
