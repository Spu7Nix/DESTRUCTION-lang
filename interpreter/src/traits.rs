use std::collections::HashMap;

use parser::internment::LocalIntern;

use crate::error::RuntimeError;

#[derive(Debug, Clone, PartialEq)] // vec isnt Copy ye ik i just had oto chek chek
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Bool(bool),
}

pub trait Maths {
    fn add(&self, other: &Self) -> Value;
    fn sub(&self, other: &Self) -> Value;
    fn div(&self, other: &Self) -> Value;
    fn mul(&self, other: &Self) -> Value;
}

pub type Variables = HashMap<LocalIntern<String>, Value>;

pub trait Structure {
    fn construct(&self, variables: &Variables) -> Result<Value, RuntimeError>;
    fn destruct(
        &self,
        value: &Value,
        variables: &mut Variables,
    ) -> Result<Option<Value>, RuntimeError>;
}
