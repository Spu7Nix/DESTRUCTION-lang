use parser::ast::Transformation;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
}

impl Expr for Transformation {
    fn evaluate(&self) -> Value {
        todo!()
    }
}

impl Maths for Value {
    fn add(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
            (Value::String(lhs), Value::String(rhs)) => Value::String(lhs.to_owned() + &rhs),
            (Value::Array(lhs), Value::Array(rhs)) => {
                Value::Array([lhs.to_owned(), rhs.to_owned()].concat())
            }
            _ => todo!(),
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
