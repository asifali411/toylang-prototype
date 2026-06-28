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
  if args.len() != 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }
  
  Ok(Value::STRING(convert_to_string(&args[0])))
}

pub fn extract_type(val: &Value) -> String {
  match val {
    Value::NUM(_) => String::from("number"),
    Value::STRING(_) => String::from("string"),
    Value::NULL => String::from("null"),
    Value::TRUE => String::from("boolean"),
    Value::FALSE => String::from("boolean"),
    Value::OBJECT(_) => String::from("object"),
    Value::FUNC(_) => String::from("function"),
    Value::CLASS(_) => String::from("class"),
    Value::NativeFunction { .. } => String::from("function"),
  }
}

pub fn type_of(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  if args.len() != 1 {
    return Err(InterpreterError::ArityMismatch { expected: 1, got: args.len() });
  }

  Ok(Value::STRING(extract_type(&args[0])))
}