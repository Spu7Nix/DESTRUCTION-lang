use parser::ast::{Type, UnaryOperator};
use parser::internment::LocalIntern;

use crate::error::RuntimeError;
use crate::traits::{Maths, Structure, Value, Variables};
use parser::ast::Transformation::Forced;
use parser::ast::{Expr, TopLevel};

use crate::destruct_algebra;

impl Value {
    fn to_type(&self) -> &Type {
        match self {
            Value::String(_) => &Type::String,
            Value::Number(_) => &Type::Number,
            Value::Tuple(_) => &Type::Tuple,
            Value::Array(_) => &Type::Array,
            Value::Bool(_) => &Type::Bool,
        }
    }
    fn cast(&self, to: &Type, from: &Type) -> Result<Value, RuntimeError> {
        if from != self.to_type() {
            return Err(RuntimeError::TypeMismatch(
                from.to_string(),
                self.to_type().to_string(),
            ));
        }
        if to == self.to_type() {
            return Ok(self.clone());
        }
        match (to, self) {
            (Type::Number, Value::String(s)) => {
                Ok(Self::Number(s.parse::<f64>().ok().unwrap_or(f64::NAN)))
            }
            (Type::Number, Value::Array(_) | Value::Tuple(_)) => Err(RuntimeError::ValueError(
                "Cannot convert array or tuple to number".to_string(),
            )),
            (Type::Array | Type::Tuple, Value::Number(_)) => Err(RuntimeError::ValueError(
                "Cannot convert number to array or tuple".to_string(),
            )),
            (Type::String, v) => Ok(Self::String(format!("{:?}", v))), //TODO: something better than just debug
            (Type::Array, Value::String(s)) => Ok(Self::Array(
                s.chars().map(|x| Value::String(String::from(x))).collect(),
            )),
            (Type::Tuple, Value::String(s)) => Ok(Self::Tuple(
                s.chars().map(|x| Value::String(String::from(x))).collect(),
            )),
            // boolean casting?
            _ => unreachable!(),
        }
    }
}

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
}

fn mul(left: &Expr, right: &Value, variables: &mut Variables) -> Result<Value, RuntimeError> {
    let factor = *match right {
        Value::Number(n) => n,
        _ => Err(RuntimeError::TypeMismatch(
            "number".to_string(),
            right.to_type().to_string(),
        ))?,
    };
    let mut out = left.construct(variables)?;
    if let Value::Number(n) = out {
        return Ok(Value::Number(n * factor));
    }
    if factor.fract() != 0.0 {
        return Err(RuntimeError::ValueError(
            "Can only multiply numbers by fractional number".to_string(),
        ));
    }
    let n = factor as usize;
    for _ in 0..(n - 1) {
        out = out.add(&left.construct(variables)?);
    }
    Ok(out)
}

pub fn interpret(top_level: TopLevel, input: Value) -> Result<Value, RuntimeError> {
    let mut value = input;
    let func = top_level
        .functions
        .get(&LocalIntern::new("main".to_string()))
        .ok_or_else(|| RuntimeError::ValueError("Missing `main` function".to_string()))?;
    run_func(func, &mut value)?;
    Ok(value)
}

fn run_func(func: &[parser::ast::Transformation], value: &mut Value) -> Result<(), RuntimeError> {
    for trans in func {
        let mut env = Variables::new();
        match trans {
            Forced {
                destruct,
                construct,
            } => {
                destruct.destruct(&*value, &mut env)?;
                *value = construct.construct(&mut env)?;

                for (name, value) in env.polyidents.iter() {
                    if !value.is_empty() {
                        return Err(RuntimeError::ValueError(format!(
                            "Polyident {} was used more times in the destruct pattern than in the construct pattern",
                            name
                        )));
                    }
                }
            }
        }
    }
    Ok(())
}

