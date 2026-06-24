use colored::Colorize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Undefined character '{char}'\nat line: {line}, col: {col}")]
    UndefinedCharacter { char: char, line: usize, col: usize },

    #[error("Invalid number '{lexeme}'\nat line: {line}, col: {col}")]
    InvalidNumber {
        lexeme: String,
        line: usize,
        col: usize,
    },

    #[error("{message}\nat line: {line}, col: {col}")]
    ExpectedCharacter { message: String, line: usize, col: usize }
}

impl LexError {
    pub fn display(&self) {
        let prefix = "Lex error".red().bold();
        match self {
            LexError::UndefinedCharacter { char, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!(
                    "{}: Undefined character '{}'\n{}\n",
                    prefix,
                    char.to_string().yellow(),
                    loc,
                );
            }
            LexError::InvalidNumber { lexeme, line, col } => {
                let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();

                eprintln!(
                    "{}: Invalid number '{}'\n{}\n",
                    prefix,
                    lexeme.yellow(),
                    loc,
                );
            }
            LexError::ExpectedCharacter { message, line, col } => {
                 let loc = format!(" at line: {}, col: {} ", line, col)
                    .black()
                    .on_green();
                
                eprintln!("{}\n{}", message, loc);
            }
        }
    }
}
