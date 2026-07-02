use crate::{errors::interpreter_error::InterpreterError, interpreter::value::Value};

#[derive(Debug)]
pub enum Signal {
    Return(Value),
    Error(InterpreterError),
    Break,
    Continue,
}

impl From<InterpreterError> for Signal {
    fn from(e: InterpreterError) -> Self {
        Signal::Error(e)
    }
}

