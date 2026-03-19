#![allow(clippy::all)]
/// Currying and Partial Application: OCaml's default vs Rust's closures.
///
/// In OCaml, ALL functions are curried by default. `let add a b = a + b` is
/// really `let add = fun a -> fun b -> a + b`. Rust doesn't curry automatically,
/// but closures achieve the same effect.

// ── Idiomatic Rust: closures for partial application ────────────────────────

/// Returns a closure that adds `n` to its argument.
/// This is the Rust equivalent of OCaml's `let add n = fun x -> x + n`
pub fn add(n: i64) -> impl Fn(i64) -> i64 {
    move |x| x + n
}

/// Returns a closure that multiplies by `n`
pub fn multiply_by(n: i64) -> impl Fn(i64) -> i64 {
    move |x| x * n
}

/// Generic partial application: fix the first argument of a two-arg function.
/// In OCaml this is free; in Rust we build it explicitly.
pub fn partial<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where
    F: Fn(A, B) -> C,
    A: Clone,
{
    move |b| f(a.clone(), b)
}

// ── Demonstrating curried-style APIs ────────────────────────────────────────

/// A curried comparison: returns a predicate
pub fn greater_than(threshold: i64) -> impl Fn(&i64) -> bool {
    move |x| *x > threshold
}

/// Apply a list of transformations (each is a curried function result)
pub fn apply_all(value: i64, transforms: &[Box<dyn Fn(i64) -> i64>]) -> i64 {
    transforms.iter().fold(value, |acc, f| f(acc))
}

// ── Functional style: simulating currying with nested closures ──────────────

/// Fully curried three-argument function
/// OCaml: `let f a b c = a + b + c` → `f 1` returns a function, `f 1 2` returns a function
pub fn curried_add3(a: i64) -> Box<dyn Fn(i64) -> Box<dyn Fn(i64) -> i64>> {
    Box::new(move |b| Box::new(move |c| a + b + c))
}

// ── Practical example: building predicates ──────────────────────────────────

/// Filter with a curried predicate builder
pub fn between(low: i64, high: i64) -> impl Fn(&i64) -> bool {
    move |x| *x >= low && *x <= high
}

pub fn filter_between(list: &[i64], low: i64, high: i64) -> Vec<i64> {
    list.iter().copied().filter(between(low, high)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_partial() {
        let add5 = add(5);
        assert_eq!(add5(3), 8);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_multiply_by() {
        let double = multiply_by(2);
        let triple = multiply_by(3);
        assert_eq!(double(7), 14);
        assert_eq!(triple(7), 21);
    }

    #[test]
    fn test_generic_partial() {
        let pow = |base: i64, exp: u32| base.pow(exp);
        let square = partial(pow, 2_i64); // partially apply base=2
                                          // Note: this gives 2^exp, not x^2
        assert_eq!(square(10), 1024); // 2^10
    }

    #[test]
    fn test_greater_than_as_predicate() {
        let data = vec![1, 5, 3, 8, 2, 9];
        let big: Vec<_> = data
            .iter()
            .filter(|x| greater_than(4)(x))
            .copied()
            .collect();
        assert_eq!(big, vec![5, 8, 9]);
    }

    #[test]
    fn test_curried_add3() {
        let f = curried_add3(1);
        let g = f(2);
        assert_eq!(g(3), 6);
        assert_eq!(curried_add3(10)(20)(30), 60);
    }

    #[test]
    fn test_filter_between() {
        assert_eq!(filter_between(&[1, 2, 3, 4, 5, 6], 2, 5), vec![2, 3, 4, 5]);
        assert_eq!(filter_between(&[], 0, 10), Vec::<i64>::new());
        assert_eq!(filter_between(&[100], 0, 10), Vec::<i64>::new());
    }

    #[test]
    fn test_apply_all_transforms() {
        let transforms: Vec<Box<dyn Fn(i64) -> i64>> = vec![
            Box::new(add(10)),
            Box::new(multiply_by(2)),
            Box::new(add(-1)),
        ];
        // 5 → +10 = 15 → *2 = 30 → -1 = 29
        assert_eq!(apply_all(5, &transforms), 29);
    }
}
