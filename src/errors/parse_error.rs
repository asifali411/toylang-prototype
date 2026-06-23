use thiserror::Error;
use colored::Colorize;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token '{token}' at {line}:{col}")]
    UnexpectedToken { token: String, line: usize, col: usize },

    #[error("Unexpected end of input")]
    UnexpectedEof,
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
        }
    }
}
