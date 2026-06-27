use crate::{errors::interpreter_error::InterpreterError, interpreter::{Interpreter, value::Value}};

type IResult<T> = Result<T, InterpreterError>;

pub fn to_int(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() > 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  match  &args[0] {
    Value::STRING(num) => {
      let n = num.parse::<i64>();
      if n.is_err() {
        return Err(InterpreterError::InvalidStatement { message: format!("Cannot convert {} to integer", num) });
      } else {
        let n = n.unwrap();
        return Ok(Value::INT(n));
      }
    },
    other => {
      return Err(InterpreterError::InvalidParameter { kind: format!("{:?}", other) });
    }
  }
}

pub fn to_float(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() > 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  match  &args[0] {
    Value::STRING(num) => {
      let n = num.parse::<f32>();
      if n.is_err() {
        return Err(InterpreterError::InvalidStatement { message: format!("Cannot convert {} to float", num) });
      } else {
        let n = n.unwrap();
        return Ok(Value::FLOAT(n));
      }
    },
    other => {
      return Err(InterpreterError::InvalidParameter { kind: format!("{:?}", other) });
    }
  }
}