use crate::lexer::token::TokenKind;
use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    // --- Operator / expression errors ---
    #[error("Unsupported unary operator '{op:?}'")]
    UnsupportedUnaryOp { op: TokenKind },

    #[error("Unsupported binary operator '{op:?}'")]
    UnsupportedBinaryOp { op: TokenKind },

    #[error("Unexpected literal kind '{kind:?}'")]
    UnexpectedLiteral { kind: TokenKind },

    #[error("Unexpected expression")]
    UnexpectedExpr,

    // --- Type errors ---
    // #[error("Type mismatch at {line}:{col}: expected {expected}, got {got}")]
    // TypeMismatch {
    //     expected: &'static str,
    //     got: &'static str,
    //     line: usize,
    //     col: usize,
    // },

    // #[error("Invalid operand type '{got}' for operator '{op:?}' at {line}:{col}")]
    // InvalidOperandType {
    //     op: TokenKind,
    //     got: &'static str,
    //     line: usize,
    //     col: usize,
    // },

    // --- Resolution errors ---
    #[error("Undefined variable '{name}' at {line}:{col}")]
    UndefinedVariable {
        name: String,
        line: usize,
        col: usize,
    },

    #[error("Undefined function '{name}' at {line}:{col}")]
    UndefinedFunction {
        name: String,
        line: usize,
        col: usize,
    },

    #[error("Undefined property '{name}' at {line}:{col}")]
    UndefinedProperty {
        name: String,
        line: usize,
        col: usize,
    },

    // --- Call errors ---
    #[error("Arity mismatch: expected {expected} argument(s) but received {got}")]
    ArityMismatch { expected: usize, got: usize },

    #[error("Invalid parameter '{name}'")]
    InvalidParameter { name: String },

    // --- Arithmetic ---
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Arithmetic error: {message}")]
    ArithmeticError { message: String },

    // --- Control flow ---
    #[error("Invalid statement: {message}")]
    InvalidStatement { message: String },
    // #[error("Stack overflow: max call depth exceeded")]
    // StackOverflow,
}

// ── helpers ──────────────────────────────────────────────────────────────────

impl InterpreterError {
    fn location(&self) -> Option<(usize, usize)> {
        match self {
            Self::UndefinedVariable { line, col, .. }
            | Self::UndefinedFunction { line, col, .. }
            | Self::UndefinedProperty { line, col, .. }
            // | Self::TypeMismatch { line, col, .. }
            // | Self::InvalidOperandType { line, col, .. } 
            => Some((*line, *col)),
            _ => None,
        }
    }

    fn detail(&self) -> String {
        match self {
            Self::UnsupportedUnaryOp { op } => {
                format!(
                    "Unsupported unary operator '{}'",
                    format!("{op:?}").yellow()
                )
            }
            Self::UnsupportedBinaryOp { op } => {
                format!(
                    "Unsupported binary operator '{}'",
                    format!("{op:?}").yellow()
                )
            }
            Self::UnexpectedLiteral { kind } => {
                format!("Unexpected literal kind '{}'", format!("{kind:?}").yellow())
            }
            Self::UnexpectedExpr => "Unexpected expression".to_string(),
            // Self::TypeMismatch { expected, got, .. } => format!(
            //     "Type mismatch: expected '{}', got '{}'",
            //     expected.yellow(),
            //     got.yellow()
            // ),
            // Self::InvalidOperandType { op, got, .. } => format!(
            //     "Invalid operand type '{}' for operator '{}'",
            //     got.yellow(),
            //     format!("{op:?}").yellow()
            // ),
            Self::UndefinedVariable { name, .. } => {
                format!("Undefined variable '{}'", name.yellow())
            }
            Self::UndefinedFunction { name, .. } => {
                format!("Undefined function '{}'", name.yellow())
            }
            Self::UndefinedProperty { name, .. } => {
                format!("Undefined property '{}'", name.yellow())
            }
            Self::ArityMismatch { expected, got } => format!(
                "Expected {} argument(s) but received {}",
                expected.to_string().yellow(),
                got.to_string().yellow()
            ),
            Self::InvalidParameter { name } => {
                format!("Invalid parameter '{}'", name.yellow())
            }
            Self::DivisionByZero => "Division by zero".to_string(),
            Self::ArithmeticError { message } => format!("Arithmetic error: {message}"),
            Self::InvalidStatement { message } => format!("Invalid statement: {message}"),
            // Self::StackOverflow => "Stack overflow: max call depth exceeded".to_string(),
        }
    }

    pub fn display(&self) {
        let prefix = "Runtime error".red().bold();
        let detail = self.detail();

        match self.location() {
            Some((line, col)) => {
                let loc = format!(" at line: {line}, col: {col} ").black().on_green();
                eprintln!("{prefix}: {detail}\n{loc}\n");
            }
            None => eprintln!("{prefix}: {detail}\n"),
        }
    }
}

