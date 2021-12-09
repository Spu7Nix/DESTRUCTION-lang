use parser::ast::{Type, UnaryOperator};
use parser::internment::LocalIntern;

use crate::error::RuntimeError;
use crate::traits::{Functions, Maths, Structure, Value, Variables};
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
            (Type::String, v) => Ok(Self::String(format!("{}", v))),
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
    fn add(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs.to_owned() + rhs)),
            (Value::Array(lhs), Value::Array(rhs)) => {
                Ok(Value::Array([lhs.to_owned(), rhs.to_owned()].concat()))
            }
            (a, b) => Err(RuntimeError::ValueError(format!(
                "Cannot add {:?} and {:?}",
                a, b
            ))),
        }
    }

    fn sub(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs - rhs)),
            (a, b) => Err(RuntimeError::ValueError(format!(
                "Cannot subtract {:?} and {:?}",
                a, b
            ))),
        }
    }

    fn div(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs / rhs)),
            _ => Err(RuntimeError::ValueError(format!(
                "Cannot divide {:?} and {:?}",
                self, other
            ))),
        }
    }

    fn and(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(*lhs && *rhs)),
            _ => Err(RuntimeError::ValueError(format!(
                "Cannot and {:?} and {:?}",
                self, other
            ))),
        }
    }

    fn or(&self, other: &Self) -> Result<Value, RuntimeError> {
        match (self, other) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(*lhs || *rhs)),
            _ => Err(RuntimeError::ValueError(format!(
                "Cannot or {:?} and {:?}",
                self, other
            ))),
        }
    }
}

fn mul(
    left: &Expr,
    right: &Value,
    variables: &mut Variables,
    functions: &Functions,
) -> Result<Value, RuntimeError> {
    let factor = *match right {
        Value::Number(n) => n,
        _ => {
            return Err(RuntimeError::TypeMismatch(
                "number".to_string(),
                right.to_type().to_string(),
            ))
        }
    };
    let mut out = left.construct(variables, functions)?;
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
        out = out.add(&left.construct(variables, functions)?)?;
    }
    Ok(out)
}

pub fn interpret(top_level: TopLevel, input: Value) -> Result<Value, RuntimeError> {
    run_func(
        LocalIntern::new("main".to_string()),
        input,
        &top_level.functions,
    )
}

