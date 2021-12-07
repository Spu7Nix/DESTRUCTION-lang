use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::traits::{Maths, Structure, Value, Variables};
use parser::ast::Transformation::Forced;
use parser::ast::{Expr, TopLevel};
use parser::internment::LocalIntern;

use crate::destruct_algebra;

impl Maths for Value {
    fn add(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
            (Value::String(lhs), Value::String(rhs)) => Value::String(lhs.to_owned() + rhs),
            (Value::Array(lhs), Value::Array(rhs)) => {
                Value::Array([lhs.to_owned(), rhs.to_owned()].concat())
            }
            _ => todo!(),
        }
    }

    fn sub(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
            _ => todo!(),
        }
    }

    fn div(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs / rhs),
            _ => todo!(),
        }
    }

    fn mul(&self, other: &Self) -> Value {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs * rhs),
            (Value::Array(lhs), Value::Number(rhs)) => {
                if rhs.fract() != 0.0 {
                    // RuntimeError::ValueError
                }
                let mut out = Vec::new();
                for _ in 0..(*rhs as usize) {
                    out.extend(lhs.iter().cloned());
                }
                Value::Array(out)
            }
            (Value::String(s), Value::Number(n)) => {
                if n.fract() != 0.0 {
                    // RuntimeError::ValueError
                }
                let mut out = String::new();

                for _ in 0..(*n as usize) {
                    out.push_str(s);
                }

                Value::String(out)
            }
            _ => todo!(),
        }
    }
}

pub fn interpret(top_level: TopLevel, input: Value) -> Result<Value, RuntimeError> {
    let mut value = input;
    for trans in top_level.transformations {
        let mut env = HashMap::new();
        match trans {
            Forced {
                destruct,
                construct,
            } => {
                destruct.destruct(&value, &mut env).unwrap();
                value = construct.construct(&env);
            }
        }
    }
    Ok(value)
}

impl Structure for Expr {
    fn construct(&self, variables: &HashMap<LocalIntern<String>, Value>) -> Value {
        match self {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s, _) => Value::String(s.to_owned()), // btw we can make strings localintern
            Expr::Array(arr) => Value::Array(arr.iter().map(|e| e.construct(variables)).collect()),
            Expr::Tuple(t) => Value::Tuple(t.iter().map(|e| e.construct(variables)).collect()),
            Expr::Ident(i) => variables.get(i).unwrap().clone(),
            Expr::Operator(op, a, b) => {
                use parser::ast::Operator::*;
                match op {
                    Add => a.construct(variables).add(&b.construct(variables)),
                    Sub => a.construct(variables).sub(&b.construct(variables)),
                    Mul => a.construct(variables).mul(&b.construct(variables)),
                    Div => a.construct(variables).div(&b.construct(variables)),
                }
            }
        }
    }

    fn destruct(
        &self,
        value: &Value,
        variables: &mut HashMap<LocalIntern<String>, Value>,
    ) -> Result<Option<Value>, RuntimeError> {
        match &self {
            Expr::Number(n) => {
                if value == &Value::Number(*n) {
                    Ok(Some(Value::Number(*n)))
                } else {
                    Err(RuntimeError::PatternMismatch)
                }
            }
            Expr::String(s, _) => {
                if let Value::String(s2) = value {
                    if s == s2 {
                        Ok(Some(Value::String(s.to_owned())))
                    } else {
                        Err(RuntimeError::PatternMismatch)
                    }
                } else {
                    Err(RuntimeError::PatternMismatch)
                }
            }
            Expr::Array(arr) => {
                if let Value::Array(arr2) = value {
                    if arr.len() != arr2.len() {
                        return Err(RuntimeError::PatternMismatch);
                    }
                    let mut arr_val = Some(Vec::new());

                    for (e, v) in arr.iter().zip(arr2.iter()) {
                        if let (Some(val), Some(arr_val)) =
                            (e.destruct(v, variables)?, &mut arr_val)
                        {
                            arr_val.push(val);
                        } else {
                            arr_val = None;
                        }
                    }

                    Ok(arr_val.map(Value::Array))
                } else {
                    Err(RuntimeError::PatternMismatch)
                }
            }
            Expr::Tuple(t) => {
                if let Value::Array(t2) = value {
                    if t.len() != t2.len() {
                        return Err(RuntimeError::PatternMismatch);
                    }
                    let mut arr_val = Some(Vec::new());

                    for (e, v) in t.iter().zip(t2.iter()) {
                        if let (Some(val), Some(arr_val)) =
                            (e.destruct(v, variables)?, &mut arr_val)
                        {
                            arr_val.push(val);
                        } else {
                            arr_val = None;
                        }
                    }

                    Ok(arr_val.map(Value::Tuple))
                } else {
                    Err(RuntimeError::PatternMismatch)
                }
            }
            Expr::Ident(i) => {
                variables.insert(*i, value.clone());
                Ok(None)
            }
            Expr::Operator(op, left, right) => {
                use parser::ast::Operator::*;
                let mut temp_vars = HashMap::new();
                match (
                    left.destruct(value, &mut temp_vars)?,
                    right.destruct(value, &mut temp_vars)?,
                ) {
                    (Some(a), Some(b)) => {
                        // incase some patterns both destruct and give a value (like @ in rust)
                        left.destruct(value, variables)?;
                        right.destruct(value, variables)?;

                        let res = match op {
                            Add => a.add(&b),
                            Sub => a.sub(&b),
                            Mul => a.mul(&b),
                            Div => a.div(&b),
                        };
                        if &res == value {
                            Ok(Some(res))
                        } else {
                            Err(RuntimeError::PatternMismatch)
                        }
                    }

                    (Some(left), None) => {
                        match op {
                            Add => destruct_algebra::add_left_destruct(&left, &*right, variables)?,
                            Sub => destruct_algebra::sub_left_destruct(&left, &*right, variables)?,
                            Mul => destruct_algebra::mul_left_destruct(&left, &*right, variables)?,
                            Div => destruct_algebra::div_left_destruct(&left, &*right, variables)?,
                        };
                        Ok(None)
                    }
                    (None, Some(right)) => {
                        match op {
                            Add => destruct_algebra::add_right_destruct(&right, &*left, variables)?,
                            Sub => destruct_algebra::sub_right_destruct(&right, &*left, variables)?,
                            Mul => destruct_algebra::mul_right_destruct(&right, &*left, variables)?,
                            Div => destruct_algebra::div_right_destruct(&right, &*left, variables)?,
                        };
                        Ok(None)
                    }

                    _ => Err(RuntimeError::ValueError),
                }
            }
        }
    }
}
