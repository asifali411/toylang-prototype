use crate::{errors::interpreter_error::InterpreterError, interpreter::{Interpreter, value::Value}};

type IResult<T> = Result<T, InterpreterError>;

pub fn output(_interp: &mut Interpreter, args: Vec<Value>) -> IResult<Value> {
  for val in args {
    match val {
      Value::INT(n) => print!("{}", n),
      Value::FLOAT(n) => print!("{}", n),

      Value::NULL => print!("null"),

      Value::TRUE => print!("true"),
      Value::FALSE => print!("false"),
      
      Value::STRING(v) => print!("{}", v),

      Value::OBJECT(obj) => print!("<instance of {}>", obj.borrow().class_name()),
      Value::FUNC(func) => print!("<function {}>", func.func_name()),
      Value::CLASS(cls) => print!("<class {}>", cls.name()),

      Value::NativeFunction { name, ..} => print!("<native function {}>", name),
    }
    print!(" ");
  }

  Ok(Value::NULL)
}