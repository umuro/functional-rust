//! Higher-Order Functions in Rust
//!
//! Functions that take other functions as arguments, return functions (closures),
//! and compose them — the foundation of functional-style programming.
//!
//! OCaml parallel: `apply`, `twice`, `compose`, `|>` operator.

// ── Passing functions as arguments ───────────────────────────────────────────

/// Apply a function to a value. Equivalent to OCaml's `let apply f x = f x`.
fn apply<A, B>(f: impl Fn(A) -> B, x: A) -> B {
    f(x)
}

/// Apply a function to a value twice. OCaml: `let twice f x = f (f x)`.
fn twice<A>(f: impl Fn(A) -> A, x: A) -> A {
    f(f(x))
}

// ── Returning functions (closures) ────────────────────────────────────────────

/// Return a closure that adds `n` to its argument.
fn adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Return a closure that multiplies its argument by `n`.
fn multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

// ── Function composition ──────────────────────────────────────────────────────

/// Compose two functions: `compose(f, g)(x)` = `f(g(x))`.
/// OCaml: `let compose f g x = f (g x)`.
fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

/// Pipe a value through a slice of `Box<dyn Fn(i32) -> i32>` transformations
/// (a poor-man's `|>` pipeline for homogeneous types).
fn pipe(value: i32, fns: &[&dyn Fn(i32) -> i32]) -> i32 {
    fns.iter().fold(value, |acc, f| f(acc))
}

// ── Iterator combinators (map / filter / fold) ────────────────────────────────

/// Double every number in a slice.
fn double_all(xs: &[i32]) -> Vec<i32> {
    xs.iter().map(|&x| x * 2).collect()
}

/// Keep only the even numbers from a slice.
fn keep_evens(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().filter(|x| x % 2 == 0).collect()
}

/// Sum all numbers in a slice using `fold`.
fn sum(xs: &[i32]) -> i32 {
    xs.iter().fold(0, |acc, &x| acc + x)
}

/// Generic higher-order transform: map, then filter, then fold — all in one pass.
fn map_filter_fold<A, B>(
    xs: impl Iterator<Item = A>,
    map_fn: impl Fn(A) -> B,
    filter_fn: impl Fn(&B) -> bool,
    init: B,
    fold_fn: impl Fn(B, B) -> B,
) -> B {
    xs.map(map_fn).filter(filter_fn).fold(init, fold_fn)
}

// ── Entry point ───────────────────────────────────────────────────────────────

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        assert_eq!(apply(|x: i32| x * 3, 7), 21);
        assert_eq!(apply(|s: &str| s.len(), "hello"), 5);
    }

    #[test]
    fn test_twice() {
        assert_eq!(twice(|x| x + 1, 5), 7);
        assert_eq!(twice(|x: i32| x * 2, 3), 12);
    }

    #[test]
    fn test_compose() {
        let f = compose(|x: i32| x * 2, |x: i32| x + 1);
        assert_eq!(f(5), 12); // (5+1)*2
        assert_eq!(f(0), 2); // (0+1)*2
    }

    #[test]
    fn test_adder_and_multiplier() {
        let add5 = adder(5);
        let times3 = multiplier(3);
        assert_eq!(add5(10), 15);
        assert_eq!(times3(4), 12);
    }

    #[test]
    fn test_pipe() {
        let inc = adder(1);
        let dbl = multiplier(2);
        assert_eq!(pipe(5, &[&inc, &dbl]), 12);
        assert_eq!(pipe(0, &[&inc, &dbl]), 2);
        assert_eq!(pipe(5, &[]), 5); // identity
    }

    #[test]
    fn test_iterator_combinators() {
        let xs = vec![1, 2, 3, 4, 5];
        assert_eq!(double_all(&xs), vec![2, 4, 6, 8, 10]);
        assert_eq!(keep_evens(&xs), vec![2, 4]);
        assert_eq!(sum(&xs), 15);
    }

    #[test]
    fn test_map_filter_fold() {
        // square [1..5], keep those > 10, sum → 16+25 = 41
        let result = map_filter_fold(1..=5, |x| x * x, |x| x > &10, 0, |acc, x| acc + x);
        assert_eq!(result, 41);
    }
}
