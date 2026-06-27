use crate::{errors::interpreter_error::InterpreterError, interpreter::{Interpreter, value::Value}};

type IResult<T> = Result<T, InterpreterError>;

pub fn to_num(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() > 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  match  &args[0] {
    Value::STRING(num) => {
      let n = num.parse::<f64>();
      if n.is_err() {
        return Err(InterpreterError::InvalidStatement { message: format!("Cannot convert {} to number", num) });
      } else {
        let n = n.unwrap();
        return Ok(Value::NUM(n));
      }
    },
    other => {
      return Err(InterpreterError::InvalidParameter { kind: format!("{:?}", other) });
    }
  }
}