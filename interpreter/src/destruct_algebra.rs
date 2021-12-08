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
                return Err(RuntimeError::PatternMismatch(format!(
                    "Expected {} to start with {}",
                    s2, s1
                )));
            }
            Value::String(s2[s1.len()..].to_string())
        }
        // a1 + x = a2
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.starts_with(a1) {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Expected {:?} to start with {:?}",
                    a2, a1
                )));
            }
            Value::Array(a2[a1.len()..].to_vec())
        }
        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot add {:?} with something to get {:?}",
                left, target_val
            )))
        }
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
                return Err(RuntimeError::PatternMismatch(format!(
                    "Expected {} to end with {}",
                    s2, s1
                )));
            }
            Value::String(s2[..(s2.len() - s1.len())].to_string())
        }
        // x + a1 = a2
        (Value::Array(a1), Value::Array(a2)) => {
            if !a2.ends_with(a1) {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Expected {:?} to end with {:?}",
                    a2, a1
                )));
            }
            Value::Array(a2[..(a2.len() - a1.len())].to_vec())
        }
        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot add something with {:?} to get {:?}",
                right, target_val
            )))
        }
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

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot subtract {:?} from something to get {:?}",
                left, target_val
            )))
        }
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

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot subtract something from {:?} to get {:?}",
                right, target_val
            )))
        }
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
                return Err(RuntimeError::PatternMismatch(format!(
                    "Length of target array {:?} is not divisible by length of destruct array {:?}",
                    a2, a1
                )));
            }
            let repeats = a2.len() / a1.len();
            for (i, el) in a2.iter().enumerate() {
                if a1[i % a1.len()] != *el {
                    return Err(RuntimeError::PatternMismatch(format!("Element {:?} at index {:?} of target array {:?} does not match element {:?} at index {:?} of destruct array {:?}", el, i, a2, a1[i % a1.len()], i % a1.len(), a1)));
                }
            }

            Value::Number(repeats as f64)
        }
        (Value::String(s1), Value::String(s2)) => {
            if s2.len() % s1.len() != 0 {
                return Err(RuntimeError::PatternMismatch(format!("Length of target string {:?} is not divisible by length of destruct string {:?}", s2, s1)));
            }
            let repeats = s2.len() / s1.len();
            for (i, c) in s2.bytes().enumerate() {
                // since .len() is the bytes
                if s1.as_bytes()[i % s1.len()] != c {
                    return Err(RuntimeError::PatternMismatch(format!("Character {:?} at index {:?} of target string {:?} does not match character {:?} at index {:?} of destruct string {:?}", c as char, i, s2, s1.as_bytes()[i % s1.len()] as char, i % s1.len(), s1)));
                }
            }
            Value::Number(repeats as f64)
        }

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot multiply {:?} with something to get {:?}",
                left, target_val
            )))
        }
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
    match (right, target_val) {
        // x * n1 = n2
        (Value::Number(n1), Value::Number(n2)) => {
            let target_val = Value::Number(n2 / n1);
            left.destruct(&target_val, variables)?;
            Ok(())
        }
        (Value::Number(n1), Value::Array(a2)) => {
            if n1.fract() != 0.0 {
                return Err(RuntimeError::ValueError(format!(
                    "Cannot multiply array with non-integer number {}",
                    n1
                )));
            }
            let n = *n1 as usize;
            if n > a2.len() {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Cannot multiply array with number {} greater than length of array {}",
                    n,
                    a2.len()
                )));
            }
            if a2.len() % n != 0 {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Length of array {:?} is not divisible by number {}",
                    a2, n
                )));
            }
            let len = a2.len() / n;

            for i in 0..n {
                let target = &Value::Array(a2[i * len..(i + 1) * len].to_vec());
                left.destruct(target, variables)?;
            }

            Ok(())
        }

        (Value::Number(n1), Value::String(s2)) => {
            if n1.fract() != 0.0 {
                return Err(RuntimeError::ValueError(format!(
                    "Cannot multiply string with non-integer number {}",
                    n1
                )));
            }
            let n = *n1 as usize;
            if n > s2.len() {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Cannot multiply string with number {} greater than length of string {}",
                    n,
                    s2.len()
                )));
            }
            if s2.len() % n != 0 {
                return Err(RuntimeError::PatternMismatch(format!(
                    "Length of string {:?} is not divisible by number {}",
                    s2, n
                )));
            }
            let len = s2.len() / n;

            for i in 0..n {
                let target = &Value::String(s2[i * len..(i + 1) * len].to_string());
                left.destruct(target, variables)?;
            }

            Ok(())
        }

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot multiply something with {:?} to get {:?}",
                right, target_val
            )))
        }
    }
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

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot divide {:?} with something to get {:?}",
                left, target_val
            )))
        }
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

        _ => {
            return Err(RuntimeError::ValueError(format!(
                "Cannot divide something with {:?} to get {:?}",
                right, target_val
            )))
        }
    };

    left.destruct(&target_val, variables)?;
    Ok(())
}
