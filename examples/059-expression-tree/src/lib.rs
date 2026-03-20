#![allow(clippy::all)]
//! # Recursive Variant — Expression Tree
//!
//! OCaml's recursive variants with payloads map to Rust's `enum` with
//! `Box`-wrapped recursive fields. The key difference: Rust requires
//! explicit heap allocation (`Box`) for recursive types because it must
//! know the size of every type at compile time.

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — enum with Box, methods via impl
// ---------------------------------------------------------------------------

/// An arithmetic expression tree.
///
/// `Box<Expr>` is required because `Expr` is recursive — without it,
/// `Expr` would have infinite size. OCaml handles this implicitly because
/// all variant payloads are heap-allocated.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

/// Convenience constructors to avoid writing `Box::new(...)` everywhere.
/// This is a common Rust pattern for recursive data structures.
impl Expr {
    pub fn num(n: f64) -> Self {
        Expr::Num(n)
    }

    pub fn new_add(l: Expr, r: Expr) -> Self {
        Expr::Add(Box::new(l), Box::new(r))
    }

    pub fn new_sub(l: Expr, r: Expr) -> Self {
        Expr::Sub(Box::new(l), Box::new(r))
    }

    pub fn new_mul(l: Expr, r: Expr) -> Self {
        Expr::Mul(Box::new(l), Box::new(r))
    }

    pub fn new_div(l: Expr, r: Expr) -> Self {
        Expr::Div(Box::new(l), Box::new(r))
    }
}

impl Expr {
    /// Evaluate the expression tree recursively.
    ///
    /// Mirrors OCaml's `eval` function — structural recursion over the variant.
    /// Takes `&self` (borrowed reference) rather than consuming the tree.
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Add(l, r) => l.eval() + r.eval(),
            Expr::Sub(l, r) => l.eval() - r.eval(),
            Expr::Mul(l, r) => l.eval() * r.eval(),
            Expr::Div(l, r) => l.eval() / r.eval(),
        }
    }
}

/// Display produces the same parenthesized format as OCaml's `to_string`.
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Add(l, r) => write!(f, "({l} + {r})"),
            Expr::Sub(l, r) => write!(f, "({l} - {r})"),
            Expr::Mul(l, r) => write!(f, "({l} * {r})"),
            Expr::Div(l, r) => write!(f, "({l} / {r})"),
        }
    }
}

// ---------------------------------------------------------------------------
// Approach B: Free functions — closer to OCaml style
// ---------------------------------------------------------------------------

/// Evaluate as a standalone function (OCaml style).
pub fn eval(expr: &Expr) -> f64 {
    expr.eval()
}

/// Convert to string as a standalone function.
pub fn to_string(expr: &Expr) -> String {
    format!("{expr}")
}

// ---------------------------------------------------------------------------
// Approach C: Safe division with Result
// ---------------------------------------------------------------------------

/// Division-safe evaluation that returns an error on divide-by-zero
/// instead of producing `inf` or `NaN`.
///
/// This is a Rust improvement over the OCaml version — OCaml's version
/// silently produces `infinity` on division by zero.
pub fn eval_safe(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(l, r) => Ok(eval_safe(l)? + eval_safe(r)?),
        Expr::Sub(l, r) => Ok(eval_safe(l)? - eval_safe(r)?),
        Expr::Mul(l, r) => Ok(eval_safe(l)? * eval_safe(r)?),
        Expr::Div(l, r) => {
            let divisor = eval_safe(r)?;
            if divisor == 0.0 {
                Err(format!("Division by zero: {expr}"))
            } else {
                Ok(eval_safe(l)? / divisor)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (1 + 2) * (10 - 4) = 18
    fn sample_expr() -> Expr {
        Expr::new_mul(
            Expr::new_add(Expr::num(1.0), Expr::num(2.0)),
            Expr::new_sub(Expr::num(10.0), Expr::num(4.0)),
        )
    }

    #[test]
    fn test_eval_basic() {
        assert!((sample_expr().eval() - 18.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_eval_single_num() {
        assert!((Expr::num(42.0).eval() - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display() {
        assert_eq!(to_string(&sample_expr()), "((1 + 2) * (10 - 4))");
    }

    #[test]
    fn test_eval_nested_division() {
        // 10 / (5 - 3) = 5
        let e = Expr::new_div(
            Expr::num(10.0),
            Expr::new_sub(Expr::num(5.0), Expr::num(3.0)),
        );
        assert!((e.eval() - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_eval_safe_division_by_zero() {
        let e = Expr::new_div(Expr::num(1.0), Expr::num(0.0));
        assert!(eval_safe(&e).is_err());
    }

    #[test]
    fn test_eval_safe_ok() {
        assert!((eval_safe(&sample_expr()).unwrap() - 18.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display_num() {
        assert_eq!(format!("{}", Expr::num(3.14)), "3.14");
    }

    #[test]
    fn test_clone_and_eq() {
        let e = sample_expr();
        let e2 = e.clone();
        assert_eq!(e, e2);
    }
}
