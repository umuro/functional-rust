/// Tail-Recursive Accumulator Pattern
///
/// OCaml relies on tail-call optimization (TCO) for stack safety.
/// Rust does NOT guarantee TCO, so idiomatic Rust uses iterators
/// or explicit loops. We show both styles for comparison.

/// Naive recursive sum — would blow the stack on large inputs in both languages.
pub fn sum_naive(list: &[i64]) -> i64 {
    match list {
        [] => 0,
        [h, rest @ ..] => h + sum_naive(rest),
    }
}

/// Tail-recursive style (Rust doesn't guarantee TCO, but the pattern is instructive).
pub fn sum_tr(list: &[i64]) -> i64 {
    fn go(acc: i64, slice: &[i64]) -> i64 {
        match slice {
            [] => acc,
            [h, rest @ ..] => go(acc + h, rest),
        }
    }
    go(0, list)
}

/// Idiomatic Rust: use iterators. This is the preferred approach.
pub fn sum_iter(list: &[i64]) -> i64 {
    list.iter().sum()
}

/// Tail-recursive reverse — process from front, collect into Vec in reverse.
/// OCaml's `h :: acc` prepends; Rust's Vec::insert(0, h) is O(n) per call.
/// We use a VecDeque-style push_front or simply iterate backwards.
pub fn rev_tr<T: Clone>(list: &[T]) -> Vec<T> {
    // Functional accumulator style: fold from left, building reversed result
    list.iter().rev().cloned().collect()
}

/// Explicit recursive version mirroring OCaml's pattern.
pub fn rev_recursive<T: Clone>(list: &[T]) -> Vec<T> {
    fn go<T: Clone>(acc: &mut Vec<T>, slice: &[T]) {
        match slice {
            [] => {}
            [h, rest @ ..] => {
                // In OCaml: h :: acc (prepend). In Rust we build reversed by
                // inserting at front — but that's O(n). Instead we push and
                // the recursion order naturally reverses.
                go(acc, rest);
                acc.push(h.clone());
            }
        }
    }
    let mut result = Vec::new();
    go(&mut result, list);
    result
}

/// Idiomatic Rust reverse.
pub fn rev_iter<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().rev().cloned().collect()
}

/// Tail-recursive Fibonacci with accumulator.
pub fn fib_tr(n: u64) -> u64 {
    fn go(a: u64, b: u64, n: u64) -> u64 {
        match n {
            0 => a,
            n => go(b, a + b, n - 1),
        }
    }
    go(0, 1, n)
}

/// Iterative Fibonacci — idiomatic Rust.
pub fn fib_iter(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let big: Vec<i64> = (1..=100_000).collect();
        let expected: i64 = 100_000 * 100_001 / 2;
        assert_eq!(sum_tr(&big), expected);
        assert_eq!(sum_iter(&big), expected);
    }

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum_tr(&[]), 0);
        assert_eq!(sum_iter(&[]), 0);
    }

    #[test]
    fn test_rev() {
        assert_eq!(rev_tr(&[1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(rev_recursive(&[1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(rev_tr::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_rev_iter() {
        assert_eq!(rev_iter(&[1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(rev_iter::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib_tr(0), 0);
        assert_eq!(fib_tr(1), 1);
        assert_eq!(fib_tr(10), 55);
        assert_eq!(fib_tr(40), 102334155);
    }

    #[test]
    fn test_fib_iter() {
        assert_eq!(fib_iter(10), 55);
        assert_eq!(fib_iter(40), 102334155);
    }
}

fn main() {
    println!("{:?}", sum_tr(&big), expected);
    println!("{:?}", sum_iter(&big), expected);
    println!("{:?}", sum_tr(&[]), 0);
}
