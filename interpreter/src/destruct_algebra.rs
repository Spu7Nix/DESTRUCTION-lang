use crate::{
    error::RuntimeError,
    traits::{Structure, Value, Variables},
};
use parser::ast::Expr;

pub fn add_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    // (a * 4 - 6) + 10 -> a // input 15 output 5
    let target_val = match (left, target_val) {
        // n1 + x = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 - n1),
        // s1 + x = s2
        (Value::String(s1), Value::String(s2)) => {
            if !s2.starts_with(s1) {
                return Err(RuntimeError::PatternMismatchT);
            }
            Value::String(s2[s1.len()..].to_string())
        }
        // a1 + x = a2
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.starts_with(a1) {
                return Err(RuntimeError::PatternMismatchT);
            }
            Value::Array(a2[a1.len()..].to_vec())
        }
        _ => return Err(RuntimeError::ValueErrorT),
    };
    right.destruct(&target_val, variables)?;
    Ok(())
}

pub fn add_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (right, target_val) {
        // x + n1 = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 - n1),
        // x + s1 = s2
        (Value::String(s1), Value::String(s2)) => {
            if !s2.ends_with(s1) {
                return Err(RuntimeError::PatternMismatchT);
            }
            Value::String(s2[..(s2.len() - s1.len())].to_string())
        }
        // x + a1 = a2
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.ends_with(a1) {
                return Err(RuntimeError::PatternMismatchT);
            }
            Value::Array(a2[..(a2.len() - a1.len())].to_vec())
        }
        _ => return Err(RuntimeError::ValueErrorT),
    };

    left.destruct(&target_val, variables)?;
    Ok(())
}

pub fn sub_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (left, target_val) {
        // n1 - x = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),

        _ => return Err(RuntimeError::ValueErrorT),
    };
    right.destruct(&target_val, variables)?;
    Ok(())
}

pub fn sub_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (right, target_val) {
        // x - n1 = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),

        _ => return Err(RuntimeError::ValueErrorT),
    };

    left.destruct(&target_val, variables)?;
    Ok(())
}

pub fn mul_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (left, target_val) {
        // n1 * x = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 / n1),
        (Value::Array(a1), Value::Array(a2)) => {
            if a2.len() % a1.len() != 0 {
                return Err(RuntimeError::PatternMismatchT);
            }
            let repeats = a2.len() / a1.len();
            for (i, el) in a2.iter().enumerate() {
                if a1[i % a1.len()] != *el {
                    return Err(RuntimeError::PatternMismatchT);
                }
            }

            Value::Number(repeats as f64)
        }
        (Value::String(s1), Value::String(s2)) => {
            if s2.len() % s1.len() != 0 {
                return Err(RuntimeError::PatternMismatchT);
            }
            let repeats = s2.len() / s1.len();
            for (i, c) in s2.bytes().enumerate() {
                // since .len() is the bytes
                if s1.as_bytes()[i % s1.len()] != c {
                    return Err(RuntimeError::PatternMismatchT);
                }
            }
            Value::Number(repeats as f64)
        }

        _ => return Err(RuntimeError::ValueErrorT),
    };
    right.destruct(&target_val, variables)?;
    Ok(())
}

pub fn mul_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (right, target_val) {
        // x * n1 = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n2 / n1),
        (Value::Number(n1), Value::Array(a2)) => {
            if n1.fract() != 0.0 {
                return Err(RuntimeError::ValueErrorT);
            }
            let n = *n1 as usize;
            if n > a2.len() {
                return Err(RuntimeError::PatternMismatchT);
            }
            if a2.len() % n != 0 {
                return Err(RuntimeError::PatternMismatchT);
            }
            let len = a2.len() / n;
            let vec = a2[..len].to_vec();
            for (i, el) in a2.iter().enumerate() {
                if vec[i % len] != *el {
                    return Err(RuntimeError::PatternMismatchT);
                }
            }

            Value::Array(vec)
        }

        (Value::Number(n1), Value::String(s2)) => {
            if n1.fract() != 0.0 {
                return Err(RuntimeError::ValueErrorT);
            }
            let n = *n1 as usize;
            if n > s2.len() {
                return Err(RuntimeError::PatternMismatchT);
            }
            if s2.len() % n != 0 {
                return Err(RuntimeError::PatternMismatchT);
            }
            let len = s2.len() / n;
            let vec = s2[..len].to_string();
            for (i, c) in s2.bytes().enumerate() {
                // since .len() is the bytes
                if vec.as_bytes()[i % len] != c {
                    return Err(RuntimeError::PatternMismatchT);
                }
            }

            Value::String(vec)
        }

        _ => return Err(RuntimeError::ValueErrorT),
    };

    left.destruct(&target_val, variables)?;
    Ok(())
}

pub fn div_left_destruct(
    left: &Value,
    right: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (left, target_val) {
        // n1 / x = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),

        _ => return Err(RuntimeError::ValueErrorT),
    };
    right.destruct(&target_val, variables)?;
    Ok(())
}

pub fn div_right_destruct(
    right: &Value,
    left: &Expr,
    target_val: &Value,
    variables: &mut Variables,
) -> Result<(), RuntimeError> {
    let target_val = match (right, target_val) {
        // x / n1 = n2
        (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),

        _ => return Err(RuntimeError::ValueErrorT),
    };

    left.destruct(&target_val, variables)?;
    Ok(())
}
