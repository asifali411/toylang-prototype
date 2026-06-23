use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    INT(i64),
    FLOAT(f32),

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
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a + b),
            (a, b) => Value::FLOAT(a.as_f32() + b.as_f32()),
            _ => panic!("Unexpected behaviour"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a - b),
            (a, b) => Value::FLOAT(a.as_f32() - b.as_f32()),
            _ => panic!("Unexpected behaviour"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::INT(a), Value::INT(b)) => Value::INT(a * b),
            (a, b) => Value::FLOAT(a.as_f32() * b.as_f32()),
            _ => panic!("Unexpected behaviour"),
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
