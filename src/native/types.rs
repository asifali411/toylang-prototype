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

pub fn convert_to_string(val: &Value) -> String {
  match val {
    Value::NUM(n) => format!("{}", n),
    Value::STRING(n) => format!("{}", n),

    Value::NULL => String::from("null"),
    Value::TRUE => String::from("true"),
    Value::FALSE => String::from("false"),

    Value::OBJECT(obj) => format!("<instance of {}>", obj.borrow().class_name()),
    Value::FUNC(func) => format!("<function {}>", func.func_name()),
    Value::CLASS(cls) => format!("<class {}>", cls.name()),

    Value::NativeFunction { name, ..} => format!("<native function {}>", name),
  }
}

pub fn to_string(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() > 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  let val = convert_to_string(&args[0]);

  Ok(Value::STRING(val))
}