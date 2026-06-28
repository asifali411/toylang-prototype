use std::io::{self, Write};

use crate::{errors::{interpreter_error::InterpreterError, lang_error::IResult}, interpreter::{Interpreter, value::Value}, native::types::convert_to_string};

pub fn output(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  for val in args {
    print!("{}", convert_to_string(&val));
    print!(" ");
  }

  Ok(Value::NULL)
}

pub fn input(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() > 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  match &args[0] {
    Value::STRING(placeholder) => {
      print!("{}", placeholder);
      io::stdout().flush().expect("Failed to flush stdout");
      let mut inp = String::new();

      io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");

      Ok(Value::STRING(inp.trim().to_string()))
    },
    _ => Err(InterpreterError::InvalidParameter { kind: format!("{:?}", args[0]) })
  }
}