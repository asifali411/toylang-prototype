use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub, Not};
use std::rc::Rc;

use crate::errors::interpreter_error::InterpreterError;
use crate::interpreter::Interpreter;
use crate::interpreter::class::class::Class;
use crate::interpreter::class::instance::Instance;
use crate::interpreter::function::Function;

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

    NativeFunction {
        name: String,
        func: NativeFn,
    },

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
            (Value::NativeFunction { name: n1, .. }, Value::NativeFunction { name: n2, .. }) => n1 == n2,
            (Value::FUNC(_), Value::FUNC(_)) => false,
            (Value::CLASS(_), Value::CLASS(_)) => false,
            (Value::OBJECT(a), Value::OBJECT(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Value {
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::NUM(n) => *n as f64,
            _ => panic!("Unexpected behaviour"),
        }
    }

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

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Value::NUM(self.as_f64() * rhs.as_f64())
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        Value::NUM(self.as_f64() - rhs.as_f64())
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        Value::NUM(self.as_f64() + rhs.as_f64())
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Value::NUM(self.as_f64() / rhs.as_f64())
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::NUM(n) => Value::NUM(-n),
            _ => panic!("Unexpected behaviour"),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::TRUE => Value::FALSE,
            Value::FALSE => Value::TRUE,
            _ => panic!("Unexpected behaviour"),
        }
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
