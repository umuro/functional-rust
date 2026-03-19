//! # CPS — Continuation-Passing Style
//!
//! Transform recursive functions to pass "what to do next" as a function argument.
//! This makes them tail-recursive in OCaml; in Rust, closures still allocate on heap.

// ---------------------------------------------------------------------------
// Approach A: Direct recursion (not tail-recursive)
// ---------------------------------------------------------------------------

pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// ---------------------------------------------------------------------------
// Approach B: CPS with boxed closures
// ---------------------------------------------------------------------------

pub fn factorial_cps(n: u64) -> u64 {
    fn go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
        if n == 0 {
            k(1)
        } else {
            go(n - 1, Box::new(move |result| k(n * result)))
        }
    }
    go(n, Box::new(|x| x))
}

// ---------------------------------------------------------------------------
// Approach C: Iterative (idiomatic Rust — no CPS needed)
// ---------------------------------------------------------------------------

pub fn factorial_iter(n: u64) -> u64 {
    (1..=n).product()
}

// ---------------------------------------------------------------------------
// Tree sum — CPS vs direct
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum Tree {
    Leaf(i64),
    Node(Box<Tree>, Box<Tree>),
}

pub fn tree_sum(t: &Tree) -> i64 {
    match t {
        Tree::Leaf(x) => *x,
        Tree::Node(l, r) => tree_sum(l) + tree_sum(r),
    }
}

pub fn tree_sum_cps(t: &Tree) -> i64 {
    fn go(t: &Tree, k: Box<dyn FnOnce(i64) -> i64 + '_>) -> i64 {
        match t {
            Tree::Leaf(x) => k(*x),
            Tree::Node(l, r) => go(l, Box::new(move |sl| go(r, Box::new(move |sr| k(sl + sr))))),
        }
    }
    go(t, Box::new(|x| x))
}

/// Stack-based tree sum — truly iterative, no recursion
pub fn tree_sum_stack(t: &Tree) -> i64 {
    let mut stack = vec![t];
    let mut sum = 0;
    while let Some(node) = stack.pop() {
        match node {
            Tree::Leaf(x) => sum += x,
            Tree::Node(l, r) => {
                stack.push(l);
                stack.push(r);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial_cps(10), 3628800);
        assert_eq!(factorial_iter(10), 3628800);
    }

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial_cps(0), 1);
    }

    #[test]
    fn test_tree_sum() {
        let t = Tree::Node(
            Box::new(Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
            Box::new(Tree::Node(Box::new(Tree::Leaf(3)), Box::new(Tree::Leaf(4)))),
        );
        assert_eq!(tree_sum(&t), 10);
        assert_eq!(tree_sum_cps(&t), 10);
        assert_eq!(tree_sum_stack(&t), 10);
    }

    #[test]
    fn test_tree_leaf() {
        let t = Tree::Leaf(42);
        assert_eq!(tree_sum(&t), 42);
        assert_eq!(tree_sum_cps(&t), 42);
    }

    #[test]
    fn test_factorial_large() {
        assert_eq!(factorial_iter(20), 2432902008176640000);
    }
}
