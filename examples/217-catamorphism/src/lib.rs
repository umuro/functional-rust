#![allow(clippy::all)]
//! # Example 217: Catamorphism — The Universal Fold
//!
//! A catamorphism (`cata`) is the single function that encodes *all* bottom-up
//! traversals of a recursive structure. You write **one algebra** — a plain function
//! that handles one node, no recursion — and `cata` takes care of the rest.
//!
//! This builds on the Fix-point idea (example 216) but uses a richer expression
//! language (`ExprF` with five variants including `NegF` and `IfZeroF`) to show
//! that adding new operations costs only a new algebra, never a new traversal.
//!
//! ## Three-step recipe
//!
//! 1. **Base functor** `ExprF<A>` — the shape of one node, children replaced by `A`.
//! 2. **Fix wrapper** `Fix` — ties the knot: `Fix ≅ ExprF<Fix>`.
//! 3. **`cata`** — the only function that recurses; everything else is an algebra.

// ============================================================
// Step 1: Base functor — five variants, children are type `A`
// ============================================================

/// One layer of an arithmetic expression.
///
/// `A` is the type of child positions; `LitF` carries no children.
#[derive(Debug, Clone)]
pub enum ExprF<A> {
    LitF(i64),
    AddF(A, A),
    MulF(A, A),
    NegF(A),
    /// `IfZeroF(cond, then_branch, else_branch)`:
    /// evaluates to `then_branch` when `cond == 0`, else `else_branch`.
    IfZeroF(A, A, A),
}

impl<A> ExprF<A> {
    /// Functorial map — apply `f` to every child position.
    ///
    /// Leaves (`LitF`) pass through unchanged; all recursive slots get transformed.
    pub fn map<B, F: FnMut(A) -> B>(self, mut f: F) -> ExprF<B> {
        match self {
            ExprF::LitF(n) => ExprF::LitF(n),
            ExprF::AddF(a, b) => ExprF::AddF(f(a), f(b)),
            ExprF::MulF(a, b) => ExprF::MulF(f(a), f(b)),
            ExprF::NegF(a) => ExprF::NegF(f(a)),
            ExprF::IfZeroF(c, t, e) => ExprF::IfZeroF(f(c), f(t), f(e)),
        }
    }
}

// ============================================================
// Step 2: Fix wrapper
// ============================================================

/// `Fix` ties the recursive knot: `Fix ≅ ExprF<Fix>`.
///
/// A `Fix` value is a fully recursive expression tree.
#[derive(Debug, Clone)]
pub struct Fix(Box<ExprF<Fix>>);

impl Fix {
    /// Wrap a layer — inject one `ExprF` node into the fixed point.
    pub fn wrap(layer: ExprF<Fix>) -> Self {
        Fix(Box::new(layer))
    }

    /// Unwrap one layer, consuming `self`.
    pub fn unfix(self) -> ExprF<Fix> {
        *self.0
    }

    // ---- Smart constructors (convenience) ----

    pub fn lit(n: i64) -> Self {
        Fix::wrap(ExprF::LitF(n))
    }

    pub fn make_add(a: Fix, b: Fix) -> Self {
        Fix::wrap(ExprF::AddF(a, b))
    }

    pub fn make_mul(a: Fix, b: Fix) -> Self {
        Fix::wrap(ExprF::MulF(a, b))
    }

    pub fn make_neg(a: Fix) -> Self {
        Fix::wrap(ExprF::NegF(a))
    }

    pub fn if_zero(cond: Fix, then_branch: Fix, else_branch: Fix) -> Self {
        Fix::wrap(ExprF::IfZeroF(cond, then_branch, else_branch))
    }
}

// ============================================================
// Step 3: cata — the one and only recursive function
// ============================================================

