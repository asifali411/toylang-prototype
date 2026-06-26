use crate::errors::{
  interpreter_error::InterpreterError, 
  lex_error::LexError, 
  parse_error::ParseError
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LangError {
  #[error(transparent)]
  Lex(#[from] LexError),

  #[error(transparent)]
  Parse(#[from] ParseError),
  
  #[error(transparent)]
  Interpreter(#[from] InterpreterError),
}

impl LangError {
  pub fn display(&self) {
      match self {
          LangError::Lex(e) => e.display(),
          LangError::Parse(e) => e.display(),
          LangError::Interpreter(e) => e.display(),
      }
  }
}