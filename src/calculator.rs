pub fn perform_operation(op: &str, op1: f64, op2: f64) -> Result<f64, &'static str> {
    match op {
        "add" | "+" => Ok(op1 + op2),
        "sub" | "-" => Ok(op1 - op2),
        "mul" | "*" => Ok(op1 * op2),
        "div" | "/" => {
            if op2 == 0.0 {
                Err("Cannot divide by zero")
            } else {
                Ok(op1 / op2)
            }
        },
        _ => Err("Invalid operation"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(perform_operation("add", 2.0, 3.0).unwrap(), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(perform_operation("sub", 5.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(perform_operation("mul", 2.0, 3.0).unwrap(), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(perform_operation("div", 6.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(perform_operation("div", 1.0, 0.0).unwrap_err(), "Cannot divide by zero");
    }

    #[test]
    fn test_invalid_operation() {
        assert_eq!(perform_operation("invalid", 1.0, 1.0).unwrap_err(), "Invalid operation");
    }
}
