#![allow(clippy::all)]
//! # Finally Tagless (Tagless Final)
//!
//! Extensible DSL interpretation using traits instead of data types.

/// Expression algebra - defines operations without concrete types.
pub trait ExprAlg {
    type Repr;
    fn lit(&self, n: i32) -> Self::Repr;
    fn add(&self, a: Self::Repr, b: Self::Repr) -> Self::Repr;
    fn mul(&self, a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

/// Evaluator interpretation - expressions become values.
pub struct Eval;

impl ExprAlg for Eval {
    type Repr = i32;
    fn lit(&self, n: i32) -> i32 {
        n
    }
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn mul(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

/// Pretty printer interpretation - expressions become strings.
pub struct Pretty;

impl ExprAlg for Pretty {
    type Repr = String;
    fn lit(&self, n: i32) -> String {
        n.to_string()
    }
    fn add(&self, a: String, b: String) -> String {
        format!("({} + {})", a, b)
    }
    fn mul(&self, a: String, b: String) -> String {
        format!("({} * {})", a, b)
    }
}

/// Generic expression builder.
pub fn example_expr<A: ExprAlg>(alg: &A) -> A::Repr {
    // (1 + 2) * 3
    alg.mul(alg.add(alg.lit(1), alg.lit(2)), alg.lit(3))
}

/// Extended algebra with subtraction.
pub trait ExprAlgExt: ExprAlg {
    fn sub(&self, a: Self::Repr, b: Self::Repr) -> Self::Repr;
}

impl ExprAlgExt for Eval {
    fn sub(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

impl ExprAlgExt for Pretty {
    fn sub(&self, a: String, b: String) -> String {
        format!("({} - {})", a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let result = example_expr(&Eval);
        assert_eq!(result, 9); // (1+2)*3
    }

    #[test]
    fn test_pretty() {
        let result = example_expr(&Pretty);
        assert_eq!(result, "((1 + 2) * 3)");
    }

    #[test]
    fn test_extended() {
        let eval = Eval;
        let result = eval.sub(eval.lit(10), eval.lit(3));
        assert_eq!(result, 7);
    }
}
