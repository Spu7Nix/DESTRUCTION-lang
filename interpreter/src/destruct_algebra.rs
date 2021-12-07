use crate::{
    error::RuntimeError,
    traits::{Value, Variables},
};
use parser::ast::Expr;

pub fn add_left_destruct(
    left: &Value,
    right: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        Value::String(s) => todo!(),
        Value::Array(a) => todo!(),
        _ => Err(RuntimeError::ValueError),
    }
}

pub fn add_right_destruct(
    right: &Value,
    left: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn sub_left_destruct(
    left: &Value,
    right: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        _ => Err(RuntimeError::ValueError),
    }
}

pub fn sub_right_destruct(
    right: &Value,
    left: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn mul_left_destruct(
    left: &Value,
    right: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        Value::Array(a) => todo!(),
        Value::String(s) => todo!(),
        _ => Err(RuntimeError::ValueError),
    }
}

pub fn mul_right_destruct(
    right: &Value,
    left: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn div_left_destruct(
    left: &Value,
    right: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    match left {
        Value::Number(n) => todo!(),
        _ => Err(RuntimeError::ValueError),
    }
}

pub fn div_right_destruct(
    right: &Value,
    left: &Expr,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    todo!()
}
