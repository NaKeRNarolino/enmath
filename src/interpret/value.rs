use std::ops::{Add, Div, Mul, Sub};
use crate::interpret::scope::Function;
use crate::parse::ast::AssignmentProp;

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Null,
    Number(f64),
    AssignmentProp(AssignmentProp),
    Function(Function),
}

impl Add for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, other: RuntimeValue) -> RuntimeValue {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => RuntimeValue::Number(a + b),
            _ => RuntimeValue::Null,
        }
    }
}

impl Sub for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, other: RuntimeValue) -> RuntimeValue {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => RuntimeValue::Number(a - b),
            _ => RuntimeValue::Null,
        }
    }
}

impl Mul for RuntimeValue {
    type Output = RuntimeValue;

    fn mul(self, other: RuntimeValue) -> RuntimeValue {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => RuntimeValue::Number(a * b),
            _ => RuntimeValue::Null,
        }
    }
}

impl Div for RuntimeValue {
    type Output = RuntimeValue;

    fn div(self, other: RuntimeValue) -> RuntimeValue {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if b == 0.0 {
                    RuntimeValue::Null
                } else {
                    RuntimeValue::Number(a / b)
                }
            }
            _ => RuntimeValue::Null,
        }
    }
}