fn run_func(
    func: LocalIntern<String>,
    mut value: Value,
    functions: &Functions,
) -> Result<Value, RuntimeError> {
    for trans in functions
        .get(&func)
        .ok_or_else(|| RuntimeError::ValueError(format!("Missing `{}` function", func)))?
    {
        let mut env = Variables::new();
        match trans {
            Forced {
                destruct,
                construct,
            } => {
                destruct.destruct(&value, &mut env, functions)?;
                value = construct.construct(&mut env, functions)?;

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
    Ok(value)
}

fn reverse_run_func(
    func: LocalIntern<String>,
    mut output: Value,
    functions: &Functions,
) -> Result<Value, RuntimeError> {
    for trans in functions
        .get(&func)
        .ok_or_else(|| RuntimeError::ValueError(format!("Missing `{}` function", func)))?
        .iter()
        .rev()
    {
        let mut env = Variables::new();
        match trans {
            Forced {
                destruct,
                construct,
            } => {
                construct.destruct(&output, &mut env, functions)?;
                output = destruct.construct(&mut env, functions)?;

                for (name, value) in env.polyidents.iter() {
                    if !value.is_empty() {
                        return Err(RuntimeError::ValueError(format!(
                            "Polyident {} was used more times in the construct pattern than in the destruct pattern",
                            name
                        )));
                    }
                }
            }
        }
    }
    Ok(output)
}

impl Structure for Expr {
    fn construct(
        &self,
        variables: &mut Variables,
        functions: &Functions,
    ) -> Result<Value, RuntimeError> {
        match self {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s, _) => Ok(Value::String(s.to_owned())), // btw we can make strings localintern
            Expr::Array(arr) => Ok(Value::Array(
                arr.iter()
                    .map(|e| -> Result<_, _> { e.construct(variables, functions) })
                    .collect::<Result<_, _>>()?,
            )),
            Expr::Tuple(t) => Ok(Value::Tuple(
                t.iter()
                    .map(|e| -> Result<_, _> { e.construct(variables, functions) })
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
                    Add => Ok(a
                        .construct(variables, functions)?
                        .add(&b.construct(variables, functions)?)?),
                    Sub => Ok(a
                        .construct(variables, functions)?
                        .sub(&b.construct(variables, functions)?)?),
                    Mul => Ok(mul(
                        a,
                        &b.construct(variables, functions)?,
                        variables,
                        functions,
                    )?),
                    Div => Ok(a
                        .construct(variables, functions)?
                        .div(&b.construct(variables, functions)?)?),
                    And => Ok(a
                        .construct(variables, functions)?
                        .and(&b.construct(variables, functions)?)?),
                    Or => Ok(a
                        .construct(variables, functions)?
                        .or(&b.construct(variables, functions)?)?),
                }
            }
            Expr::Cast(exp, to, from) => exp.construct(variables, functions)?.cast(to, from),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::UnaryOp(op, val) => {
                let val = val.construct(variables, functions)?;
                match (op, val) {
                    (UnaryOperator::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOperator::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (a, val) => Err(RuntimeError::ValueError(format!(
                        "Cannot apply unary operator {:?} to {}",
                        a, val
                    ))),
                }
            }
            Expr::Any => Err(RuntimeError::ValueError("Cannot construct `_`".to_string())),
            Expr::Call(f, a) => run_func(*f, a.construct(variables, functions)?, functions),
        }
    }

    fn destruct(
        &self,
        value: &Value,
        variables: &mut Variables,
        functions: &Functions,
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
                            (e.destruct(v, variables, functions)?, &mut arr_val)
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
                            (e.destruct(v, variables, functions)?, &mut arr_val)
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
                    left.construct(&mut Variables::new(), functions),
                    right.construct(&mut Variables::new(), functions),
                ) {
                    (Ok(a), Ok(b)) => {
                        // incase some patterns both destruct and give a value (like @ in rust)
                        left.destruct(&a, variables, functions)?;
                        right.destruct(&b, variables, functions)?;

                        let res = match op {
                            Add => a.add(&b)?,
                            Sub => a.sub(&b)?,
                            Mul => mul(left, &b, variables, functions)?,
                            Div => a.div(&b)?,
                            And => a.and(&b)?,
                            Or => a.or(&b)?,
                        };
                        if &res == value {
                            Ok(Some(res))
                        } else {
                            Err(RuntimeError::PatternMismatch(format!(
                                "Expected {} from destruct expression, found {}",
                                value, res
                            )))
                        }
                    }

                    (Ok(left), Err(_)) => {
                        match op {
                            Add => destruct_algebra::add_left_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                            Sub => destruct_algebra::sub_left_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                            Mul => destruct_algebra::mul_left_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                            Div => destruct_algebra::div_left_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                            And => destruct_algebra::and_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                            Or => destruct_algebra::or_destruct(
                                &left, &*right, value, variables, functions,
                            )?,
                        };
                        Ok(None)
                    }
                    (Err(_), Ok(right)) => {
                        match op {
                            Add => destruct_algebra::add_right_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                            Sub => destruct_algebra::sub_right_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                            Mul => destruct_algebra::mul_right_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                            Div => destruct_algebra::div_right_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                            And => destruct_algebra::and_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                            Or => destruct_algebra::or_destruct(
                                &right, &*left, value, variables, functions,
                            )?,
                        };
                        Ok(None)
                    }

                    _ => Err(RuntimeError::ValueError(
                        "Cannot destruct expression with two unknowns".to_string(),
                    )),
                }
            }
            Expr::Cast(exp, to, from) => exp.destruct(&value.cast(from, to)?, variables, functions),
            Expr::UnaryOp(op, val) => {
                let target_value = match (op, value) {
                    // -x = n
                    (UnaryOperator::Neg, Value::Number(n)) => Value::Number(-n),
                    // !x = b
                    (UnaryOperator::Not, Value::Bool(b)) => Value::Bool(!b),
                    (op, v) => {
                        return Err(RuntimeError::ValueError(format!(
                            "Cannot apply unary operator {:?} to {}",
                            op, v
                        )))
                    }
                };
                val.destruct(&target_value, variables, functions)
            }
            Expr::Any => Ok(None),
            Expr::Call(f, a) => {
                let target_val = reverse_run_func(*f, value.clone(), functions)?;
                a.destruct(&target_val, variables, functions)
            }
        }
    }
}
