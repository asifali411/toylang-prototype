use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Undefined character '{char}' at line {line}, col {col}")]
    UndefinedCharacter { char: char, line: usize, col: usize },

    #[error("Invalid number '{lexeme}' at line {line}, col {col}")]
    InvalidNumber { lexeme: String, line: usize, col: usize },

    #[error("Invalid escape character '\\{char}' at line {line}, col {col}")]
    InvalidEscapeCharacter { char: char, line: usize, col: usize },

    #[error("{message} at line {line}, col {col}")]
    ExpectedCharacter { message: String, line: usize, col: usize },
}

impl LexError {
    fn location(&self) -> (usize, usize) {
        match self {
            Self::UndefinedCharacter { line, col, .. }
            | Self::InvalidNumber { line, col, .. }
            | Self::InvalidEscapeCharacter { line, col, .. }
            | Self::ExpectedCharacter { line, col, .. } => (*line, *col),
        }
    }

    fn detail(&self) -> String {
        match self {
            Self::UndefinedCharacter { char, .. } => {
                format!("Undefined character '{}'", char.to_string().yellow())
            }
            Self::InvalidNumber { lexeme, .. } => {
                format!("Invalid number '{}'", lexeme.yellow())
            }
            Self::InvalidEscapeCharacter { char, .. } => {
                format!("Invalid escape character '\\{}'", char.to_string().yellow())
            }
            Self::ExpectedCharacter { message, .. } => message.clone(),
        }
    }

    pub fn display(&self) {
        let (line, col) = self.location();
        let prefix = "Lex error".red().bold();
        let detail = self.detail();
        let loc = format!(" at line: {line}, col: {col} ").black().on_green();

        eprintln!("{prefix}: {detail}\n{loc}\n");
    }
}