/// Catamorphism: fold an expression tree bottom-up using algebra `alg`.
///
/// `alg` is called once per node *after* all children have already been
/// reduced.  `alg` never recurses — `cata` handles that entirely.
///
/// ```text
/// cata alg (Fix layer) = alg (map (cata alg) layer)
/// ```
pub fn cata<A, F>(expr: Fix, alg: &F) -> A
where
    F: Fn(ExprF<A>) -> A,
{
    alg(expr.unfix().map(|child| cata(child, alg)))
}

// ============================================================
// Algebras — zero recursion, one concern each
// ============================================================

/// Evaluate an expression to an `i64`.
///
/// This algebra handles only the *local* arithmetic step; recursion is in `cata`.
pub fn eval(expr: Fix) -> i64 {
    cata(expr, &|node: ExprF<i64>| match node {
        ExprF::LitF(n) => n,
        ExprF::AddF(a, b) => a + b,
        ExprF::MulF(a, b) => a * b,
        ExprF::NegF(a) => -a,
        ExprF::IfZeroF(c, t, e) => {
            if c == 0 {
                t
            } else {
                e
            }
        }
    })
}

/// Pretty-print an expression as a `String`.
pub fn show(expr: Fix) -> String {
    cata(expr, &|node| match node {
        ExprF::LitF(n) => n.to_string(),
        ExprF::AddF(a, b) => format!("({a} + {b})"),
        ExprF::MulF(a, b) => format!("({a} * {b})"),
        ExprF::NegF(a) => format!("(-{a})"),
        ExprF::IfZeroF(c, t, e) => format!("(ifz {c} then {t} else {e})"),
    })
}

/// Count the total number of nodes in the expression tree.
pub fn count_nodes(expr: Fix) -> usize {
    cata(expr, &|node| match node {
        ExprF::LitF(_) => 1,
        ExprF::AddF(a, b) | ExprF::MulF(a, b) => 1 + a + b,
        ExprF::NegF(a) => 1 + a,
        ExprF::IfZeroF(c, t, e) => 1 + c + t + e,
    })
}

/// Collect all literal values in left-to-right order.
pub fn collect_lits(expr: Fix) -> Vec<i64> {
    cata(expr, &|node: ExprF<Vec<i64>>| match node {
        ExprF::LitF(n) => vec![n],
        ExprF::AddF(mut a, b) | ExprF::MulF(mut a, b) => {
            a.extend(b);
            a
        }
        ExprF::NegF(a) => a,
        ExprF::IfZeroF(mut c, t, e) => {
            c.extend(t);
            c.extend(e);
            c
        }
    })
}

