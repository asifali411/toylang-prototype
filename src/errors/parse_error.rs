use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token '{token}' at {line}:{col}")]
    UnexpectedToken {
        token: String,
        line: usize,
        col: usize,
    },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("{message} at {line}:{col}")]
    ExpectedToken {
        message: String,
        line: usize,
        col: usize,
    },

    #[error("Expected variable name after var keyword at {line}:{col}")]
    ExpectedVariableName { line: usize, col: usize },

    #[error("Expected function name after func keyword at {line}:{col}")]
    ExpectedFunctionName { line: usize, col: usize },

    #[error("Invalid statement, {message} at {line}:{col}")]
    InvalidStatement {
        message: String,
        line: usize,
        col: usize,
    },
}

impl ParseError {
    pub fn display(&self) {
        let prefix = "Parse error".red().bold();
        match self {
            ParseError::UnexpectedToken { token, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!(
                    "{}: Unexpected token '{}'\n{}\n",
                    prefix,
                    token.yellow(),
                    loc,
                );
            }
            ParseError::UnexpectedEof => {
                eprintln!("{}: unexpected end of input", prefix);
            }
            ParseError::ExpectedToken { message, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!("{}: {}\n{}\n", prefix, message, loc);
            }
            ParseError::ExpectedVariableName { line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!(
                    "{}: Expected variable name after var keyword \n{}\n",
                    prefix, loc
                );
            }
            ParseError::ExpectedFunctionName { line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!(
                    "{}: Expected function name after func keyword \n{}\n",
                    prefix, loc
                );
            }
            ParseError::InvalidStatement { message, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!("{}:\n{}\n{}", prefix, message, loc);
            }
        }
    }
}
