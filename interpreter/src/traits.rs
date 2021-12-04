use parser::ast::{Value, Pattern};

impl<'a> Expr for Pattern<'a> {
    fn evaluate(&self) -> Value {
        todo!()
    }
}

impl<'a> Maths for Value<'a> {
    fn add(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
            (Value::Number(_), Value::String(_)) => todo!(),
            (Value::Number(_), Value::Array(_)) => todo!(),
            (Value::Number(_), Value::Tuple(_)) => todo!(),
            (Value::String(_), Value::Number(_)) => todo!(),
            (Value::String(_), Value::String(_)) => todo!(),
            (Value::String(_), Value::Array(_)) => todo!(),
            (Value::String(_), Value::Tuple(_)) => todo!(),
            (Value::Array(_), Value::Number(_)) => todo!(),
            (Value::Array(_), Value::String(_)) => todo!(),
            (Value::Array(_), Value::Array(_)) => todo!(),
            (Value::Array(_), Value::Tuple(_)) => todo!(),
            (Value::Tuple(_), Value::Number(_)) => todo!(),
            (Value::Tuple(_), Value::String(_)) => todo!(),
            (Value::Tuple(_), Value::Array(_)) => todo!(),
            (Value::Tuple(_), Value::Tuple(_)) => todo!(),
        }
    }

    fn sub(&self, other: &Self) -> Value {
        todo!()
    }

    fn div(&self, other: &Self) -> Value {
        todo!()
    }

    fn mul(&self, other: &Self) -> Value {
        todo!()
    }
}


pub trait Maths {
    fn add(&self, other: &Self) -> Value;
    fn sub(&self, other: &Self) -> Value;
    fn div(&self, other: &Self) -> Value;
    fn mul(&self, other: &Self) -> Value;
}

pub trait Expr {
    fn evaluate(&self) -> Value;
}