/// Compute the maximum depth of the expression tree.
pub fn depth(expr: Fix) -> usize {
    cata(expr, &|node: ExprF<usize>| match node {
        ExprF::LitF(_) => 0,
        ExprF::AddF(a, b) | ExprF::MulF(a, b) => 1 + a.max(b),
        ExprF::NegF(a) => 1 + a,
        ExprF::IfZeroF(c, t, e) => 1 + c.max(t).max(e),
    })
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Helpers ----

    /// `(2 + 3) * (-4)` — evaluates to -20
    fn sample() -> Fix {
        Fix::make_mul(
            Fix::make_add(Fix::lit(2), Fix::lit(3)),
            Fix::make_neg(Fix::lit(4)),
        )
    }

    /// `ifz 0 then 10 else 99` — evaluates to 10
    fn if_zero_true() -> Fix {
        Fix::if_zero(Fix::lit(0), Fix::lit(10), Fix::lit(99))
    }

    /// `ifz 1 then 10 else 99` — evaluates to 99
    fn if_zero_false() -> Fix {
        Fix::if_zero(Fix::lit(1), Fix::lit(10), Fix::lit(99))
    }

    // ---- eval ----

    #[test]
    fn test_eval_literal() {
        assert_eq!(eval(Fix::lit(42)), 42);
    }

    #[test]
    fn test_eval_neg() {
        assert_eq!(eval(Fix::make_neg(Fix::lit(7))), -7);
    }

    #[test]
    fn test_eval_sample() {
        // (2 + 3) * (-4) = 5 * -4 = -20
        assert_eq!(eval(sample()), -20);
    }

    #[test]
    fn test_eval_if_zero_taken() {
        assert_eq!(eval(if_zero_true()), 10);
    }

    #[test]
    fn test_eval_if_zero_not_taken() {
        assert_eq!(eval(if_zero_false()), 99);
    }

    #[test]
    fn test_eval_nested_if_zero() {
        // ifz (1 - 1) then 5 else 6  →  5  (since 1-1 = 0)
        let cond = Fix::make_add(Fix::lit(1), Fix::make_neg(Fix::lit(1)));
        let e = Fix::if_zero(cond, Fix::lit(5), Fix::lit(6));
        assert_eq!(eval(e), 5);
    }

    // ---- show ----

    #[test]
    fn test_show_literal() {
        assert_eq!(show(Fix::lit(3)), "3");
    }

    #[test]
    fn test_show_neg() {
        assert_eq!(show(Fix::make_neg(Fix::lit(5))), "(-5)");
    }

    #[test]
    fn test_show_sample() {
        assert_eq!(show(sample()), "((2 + 3) * (-4))");
    }

    #[test]
    fn test_show_if_zero() {
        assert_eq!(show(if_zero_true()), "(ifz 0 then 10 else 99)");
    }

    // ---- count_nodes ----

    #[test]
    fn test_count_nodes_literal() {
        assert_eq!(count_nodes(Fix::lit(0)), 1);
    }

    #[test]
    fn test_count_nodes_sample() {
        // mul, add, lit(2), lit(3), neg, lit(4) = 6
        assert_eq!(count_nodes(sample()), 6);
    }

    #[test]
    fn test_count_nodes_if_zero() {
        // ifz, lit(0), lit(10), lit(99) = 4
        assert_eq!(count_nodes(if_zero_true()), 4);
    }

    // ---- collect_lits ----

    #[test]
    fn test_collect_lits_simple() {
        assert_eq!(collect_lits(Fix::lit(7)), vec![7]);
    }

    #[test]
    fn test_collect_lits_sample() {
        // (2 + 3) * (-4) → left-to-right: [2, 3, 4]
        assert_eq!(collect_lits(sample()), vec![2, 3, 4]);
    }

    #[test]
    fn test_collect_lits_if_zero() {
        assert_eq!(collect_lits(if_zero_true()), vec![0, 10, 99]);
    }

    // ---- depth ----

    #[test]
    fn test_depth_literal() {
        assert_eq!(depth(Fix::lit(0)), 0);
    }

    #[test]
    fn test_depth_sample() {
        // mul(add(lit,lit), neg(lit)) → depth 2
        assert_eq!(depth(sample()), 2);
    }

    #[test]
    fn test_depth_if_zero() {
        // ifz(lit, lit, lit) → depth 1
        assert_eq!(depth(if_zero_true()), 1);
    }

    // ---- algebra independence ----

    #[test]
    fn test_two_algebras_independent() {
        // The same tree structure with different algebras yields different results.
        let e1 = sample();
        let e2 = sample();
        assert_eq!(eval(e1), -20);
        assert_eq!(show(e2), "((2 + 3) * (-4))");
    }

    #[test]
    fn test_custom_algebra_via_cata() {
        // Inline algebra: count how many negative literals appear in a tree.
        // (This would be awkward to write as a hand-rolled recursion.)
        let tree = Fix::make_add(
            Fix::make_neg(Fix::lit(-3)), // neg of a negative — still 1 negative lit
            Fix::make_mul(Fix::lit(5), Fix::make_neg(Fix::lit(-1))),
        );
        let neg_count: usize = cata(tree, &|node| match node {
            ExprF::LitF(n) => usize::from(n < 0),
            ExprF::AddF(a, b) | ExprF::MulF(a, b) => a + b,
            ExprF::NegF(a) => a,
            ExprF::IfZeroF(c, t, e) => c + t + e,
        });
        // -3 and -1 are negative literals → count = 2
        assert_eq!(neg_count, 2);
    }
}
