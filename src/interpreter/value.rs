use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::rc::Rc;

use crate::errors::interpreter_error::InterpreterError;
use crate::errors::lang_error::IResult;
use crate::interpreter::Interpreter;
use crate::interpreter::class::class::Class;
use crate::interpreter::class::instance::Instance;
use crate::interpreter::function::Function;
use crate::native::types::{convert_to_string, extract_type};

pub type NativeFn = fn(&mut Interpreter, Vec<Value>) -> Result<Value, InterpreterError>;

#[derive(Debug, Clone)]
pub enum Value {
    NUM(f64),
    STRING(String),

    TRUE,
    FALSE,

    FUNC(Function),
    CLASS(Class),
    OBJECT(Rc<RefCell<Instance>>),
    ARRAY(Vec<Box<Value>>),
    HASHMAP(HashMap<String, Box<Value>>),

    NativeFunction { name: String, func: NativeFn },

    NULL,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::NUM(a), Value::NUM(b)) => a == b,
            (Value::STRING(a), Value::STRING(b)) => a == b,
            (Value::TRUE, Value::TRUE) => true,
            (Value::FALSE, Value::FALSE) => true,
            (Value::NULL, Value::NULL) => true,
            (Value::NativeFunction { name: n1, .. }, Value::NativeFunction { name: n2, .. }) => {
                n1 == n2
            }
            (Value::FUNC(_), Value::FUNC(_)) => false,
            (Value::CLASS(_), Value::CLASS(_)) => false,
            (Value::OBJECT(a), Value::OBJECT(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Value {
    pub fn lt(&self, value: &Value) -> Value {
        if self < value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn gt(&self, value: &Value) -> Value {
        if self > value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn lt_eq(&self, value: &Value) -> Value {
        if self <= value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn gt_eq(&self, value: &Value) -> Value {
        if self >= value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn eq(&self, value: &Value) -> Value {
        if self == value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn not_eq(&self, value: &Value) -> Value {
        if self != value {
            Value::TRUE
        } else {
            Value::FALSE
        }
    }

    pub fn is_true(&self) -> bool {
        self == &Value::TRUE
    }
}

impl Add for Value {
    type Output = IResult<Value>;

    fn add(self, rhs: Value) -> IResult<Value> {
        let res = match (self, rhs) {
            (Value::NUM(a), Value::NUM(b)) => Value::NUM(a + b),
            (Value::STRING(a), Value::STRING(b)) => Value::STRING(format!("{}{}", a, b)),
            (a, b) => {
                let a_type = extract_type(&a);
                let b_type = extract_type(&b);
                let a = convert_to_string(&a);
                let b = convert_to_string(&b);

                let message = format!(
                    "Cannot add <{}>({}) and <{}>({}) together",
                    a_type, a, b_type, b
                );
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl Sub for Value {
    type Output = IResult<Value>;

    fn sub(self, rhs: Value) -> IResult<Value> {
        let res = match (self, rhs) {
            (Value::NUM(a), Value::NUM(b)) => Value::NUM(a - b),
            (a, b) => {
                let a_type = extract_type(&a);
                let b_type = extract_type(&b);
                let a = convert_to_string(&a);
                let b = convert_to_string(&b);

                let message = format!(
                    "Cannot substract <{}>({}) and <{}>({}) together",
                    a_type, a, b_type, b
                );
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl Mul for Value {
    type Output = IResult<Value>;

    fn mul(self, rhs: Value) -> IResult<Value> {
        let res = match (self, rhs) {
            (Value::NUM(a), Value::NUM(b)) => Value::NUM(a * b),
            (a, b) => {
                let a_type = extract_type(&a);
                let b_type = extract_type(&b);
                let a = convert_to_string(&a);
                let b = convert_to_string(&b);

                let message = format!(
                    "Cannot multiply <{}>({}) and <{}>({}) together",
                    a_type, a, b_type, b
                );
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl Div for Value {
    type Output = IResult<Value>;

    fn div(self, rhs: Value) -> IResult<Value> {
        let res = match (self, rhs) {
            (Value::NUM(a), Value::NUM(b)) => Value::NUM(a / b),
            (a, b) => {
                let a_type = extract_type(&a);
                let b_type = extract_type(&b);
                let a = convert_to_string(&a);
                let b = convert_to_string(&b);

                let message = format!(
                    "Cannot divide <{}>({}) and <{}>({}) together",
                    a_type, a, b_type, b
                );
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl Neg for Value {
    type Output = IResult<Value>;

    fn neg(self) -> Self::Output {
        let res = match self {
            Value::NUM(n) => Value::NUM(-n),
            other => {
                let other_type = extract_type(&other);
                let other = convert_to_string(&other);

                let message = format!("Cannot negate <{}>({})", other_type, other);
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl Not for Value {
    type Output = IResult<Value>;

    fn not(self) -> Self::Output {
        let res = match self {
            Value::TRUE => Value::FALSE,
            Value::FALSE => Value::TRUE,
            other => {
                let other_type = extract_type(&other);
                let other = convert_to_string(&other);

                let message = format!("Cannot negate <{}>({})", other_type, other);
                return Err(InterpreterError::ArithmeticError { message });
            }
        };
        Ok(res)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::NUM(a), Value::NUM(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
