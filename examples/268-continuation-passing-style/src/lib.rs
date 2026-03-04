// Continuation-Passing Style (CPS) transforms recursive functions so the
// continuation (what to do next) is passed as a closure argument, making
// the recursive call structurally tail-recursive.
//
// In Rust, continuations are `Box<dyn FnOnce(T) -> R>` because each step
// wraps the previous continuation in a new closure (a heterogeneous chain
// that requires heap allocation and dynamic dispatch), and each continuation
// is consumed exactly once.

// ---------------------------------------------------------------------------
// Factorial — direct style (not tail-recursive)
// ---------------------------------------------------------------------------

/// Direct recursive factorial — mirrors the naive OCaml version.
/// Not tail-recursive: every call leaves a pending multiplication on the stack.
pub fn factorial_direct(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_direct(n - 1)
    }
}

// ---------------------------------------------------------------------------
// Factorial — CPS style
// ---------------------------------------------------------------------------

// Each recursive step wraps `k` in a new closure that performs the pending
// multiplication, so the recursive call itself is the last operation
// (structurally tail-recursive).
//
// Note: Rust does not guarantee tail-call optimisation, so the *call stack*
// still grows with `n`.  The continuation closures live on the heap, but the
// activation frames still accumulate.  For true stack safety at scale, use a
// trampoline or an iterative accumulator.
fn factorial_go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    if n == 0 {
        k(1)
    } else {
        factorial_go(n - 1, Box::new(move |result| k(n * result)))
    }
}

/// CPS factorial — structurally tail-recursive via continuation threading.
/// The identity closure `|x| x` is the initial "return the answer" continuation.
pub fn factorial_cps(n: u64) -> u64 {
    factorial_go(n, Box::new(|x| x))
}

// ---------------------------------------------------------------------------
// Binary tree — CPS sum
// ---------------------------------------------------------------------------

/// A binary tree whose leaves hold values of type `T`.
pub enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

// Internal CPS driver for tree summation.
// Traverses `t` depth-first; instead of waiting for sub-results on the stack,
// each branch continuation is threaded explicitly through closures.
fn sum_go<'a>(t: &'a Tree<i64>, k: Box<dyn FnOnce(i64) -> i64 + 'a>) -> i64 {
    match t {
        Tree::Leaf(x) => k(*x),
        Tree::Node(l, r) => sum_go(
            l,
            Box::new(move |sl| sum_go(r, Box::new(move |sr| k(sl + sr)))),
        ),
    }
}

/// CPS tree sum — sums all leaf values using continuation-passing traversal.
pub fn sum_cps(t: &Tree<i64>) -> i64 {
    sum_go(t, Box::new(|x| x))
}

// ---------------------------------------------------------------------------
// Convenience constructors
// ---------------------------------------------------------------------------

/// Shorthand for building a `Leaf` node.
pub fn leaf<T>(v: T) -> Tree<T> {
    Tree::Leaf(v)
}

/// Shorthand for building an inner `Node`.
pub fn node<T>(l: Tree<T>, r: Tree<T>) -> Tree<T> {
    Tree::Node(Box::new(l), Box::new(r))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- factorial_direct --

    #[test]
    fn test_direct_base_case() {
        assert_eq!(factorial_direct(0), 1);
    }

    #[test]
    fn test_direct_small() {
        assert_eq!(factorial_direct(1), 1);
        assert_eq!(factorial_direct(5), 120);
    }

    #[test]
    fn test_direct_ten() {
        assert_eq!(factorial_direct(10), 3_628_800);
    }

    // -- factorial_cps --

    #[test]
    fn test_cps_base_case() {
        assert_eq!(factorial_cps(0), 1);
    }

    #[test]
    fn test_cps_small() {
        assert_eq!(factorial_cps(1), 1);
        assert_eq!(factorial_cps(5), 120);
    }

    #[test]
    fn test_cps_ten() {
        assert_eq!(factorial_cps(10), 3_628_800);
    }

    #[test]
    fn test_cps_matches_direct() {
        for n in 0..=12 {
            assert_eq!(factorial_cps(n), factorial_direct(n));
        }
    }

    // -- sum_cps --

    #[test]
    fn test_sum_single_leaf() {
        let t = leaf(42_i64);
        assert_eq!(sum_cps(&t), 42);
    }

    #[test]
    fn test_sum_two_leaves() {
        let t = node(leaf(3_i64), leaf(7_i64));
        assert_eq!(sum_cps(&t), 10);
    }

    #[test]
    fn test_sum_ocaml_example() {
        // Node(Node(Leaf 1, Leaf 2), Node(Leaf 3, Leaf 4)) → 10
        let t = node(
            node(leaf(1_i64), leaf(2_i64)),
            node(leaf(3_i64), leaf(4_i64)),
        );
        assert_eq!(sum_cps(&t), 10);
    }

    #[test]
    fn test_sum_asymmetric_tree() {
        // Right-skewed: Node(Leaf 1, Node(Leaf 2, Leaf 3)) → 6
        let t = node(leaf(1_i64), node(leaf(2_i64), leaf(3_i64)));
        assert_eq!(sum_cps(&t), 6);
    }

    #[test]
    fn test_sum_negative_values() {
        let t = node(leaf(-5_i64), node(leaf(3_i64), leaf(-2_i64)));
        assert_eq!(sum_cps(&t), -4);
    }
}
