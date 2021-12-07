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
                value = construct.construct(&env)?;
            }
        }
    }
    Ok(value)
}

impl Structure for Expr {
    fn construct(
        &self,
        variables: &HashMap<LocalIntern<String>, Value>,
    ) -> Result<Value, RuntimeError> {
        match self {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s, _) => Ok(Value::String(s.to_owned())), // btw we can make strings localintern
            Expr::Array(arr) => Ok(Value::Array(
                arr.iter()
                    .map(|e| -> Result<_, _> { e.construct(variables) })
                    .collect::<Result<_, _>>()?,
            )),
            Expr::Tuple(t) => Ok(Value::Tuple(
                t.iter()
                    .map(|e| -> Result<_, _> { e.construct(variables) })
                    .collect::<Result<_, _>>()?,
            )),
            Expr::Ident(i) => variables.get(i).cloned().ok_or(RuntimeError::ValueErrorT),
            Expr::Operator(op, a, b) => {
                use parser::ast::Operator::*;
                match op {
                    Add => Ok(a.construct(variables)?.add(&b.construct(variables)?)),
                    Sub => Ok(a.construct(variables)?.sub(&b.construct(variables)?)),
                    Mul => Ok(a.construct(variables)?.mul(&b.construct(variables)?)),
                    Div => Ok(a.construct(variables)?.div(&b.construct(variables)?)),
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
                    Err(RuntimeError::PatternMismatchT)
                }
            }
            Expr::String(s, _) => {
                if let Value::String(s2) = value {
                    if s == s2 {
                        Ok(Some(Value::String(s.to_owned())))
                    } else {
                        Err(RuntimeError::PatternMismatchT)
                    }
                } else {
                    Err(RuntimeError::PatternMismatchT)
                }
            }
            Expr::Array(arr) => {
                // i fugured out the destruct thing!!
                if let Value::Array(arr2) = value {
                    if arr.len() != arr2.len() {
                        return Err(RuntimeError::PatternMismatchT);
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
                    Err(RuntimeError::PatternMismatchT)
                }
            }
            Expr::Tuple(t) => {
                if let Value::Array(t2) = value {
                    if t.len() != t2.len() {
                        return Err(RuntimeError::PatternMismatchT);
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
                    Err(RuntimeError::PatternMismatchT)
                }
            }
            Expr::Ident(i) => {
                variables.insert(*i, value.clone());
                Ok(None)
            }
            Expr::Operator(op, left, right) => {
                use parser::ast::Operator::*;
                match (
                    left.construct(&HashMap::new()),
                    right.construct(&HashMap::new()),
                ) {
                    (Ok(a), Ok(b)) => {
                        // incase some patterns both destruct and give a value (like @ in rust)
                        left.destruct(&a, variables)?;
                        right.destruct(&b, variables)?;

                        let res = match op {
                            Add => a.add(&b),
                            Sub => a.sub(&b),
                            Mul => a.mul(&b),
                            Div => a.div(&b),
                        };
                        if &res == value {
                            Ok(Some(res))
                        } else {
                            Err(RuntimeError::PatternMismatchT)
                        }
                    }

                    (Ok(left), Err(_)) => {
                        match op {
                            Add => destruct_algebra::add_left_destruct(
                                &left, &*right, value, variables,
                            )?,
                            Sub => destruct_algebra::sub_left_destruct(
                                &left, &*right, value, variables,
                            )?,
                            Mul => destruct_algebra::mul_left_destruct(
                                &left, &*right, value, variables,
                            )?,
                            Div => destruct_algebra::div_left_destruct(
                                &left, &*right, value, variables,
                            )?,
                        };
                        Ok(None)
                    }
                    (Err(_), Ok(right)) => {
                        match op {
                            Add => destruct_algebra::add_right_destruct(
                                &right, &*left, value, variables,
                            )?,
                            Sub => destruct_algebra::sub_right_destruct(
                                &right, &*left, value, variables,
                            )?,
                            Mul => destruct_algebra::mul_right_destruct(
                                &right, &*left, value, variables,
                            )?,
                            Div => destruct_algebra::div_right_destruct(
                                &right, &*left, value, variables,
                            )?,
                        };
                        Ok(None)
                    }

                    _ => Err(RuntimeError::ValueErrorT),
                }
            }
        }
    }
}
