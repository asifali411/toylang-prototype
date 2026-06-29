use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token '{token}' at {line}:{col}")]
    UnexpectedToken { token: String, line: usize, col: usize },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("{message} at {line}:{col}")]
    ExpectedToken { message: String, line: usize, col: usize },

    #[error("Expected name after '{keyword}' keyword at {line}:{col}")]
    ExpectedName { keyword: &'static str, line: usize, col: usize },

    #[error("Invalid statement: {message} at {line}:{col}")]
    InvalidStatement { message: String, line: usize, col: usize },
}

// ── helpers ───────────────────────────────────────────────────────────────────

impl ParseError {
    fn location(&self) -> Option<(usize, usize)> {
        match self {
            Self::UnexpectedToken { line, col, .. }
            | Self::ExpectedToken { line, col, .. }
            | Self::ExpectedName { line, col, .. }
            | Self::InvalidStatement { line, col, .. } => Some((*line, *col)),
            Self::UnexpectedEof => None,
        }
    }

    fn detail(&self) -> String {
        match self {
            Self::UnexpectedToken { token, .. } => {
                format!("Unexpected token '{}'", token.yellow())
            }
            Self::UnexpectedEof => "Unexpected end of input".to_string(),
            Self::ExpectedToken { message, .. } => message.clone(),
            Self::ExpectedName { keyword, .. } => {
                format!("Expected name after '{}' keyword", keyword.yellow())
            }
            Self::InvalidStatement { message, .. } => {
                format!("Invalid statement: {message}")
            }
        }
    }

    pub fn display(&self) {
        let prefix = "Parse error".red().bold();
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