impl Structure for Expr {
    fn construct(&self, variables: &mut Variables) -> Result<Value, RuntimeError> {
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
            Expr::Ident(i) => variables
                .get(*i)
                .cloned()
                .ok_or_else(|| RuntimeError::ValueError(format!("Identifier {} not found", i))),

            Expr::PolyIdent(i) => variables.take_polyident(*i)?.ok_or_else(|| {
                RuntimeError::ValueError(format!("Poly-identifier {} not found", i))
            }),
            Expr::Operator(op, a, b) => {
                use parser::ast::Operator::*;
                match op {
                    Add => Ok(a.construct(variables)?.add(&b.construct(variables)?)),
                    Sub => Ok(a.construct(variables)?.sub(&b.construct(variables)?)),
                    Mul => Ok(mul(a, &b.construct(variables)?, variables)?),
                    Div => Ok(a.construct(variables)?.div(&b.construct(variables)?)),
                }
            }
            Expr::Cast(exp, to, from) => exp.construct(variables)?.cast(to, from),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::UnaryOp(op, val) => {
                let val = val.construct(variables)?;
                match (op, val) {
                    (UnaryOperator::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOperator::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (a, val) => Err(RuntimeError::ValueError(format!(
                        "Cannot apply unary operator {:?} to {:?}",
                        a, val
                    ))),
                }
            }
            Expr::Any => Err(RuntimeError::ValueError("Cannot construct `_`".to_string())),
        }
    }

    fn destruct(
        &self,
        value: &Value,
        variables: &mut Variables,
    ) -> Result<Option<Value>, RuntimeError> {
        match &self {
            Expr::Number(n) => {
                if value == &Value::Number(*n) {
                    Ok(Some(Value::Number(*n)))
                } else {
                    Err(RuntimeError::PatternMismatch(format!(
                        "Expected number {:?}",
                        n
                    )))
                }
            }
            Expr::Bool(b) => {
                if value == &Value::Bool(*b) {
                    Ok(Some(Value::Bool(*b)))
                } else {
                    Err(RuntimeError::PatternMismatch(format!(
                        "Expected bool {:?}",
                        b
                    )))
                }
            }
            Expr::String(s, _) => {
                if let Value::String(s2) = value {
                    if s == s2 {
                        Ok(Some(Value::String(s.to_owned())))
                    } else {
                        Err(RuntimeError::PatternMismatch(format!(
                            "Expected string {:?}",
                            s
                        )))
                    }
                } else {
                    Err(RuntimeError::PatternMismatch(format!(
                        "Expected string {:?}",
                        s
                    )))
                }
            }
            Expr::Array(arr) => {
                // i fugured out the destruct thing!!
                if let Value::Array(arr2) = value {
                    if arr.len() != arr2.len() {
                        return Err(RuntimeError::PatternMismatch(format!(
                            "Expected array of length {}",
                            arr.len()
                        )));
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
                    Err(RuntimeError::PatternMismatch("Expected array".to_string()))
                }
            }
            Expr::Tuple(t) => {
                if let Value::Tuple(t2) = value {
                    if t.len() != t2.len() {
                        return Err(RuntimeError::PatternMismatch(format!(
                            "Expected tuple of length {}",
                            t.len()
                        )));
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
                    Err(RuntimeError::PatternMismatch("Expected tuple".to_string()))
                }
            }
            Expr::Ident(i) => {
                variables.insert(*i, value.clone())?;
                Ok(None)
            }
            Expr::PolyIdent(i) => {
                variables.insert_polyident(*i, value.clone())?;
                Ok(None)
            }

            Expr::Operator(op, left, right) => {
                use parser::ast::Operator::*;
                match (
                    left.construct(&mut Variables::new()),
                    right.construct(&mut Variables::new()),
                ) {
                    (Ok(a), Ok(b)) => {
                        // incase some patterns both destruct and give a value (like @ in rust)
                        left.destruct(&a, variables)?;
                        right.destruct(&b, variables)?;

                        let res = match op {
                            Add => a.add(&b),
                            Sub => a.sub(&b),
                            Mul => mul(left, &b, variables)?,
                            Div => a.div(&b),
                        };
                        if &res == value {
                            Ok(Some(res))
                        } else {
                            Err(RuntimeError::PatternMismatch(format!(
                                "Expected {:?} from destruct expression, found {:?}",
                                value, res
                            )))
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

                    _ => Err(RuntimeError::ValueError(
                        "Cannot destruct expression with two unknowns".to_string(),
                    )),
                }
            }
            Expr::Cast(exp, to, from) => exp.destruct(&value.cast(from, to)?, variables),
            Expr::UnaryOp(op, val) => {
                let target_value = match (op, value) {
                    // -x = n
                    (UnaryOperator::Neg, Value::Number(n)) => Value::Number(-n),
                    // !x = b
                    (UnaryOperator::Not, Value::Bool(b)) => Value::Bool(!b),
                    (op, v) => {
                        return Err(RuntimeError::ValueError(format!(
                            "Cannot apply unary operator {:?} to {:?}",
                            op, v
                        )))
                    }
                };
                val.destruct(&target_value, variables)
            }
            Expr::Any => Ok(None),
        }
    }
}
