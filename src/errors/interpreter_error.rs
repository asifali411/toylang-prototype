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
        }
    }
}
