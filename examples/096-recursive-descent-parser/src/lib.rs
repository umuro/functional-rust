//! # Simple Recursive Descent Parser
//!
//! Parse arithmetic expressions into an AST with correct precedence.
//! OCaml's mutual recursion with `and` maps to Rust functions calling each other.

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

// ---------------------------------------------------------------------------
// Approach A: Slice-based parser (mirrors OCaml's list consumption)
// ---------------------------------------------------------------------------

pub fn parse<'a>(tokens: &'a [&'a str]) -> Result<Expr, String> {
    let (expr, rest) = parse_expr(tokens)?;
    if rest.is_empty() {
        Ok(expr)
    } else {
        Err(format!("unexpected tokens: {:?}", rest))
    }
}

fn parse_expr<'a>(tokens: &'a [&'a str]) -> Result<(Expr, &'a [&'a str]), String> {
    let (left, rest) = parse_term(tokens)?;
    if let Some((&"+", rest)) = rest.split_first() {
        let (right, rest) = parse_expr(rest)?;
        Ok((Expr::Add(Box::new(left), Box::new(right)), rest))
    } else {
        Ok((left, rest))
    }
}

fn parse_term<'a>(tokens: &'a [&'a str]) -> Result<(Expr, &'a [&'a str]), String> {
    let (left, rest) = parse_atom(tokens)?;
    if let Some((&"*", rest)) = rest.split_first() {
        let (right, rest) = parse_term(rest)?;
        Ok((Expr::Mul(Box::new(left), Box::new(right)), rest))
    } else {
        Ok((left, rest))
    }
}

fn parse_atom<'a>(tokens: &'a [&'a str]) -> Result<(Expr, &'a [&'a str]), String> {
    match tokens.split_first() {
        Some((token, rest)) => {
            let n: i64 = token.parse().map_err(|_| format!("not a number: {}", token))?;
            Ok((Expr::Num(n), rest))
        }
        None => Err("unexpected end of input".to_string()),
    }
}

// ---------------------------------------------------------------------------
// Approach B: Evaluator (mirrors OCaml's eval)
// ---------------------------------------------------------------------------

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

// ---------------------------------------------------------------------------
// Approach C: Index-based parser (avoids slice lifetimes)
// ---------------------------------------------------------------------------

pub fn parse_and_eval(tokens: &[&str]) -> Result<i64, String> {
    let expr = parse(tokens)?;
    Ok(eval(&expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        assert_eq!(parse_and_eval(&["2", "+", "3"]), Ok(5));
    }

    #[test]
    fn test_simple_mul() {
        assert_eq!(parse_and_eval(&["2", "*", "3"]), Ok(6));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(parse_and_eval(&["2", "+", "3", "*", "4"]), Ok(14));
    }

    #[test]
    fn test_single_number() {
        assert_eq!(parse_and_eval(&["42"]), Ok(42));
    }

    #[test]
    fn test_empty() {
        assert!(parse_and_eval(&[]).is_err());
    }

    #[test]
    fn test_complex() {
        assert_eq!(parse_and_eval(&["1", "+", "2", "+", "3"]), Ok(6));
    }
}
