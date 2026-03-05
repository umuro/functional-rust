//! # Example 215: Recursion Schemes — Separating What From How
//!
//! Demonstrates how to factor recursion *out* of business logic using
//! catamorphisms. Instead of copying the same traversal into every function,
//! you write one `cata` that handles the recursion, and plain functions
//! (algebras) that handle only the local computation step.

// ============================================================
// Approach 1: Direct recursion — recursion is mixed with logic
// ============================================================

/// A simple arithmetic expression tree.
#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn lit(n: i64) -> Self {
        Expr::Lit(n)
    }
    pub fn make_add(a: Expr, b: Expr) -> Self {
        Expr::Add(Box::new(a), Box::new(b))
    }
    pub fn make_mul(a: Expr, b: Expr) -> Self {
        Expr::Mul(Box::new(a), Box::new(b))
    }
}

/// Evaluate — recursion entangled with arithmetic.
pub fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Lit(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

/// Pretty-print — same traversal structure, different payload.
pub fn show(e: &Expr) -> String {
    match e {
        Expr::Lit(n) => n.to_string(),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
    }
}

/// Tree depth — yet again the identical recursive shape.
pub fn depth(e: &Expr) -> usize {
    match e {
        Expr::Lit(_) => 0,
        Expr::Add(a, b) | Expr::Mul(a, b) => 1 + depth(a).max(depth(b)),
    }
}

// ============================================================
// Approach 2: Recursion Schemes — factor the recursion out
// ============================================================
//
// Key idea: make a "base functor" ExprF<A> where every recursive
// occurrence of Expr is replaced by a type variable A.
// Then:
//   - `ExprF<Box<Expr>>` ≅ one layer of Expr (= `project`)
//   - `map` applies a function to every recursive position
//   - `cata` wires map + recursion together, accepting an "algebra"
//     `ExprF<B> -> B` that only handles the *local* step.
//
// Result: eval, show, depth become three-line functions with zero
// recursion. Adding a new traversal costs only the algebra.

/// Base functor: `Expr` with recursive positions replaced by `A`.
pub enum ExprF<A> {
    Lit(i64),
    Add(A, A),
    Mul(A, A),
}

impl<A> ExprF<A> {
    /// Functorial map — apply `f` to every recursive position.
    pub fn map<B, F: Fn(A) -> B>(self, f: F) -> ExprF<B> {
        match self {
            ExprF::Lit(n) => ExprF::Lit(n),
            ExprF::Add(a, b) => ExprF::Add(f(a), f(b)),
            ExprF::Mul(a, b) => ExprF::Mul(f(a), f(b)),
        }
    }
}

/// Project: peel off one layer of `Expr`, exposing `ExprF<Box<Expr>>`.
fn project(e: Expr) -> ExprF<Box<Expr>> {
    match e {
        Expr::Lit(n) => ExprF::Lit(n),
        Expr::Add(a, b) => ExprF::Add(a, b),
        Expr::Mul(a, b) => ExprF::Mul(a, b),
    }
}

/// Catamorphism: the *only* place recursion lives.
///
/// Given an algebra `f: ExprF<A> -> A`, folds the whole tree bottom-up:
/// 1. Peel one layer with `project`.
/// 2. Recurse into every child with `cata` itself (via `map`).
/// 3. Hand the fully-reduced layer to the algebra `f`.
pub fn cata<A, F>(e: Expr, alg: &F) -> A
where
    F: Fn(ExprF<A>) -> A,
{
    alg(project(e).map(|child| cata(*child, alg)))
}

// ============================================================
// Algebras — pure logic, zero recursion
// ============================================================

/// Evaluate using cata: just describe the *local* computation.
pub fn eval_cata(e: Expr) -> i64 {
    cata(e, &|node| match node {
        ExprF::Lit(n) => n,
        ExprF::Add(a, b) => a + b,
        ExprF::Mul(a, b) => a * b,
    })
}

/// Pretty-print using cata.
pub fn show_cata(e: Expr) -> String {
    cata(e, &|node| match node {
        ExprF::Lit(n) => n.to_string(),
        ExprF::Add(a, b) => format!("({a} + {b})"),
        ExprF::Mul(a, b) => format!("({a} * {b})"),
    })
}

/// Depth using cata.
pub fn depth_cata(e: Expr) -> usize {
    cata(e, &|node: ExprF<usize>| match node {
        ExprF::Lit(_) => 0,
        ExprF::Add(a, b) | ExprF::Mul(a, b) => 1 + a.max(b),
    })
}

/// Count nodes using cata — free, no new traversal code needed.
pub fn count_nodes(e: Expr) -> usize {
    cata(e, &|node| match node {
        ExprF::Lit(_) => 1,
        ExprF::Add(a, b) | ExprF::Mul(a, b) => 1 + a + b,
    })
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// (2 + 3) * 4  =  20,  depth 2,  5 nodes
    fn sample() -> Expr {
        Expr::make_mul(Expr::make_add(Expr::lit(2), Expr::lit(3)), Expr::lit(4))
    }

    // --- Approach 1: direct ---

    #[test]
    fn test_eval_direct_literal() {
        assert_eq!(eval(&Expr::lit(7)), 7);
    }

    #[test]
    fn test_eval_direct_compound() {
        assert_eq!(eval(&sample()), 20);
    }

    #[test]
    fn test_show_direct() {
        assert_eq!(show(&sample()), "((2 + 3) * 4)");
    }

    #[test]
    fn test_depth_direct() {
        assert_eq!(depth(&Expr::lit(0)), 0);
        assert_eq!(depth(&sample()), 2);
    }

    // --- Approach 2: cata ---

    #[test]
    fn test_eval_cata_literal() {
        assert_eq!(eval_cata(Expr::lit(42)), 42);
    }

    #[test]
    fn test_eval_cata_compound() {
        assert_eq!(eval_cata(sample()), 20);
    }

    #[test]
    fn test_show_cata() {
        assert_eq!(show_cata(sample()), "((2 + 3) * 4)");
        assert_eq!(show_cata(Expr::lit(99)), "99");
    }

    #[test]
    fn test_depth_cata() {
        assert_eq!(depth_cata(Expr::lit(0)), 0);
        assert_eq!(depth_cata(sample()), 2);
        // Deeper tree: ((2+3)*4) + 1  →  depth 3
        let deeper = Expr::make_add(sample(), Expr::lit(1));
        assert_eq!(depth_cata(deeper), 3);
    }

    #[test]
    fn test_count_nodes() {
        // (2 + 3) * 4  →  mul, add, lit(2), lit(3), lit(4)  =  5
        assert_eq!(count_nodes(sample()), 5);
        assert_eq!(count_nodes(Expr::lit(0)), 1);
    }

    #[test]
    fn test_direct_and_cata_agree() {
        // Both approaches must produce identical results.
        let e_ref = sample();
        let e_owned = sample();
        assert_eq!(eval(&e_ref), eval_cata(e_owned));
    }

    #[test]
    fn test_nested_mul() {
        // 2 * (3 * 4)  =  24
        let e = Expr::make_mul(Expr::lit(2), Expr::make_mul(Expr::lit(3), Expr::lit(4)));
        assert_eq!(eval(&e), 24);
        assert_eq!(eval_cata(e.clone()), 24);
        assert_eq!(show(&e), "(2 * (3 * 4))");
        assert_eq!(show_cata(e), "(2 * (3 * 4))");
    }
}
