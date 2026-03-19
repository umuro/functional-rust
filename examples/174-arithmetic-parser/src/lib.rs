// Example 174: Arithmetic Expression Evaluator
// Full arithmetic evaluator: +,-,*,/ with precedence, parens, unary minus

type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ============================================================
// Approach 1: Recursive descent evaluator
// ============================================================

fn parse_number(input: &str) -> ParseResult<f64> {
    let s = input.trim_start();
    let bytes = s.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && (bytes[pos].is_ascii_digit() || bytes[pos] == b'.') {
        pos += 1;
    }
    if pos == 0 {
        return Err("Expected number".to_string());
    }
    let n: f64 = s[..pos]
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;
    Ok((n, &s[pos..]))
}

fn eval_expr(input: &str) -> ParseResult<f64> {
    eval_additive(input)
}

fn eval_additive(input: &str) -> ParseResult<f64> {
    let (mut lhs, mut rest) = eval_multiplicative(input)?;
    loop {
        let s = rest.trim_start();
        if s.starts_with('+') {
            let (rhs, r) = eval_multiplicative(&s[1..])?;
            lhs += rhs;
            rest = r;
        } else if s.starts_with('-') {
            let (rhs, r) = eval_multiplicative(&s[1..])?;
            lhs -= rhs;
            rest = r;
        } else {
            break;
        }
    }
    Ok((lhs, rest))
}

fn eval_multiplicative(input: &str) -> ParseResult<f64> {
    let (mut lhs, mut rest) = eval_unary(input)?;
    loop {
        let s = rest.trim_start();
        if s.starts_with('*') {
            let (rhs, r) = eval_unary(&s[1..])?;
            lhs *= rhs;
            rest = r;
        } else if s.starts_with('/') {
            let (rhs, r) = eval_unary(&s[1..])?;
            if rhs == 0.0 {
                return Err("Division by zero".to_string());
            }
            lhs /= rhs;
            rest = r;
        } else {
            break;
        }
    }
    Ok((lhs, rest))
}

fn eval_unary(input: &str) -> ParseResult<f64> {
    let s = input.trim_start();
    if s.starts_with('-') {
        let (val, rest) = eval_unary(&s[1..])?;
        Ok((-val, rest))
    } else {
        eval_primary(s)
    }
}

fn eval_primary(input: &str) -> ParseResult<f64> {
    let s = input.trim_start();
    if s.starts_with('(') {
        let (val, rest) = eval_expr(&s[1..])?;
        let rest = rest.trim_start();
        if rest.starts_with(')') {
            Ok((val, &rest[1..]))
        } else {
            Err("Expected ')'".to_string())
        }
    } else {
        parse_number(s)
    }
}

// ============================================================
// Approach 2: Evaluate string completely
// ============================================================

fn evaluate(expr: &str) -> Result<f64, String> {
    let (val, rest) = eval_expr(expr)?;
    if rest.trim().is_empty() {
        Ok(val)
    } else {
        Err(format!("Unexpected trailing: \"{}\"", rest.trim()))
    }
}

// ============================================================
// Approach 3: With built-in functions
// ============================================================

fn eval_function(name: &str, arg: f64) -> Result<f64, String> {
    match name {
        "sqrt" => Ok(arg.sqrt()),
        "abs" => Ok(arg.abs()),
        "sin" => Ok(arg.sin()),
        "cos" => Ok(arg.cos()),
        "ln" => Ok(arg.ln()),
        _ => Err(format!("Unknown function: {}", name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(evaluate("2 + 3"), Ok(5.0));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(evaluate("2 + 3 * 4"), Ok(14.0));
    }

    #[test]
    fn test_parens() {
        assert_eq!(evaluate("(2 + 3) * 4"), Ok(20.0));
    }

    #[test]
    fn test_subtraction_division() {
        assert_eq!(evaluate("10 / 2 - 3"), Ok(2.0));
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(evaluate("-5"), Ok(-5.0));
    }

    #[test]
    fn test_unary_in_parens() {
        assert_eq!(evaluate("-(2 + 3)"), Ok(-5.0));
    }

    #[test]
    fn test_multiply_negative() {
        assert_eq!(evaluate("2 * -3"), Ok(-6.0));
    }

    #[test]
    fn test_float() {
        assert_eq!(evaluate("1.5 + 2.5"), Ok(4.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate("1 / 0").is_err());
    }

    #[test]
    fn test_incomplete_expr() {
        assert!(evaluate("2 +").is_err());
    }

    #[test]
    fn test_complex() {
        let val = evaluate("(1 + 2) * (3 + 4) / 7").unwrap();
        assert!((val - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_functions() {
        assert!((eval_function("sqrt", 16.0).unwrap() - 4.0).abs() < 1e-10);
        assert_eq!(eval_function("abs", -5.0), Ok(5.0));
    }
}
