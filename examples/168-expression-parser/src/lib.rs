#![allow(clippy::all)]
// Example 168: Expression Parser
// Pratt parsing for expressions with precedence

type ParseResult<'a, T> = Result<(T, &'a str), String>;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(f64),
    BinOp(String, Box<Expr>, Box<Expr>),
    UnaryMinus(Box<Expr>),
}

// ============================================================
// Approach 1: Pratt parser with binding power
// ============================================================

fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
    match op {
        "+" | "-" => Some((5, 6)), // left-associative
        "*" | "/" => Some((7, 8)), // left-associative
        "^" => Some((10, 9)),      // right-associative
        _ => None,
    }
}

fn prefix_binding_power(op: &str) -> Option<u8> {
    match op {
        "-" => Some(9),
        _ => None,
    }
}

fn parse_number(input: &str) -> ParseResult<'_, Expr> {
    let s = input.trim_start();
    let bytes = s.as_bytes();
    let mut pos = 0;
    if pos < bytes.len() && bytes[pos] == b'-' {
        pos += 1;
    }
    let start = pos;
    while pos < bytes.len() && bytes[pos].is_ascii_digit() {
        pos += 1;
    }
    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }
    if pos == start || (pos == 1 && bytes[0] == b'-') {
        return Err("Expected number".to_string());
    }
    let num: f64 = s[..pos]
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;
    Ok((Expr::Num(num), &s[pos..]))
}

fn parse_op(input: &str) -> ParseResult<'_, &str> {
    let s = input.trim_start();
    match s.chars().next() {
        Some(c @ ('+' | '-' | '*' | '/' | '^')) => Ok((&s[..c.len_utf8()], &s[c.len_utf8()..])),
        _ => Err("Expected operator".to_string()),
    }
}

fn pratt_expr(input: &str, min_bp: u8) -> ParseResult<'_, Expr> {
    let s = input.trim_start();

    // Prefix: parentheses, unary minus, or number
    let (mut lhs, mut rest) = if s.starts_with('(') {
        let (expr, r) = pratt_expr(&s[1..], 0)?;
        let r = r.trim_start();
        if r.starts_with(')') {
            (expr, &r[1..])
        } else {
            return Err("Expected ')'".to_string());
        }
    } else if s.starts_with('-') {
        if let Some(rbp) = prefix_binding_power("-") {
            let (rhs, r) = pratt_expr(&s[1..], rbp)?;
            (Expr::UnaryMinus(Box::new(rhs)), r)
        } else {
            parse_number(s)?
        }
    } else {
        parse_number(s)?
    };

    // Infix loop
    loop {
        let op = match parse_op(rest) {
            Ok((op, _)) => op.to_string(),
            Err(_) => break,
        };
        let (lbp, rbp) = match infix_binding_power(&op) {
            Some(bp) => bp,
            None => break,
        };
        if lbp < min_bp {
            break;
        }
        let (_, after_op) = parse_op(rest)?;
        let (rhs, r) = pratt_expr(after_op, rbp)?;
        lhs = Expr::BinOp(op, Box::new(lhs), Box::new(rhs));
        rest = r;
    }

    Ok((lhs, rest))
}

// ============================================================
// Approach 2: Evaluate directly during parsing
// ============================================================

fn eval_expr(input: &str) -> ParseResult<'_, f64> {
    fn eval_pratt(input: &str, min_bp: u8) -> ParseResult<'_, f64> {
        let s = input.trim_start();
        let (mut lhs, mut rest) = if s.starts_with('(') {
            let (val, r) = eval_pratt(&s[1..], 0)?;
            let r = r.trim_start();
            if r.starts_with(')') {
                (val, &r[1..])
            } else {
                return Err("Expected ')'".to_string());
            }
        } else if s.starts_with('-') && !s[1..].trim_start().starts_with(['+', '-', '*', '/']) {
            let (val, r) = eval_pratt(&s[1..], 9)?;
            (-val, r)
        } else {
            // parse number
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
            (n, &s[pos..])
        };

        loop {
            let trimmed = rest.trim_start();
            let op = match trimmed.chars().next() {
                Some(c @ ('+' | '-' | '*' | '/' | '^')) => c,
                _ => break,
            };
            let op_str = &trimmed[..1];
            let (lbp, rbp) = match infix_binding_power(op_str) {
                Some(bp) => bp,
                None => break,
            };
            if lbp < min_bp {
                break;
            }
            let (rhs, r) = eval_pratt(&trimmed[1..], rbp)?;
            lhs = match op {
                '+' => lhs + rhs,
                '-' => lhs - rhs,
                '*' => lhs * rhs,
                '/' => lhs / rhs,
                '^' => lhs.powf(rhs),
                _ => unreachable!(),
            };
            rest = r;
        }
        Ok((lhs, rest))
    }
    eval_pratt(input, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let (expr, _) = pratt_expr("1 + 2", 0).unwrap();
        assert_eq!(
            expr,
            Expr::BinOp(
                "+".into(),
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0))
            )
        );
    }

    #[test]
    fn test_precedence() {
        // 1 + 2 * 3 = 1 + (2*3)
        let (expr, _) = pratt_expr("1 + 2 * 3", 0).unwrap();
        match expr {
            Expr::BinOp(ref op, _, ref rhs) => {
                assert_eq!(op, "+");
                match rhs.as_ref() {
                    Expr::BinOp(ref op2, _, _) => assert_eq!(op2, "*"),
                    _ => panic!("Expected BinOp"),
                }
            }
            _ => panic!("Expected BinOp"),
        }
    }

    #[test]
    fn test_parens() {
        let (expr, _) = pratt_expr("(1 + 2) * 3", 0).unwrap();
        match expr {
            Expr::BinOp(ref op, ref lhs, _) => {
                assert_eq!(op, "*");
                match lhs.as_ref() {
                    Expr::BinOp(ref op2, _, _) => assert_eq!(op2, "+"),
                    _ => panic!("Expected BinOp"),
                }
            }
            _ => panic!("Expected BinOp"),
        }
    }

    #[test]
    fn test_right_assoc() {
        // 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 512
        let (val, _) = eval_expr("2 ^ 3 ^ 2").unwrap();
        assert!((val - 512.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_precedence() {
        let (val, _) = eval_expr("1 + 2 * 3").unwrap();
        assert!((val - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_parens() {
        let (val, _) = eval_expr("(1 + 2) * 3").unwrap();
        assert!((val - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_unary_minus() {
        let (val, _) = eval_expr("-5").unwrap();
        assert!((val - (-5.0)).abs() < 1e-10);
    }
}
