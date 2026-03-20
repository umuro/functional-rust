#![allow(clippy::all)]
//! Visitor Pattern via Fold — Expression Evaluator
//!
//! Uses fold as a visitor pattern replacement. Instead of defining a Visitor
//! trait with methods for each variant, we pass closures — one per variant.
//! This is the functional approach to the classic OOP visitor pattern.

// ── Solution 1: Idiomatic Rust — enum + match with closures ──

/// An arithmetic expression tree.
/// OCaml: `type expr = Lit of float | Add of expr * expr | Mul of expr * expr | Neg of expr`
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

/// Fold over an expression tree, applying a function for each variant.
/// This is the "visitor" — but purely functional, no trait needed.
///
/// OCaml: `val fold : lit:('a -> 'b) -> add:('b -> 'b -> 'b) -> mul:('b -> 'b -> 'b) -> neg:('b -> 'b) -> expr -> 'b`
pub fn fold<R>(
    expr: &Expr,
    lit: &dyn Fn(f64) -> R,
    add: &dyn Fn(R, R) -> R,
    mul: &dyn Fn(R, R) -> R,
    neg: &dyn Fn(R) -> R,
) -> R {
    match expr {
        Expr::Lit(x) => lit(*x),
        Expr::Add(a, b) => add(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Mul(a, b) => mul(fold(a, lit, add, mul, neg), fold(b, lit, add, mul, neg)),
        Expr::Neg(a) => neg(fold(a, lit, add, mul, neg)),
    }
}

/// Evaluate an expression to its f64 result.
/// OCaml: `let eval = fold ~lit:Fun.id ~add:(+.) ~mul:( *.) ~neg:(fun x -> -.x)`
pub fn eval(expr: &Expr) -> f64 {
    fold(
        expr,
        &|x| x,        // lit: identity
        &|a, b| a + b, // add: sum
        &|a, b| a * b, // mul: product
        &|x| -x,       // neg: negate
    )
}

/// Convert an expression to its string representation.
/// OCaml: `let to_string = fold ~lit:string_of_float ~add:... ~mul:... ~neg:...`
pub fn to_string(expr: &Expr) -> String {
    fold(
        expr,
        &|x| format!("{x}"),
        &|a, b| format!("({a} + {b})"),
        &|a, b| format!("({a} * {b})"),
        &|a| format!("(-{a})"),
    )
}

// ── Solution 2: Trait-based visitor (OOP-style, for comparison) ──

/// Traditional visitor trait — one method per variant
pub trait ExprVisitor<R> {
    fn visit_lit(&self, x: f64) -> R;
    fn visit_add(&self, a: R, b: R) -> R;
    fn visit_mul(&self, a: R, b: R) -> R;
    fn visit_neg(&self, a: R) -> R;
}

/// Accept method that dispatches to the visitor
pub fn accept<R>(expr: &Expr, visitor: &dyn ExprVisitor<R>) -> R {
    match expr {
        Expr::Lit(x) => visitor.visit_lit(*x),
        Expr::Add(a, b) => visitor.visit_add(accept(a, visitor), accept(b, visitor)),
        Expr::Mul(a, b) => visitor.visit_mul(accept(a, visitor), accept(b, visitor)),
        Expr::Neg(a) => visitor.visit_neg(accept(a, visitor)),
    }
}

/// Evaluator visitor
pub struct Evaluator;

impl ExprVisitor<f64> for Evaluator {
    fn visit_lit(&self, x: f64) -> f64 {
        x
    }
    fn visit_add(&self, a: f64, b: f64) -> f64 {
        a + b
    }
    fn visit_mul(&self, a: f64, b: f64) -> f64 {
        a * b
    }
    fn visit_neg(&self, a: f64) -> f64 {
        -a
    }
}

// ── Solution 3: Count nodes via fold ──

/// Count the number of nodes in the expression tree.
pub fn count_nodes(expr: &Expr) -> usize {
    fold(expr, &|_| 1, &|a, b| 1 + a + b, &|a, b| 1 + a + b, &|a| {
        1 + a
    })
}

/// Helper constructors for cleaner test code
pub fn lit(x: f64) -> Expr {
    Expr::Lit(x)
}

pub fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}

pub fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}

pub fn neg(a: Expr) -> Expr {
    Expr::Neg(Box::new(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Add(Mul(Lit(2.0), Lit(3.0)), Neg(Lit(1.0)))  →  (2*3) + (-1) = 5
    fn sample_expr() -> Expr {
        add(mul(lit(2.0), lit(3.0)), neg(lit(1.0)))
    }

    #[test]
    fn test_eval_simple() {
        assert!((eval(&lit(42.0)) - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_eval_compound() {
        let e = sample_expr();
        assert!((eval(&e) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_to_string() {
        let e = sample_expr();
        assert_eq!(to_string(&e), "((2 * 3) + (-1))");
    }

    #[test]
    fn test_visitor_eval() {
        let e = sample_expr();
        assert!((accept(&e, &Evaluator) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_count_nodes() {
        let e = sample_expr();
        // Lit(2), Lit(3), Mul, Lit(1), Neg, Add = 6 nodes
        assert_eq!(count_nodes(&e), 6);
    }

    #[test]
    fn test_single_lit_to_string() {
        assert_eq!(to_string(&lit(3.14)), "3.14");
    }

    #[test]
    fn test_nested_negation() {
        let e = neg(neg(lit(5.0)));
        assert!((eval(&e) - 5.0).abs() < f64::EPSILON);
        assert_eq!(to_string(&e), "(-(-5))");
    }
}
