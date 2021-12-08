use std::collections::HashMap;

use parser::{ast::Transformation, internment::LocalIntern};

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
}

#[derive(Debug)]
pub struct Variables {
    idents: HashMap<LocalIntern<String>, Value>,
    pub polyidents: HashMap<LocalIntern<String>, Vec<Value>>,
}

pub type Functions = HashMap<LocalIntern<String>, Vec<Transformation>>;

pub trait Structure {
    fn construct(
        &self,
        variables: &mut Variables,
        functions: &Functions,
    ) -> Result<Value, RuntimeError>;
    fn destruct(
        &self,
        value: &Value,
        variables: &mut Variables,
        functions: &Functions,
    ) -> Result<Option<Value>, RuntimeError>;
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            idents: HashMap::new(),
            polyidents: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: LocalIntern<String>, value: Value) -> Result<(), RuntimeError> {
        if let Some(a) = self.idents.get(&key) {
            if a != &value {
                Err(RuntimeError::ValueError(format!(
                    "Variable {} already has a value different from {:?}",
                    key, value
                )))
            } else {
                Ok(())
            }
        } else {
            self.idents.insert(key, value);
            Ok(())
        }
    }
    pub fn get(&self, i: LocalIntern<String>) -> Option<&Value> {
        self.idents.get(&i)
    }

    pub fn insert_polyident(
        &mut self,
        key: LocalIntern<String>,
        value: Value,
    ) -> Result<(), RuntimeError> {
        if let Some(a) = self.polyidents.get_mut(&key) {
            a.push(value);
            Ok(())
        } else {
            self.polyidents.insert(key, vec![value]);
            Ok(())
        }
    }

    pub fn take_polyident(
        &mut self,
        i: LocalIntern<String>,
    ) -> Result<Option<Value>, RuntimeError> {
        if let Some(v) = self.polyidents.get_mut(&i) {
            if v.is_empty() {
                Err(RuntimeError::ValueError(format!(
                    "Polyident {} is used up",
                    i
                )))
            } else {
                Ok(Some(v.remove(0)))
            }
        } else {
            Ok(None)
        }
    }
}

impl Default for Variables {
    fn default() -> Self {
        Self::new()
    }
}
