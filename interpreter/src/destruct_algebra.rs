use crate::{
    error::RuntimeError,
    traits::{Value, Variables},
};
use parser::ast::Expr;

pub fn add_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> { // (a * 4 - 6) + 10 -> a // input 15 output 5 
    let target_val = match (left, target_val) {
        // num + x = num
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 - n1),
        // string + x = string 
        (Value::String(s1), Value::String(s2)) => {
            if !s2.starts_with(s1) {
                return Err(RuntimeError::PatternMismatchT); 
            }
            Value::String(s2[s1.len()..].to_string())
        },
        // arr + x = arr
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.starts_with(a1) {
                return Err(RuntimeError::PatternMismatchT); 
            }
            Value::Array(a2[a1.len()..].to_vec())
        },
        _ => return Err(RuntimeError::ValueErrorT),
    };
    right.destruct(target_val, variables)?;
    Ok(())
}

pub fn add_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (right, target_val) {
        // x + num = num
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 - n1),
        // x + string = string
        (Value::String(s1), Value::String(s2)) => {
            if !s2.ends_with(s1) {
                return Err(RuntimeError::PatternMismatchT); 
            }
            Value::String(s2[..(s2.len() - s1.len())].to_string())
        },
        // x + arr = arr
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.ends_with(a1) {
                return Err(RuntimeError::PatternMismatchT); 
            }
            Value::Array(a2[..(a2.len() - a1.len())].to_vec())
        },
        _ => Err(RuntimeError::ValueErrorT),
    };

    left.destruct(target_val, variables)?;
    Ok(())
}

pub fn sub_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        _ => Err(RuntimeError::ValueErrorT),
    }
}

pub fn sub_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn mul_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        Value::Array(a) => todo!(),
        Value::String(s) => todo!(),
        _ => Err(RuntimeError::ValueErrorT),
    }
}

pub fn mul_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn div_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        _ => Err(RuntimeError::ValueErrorT),
    }
}

pub fn div_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}
