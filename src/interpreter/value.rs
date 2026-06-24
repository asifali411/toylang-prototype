use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub, Not};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    INT(i64),
    FLOAT(f32),

    TRUE,
    FALSE,

    NULL,
}

impl Value {
    pub fn as_f32(&self) -> f32 {
        match self {
            Value::INT(n) => *n as f32,
            Value::FLOAT(n) => *n,
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

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a + b),
            (a, b) => Value::FLOAT(a.as_f32() + b.as_f32()),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a - b),
            (a, b) => Value::FLOAT(a.as_f32() - b.as_f32()),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a * b),
            (a, b) => Value::FLOAT(a.as_f32() * b.as_f32()),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Value::FLOAT(self.as_f32() / rhs.as_f32())
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::INT(n) => Value::INT(-n),
            Value::FLOAT(n) => Value::FLOAT(-n),
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
            (Value::INT(a), Value::INT(b)) => a.partial_cmp(b),
            (Value::FLOAT(a), Value::FLOAT(b)) => a.partial_cmp(b),

            (Value::INT(a), Value::FLOAT(b)) => (*a as f32).partial_cmp(b),
            (Value::FLOAT(a), Value::INT(b)) => a.partial_cmp(&(*b as f32)),

            _ => None,
        }
    }
}
