#![allow(clippy::all)]
//! # Continuation-Passing Style (CPS)
//!
//! Transform direct-style functions to pass results to continuations.

/// Factorial in direct style (for comparison).
pub fn fact(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

/// Factorial in CPS - result passed to continuation k.
pub fn fact_k<R: 'static>(n: u64, k: Box<dyn FnOnce(u64) -> R>) -> R {
    if n <= 1 {
        k(1)
    } else {
        fact_k(n - 1, Box::new(move |r| k(n * r)))
    }
}

/// Fibonacci in direct style.
pub fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// Fibonacci in CPS (requires boxed closure for recursion).
pub fn fib_k<R: 'static>(n: u64, k: Box<dyn FnOnce(u64) -> R>) -> R {
    if n <= 1 {
        k(n)
    } else {
        fib_k(
            n - 1,
            Box::new(move |r1| fib_k(n - 2, Box::new(move |r2| k(r1 + r2)))),
        )
    }
}

/// Map in CPS style.
pub fn map_k<T, U, R>(items: Vec<T>, f: impl Fn(T) -> U + Clone, k: impl FnOnce(Vec<U>) -> R) -> R {
    fn go<T, U, R>(
        mut items: Vec<T>,
        f: impl Fn(T) -> U + Clone,
        mut acc: Vec<U>,
        k: impl FnOnce(Vec<U>) -> R,
    ) -> R {
        if items.is_empty() {
            k(acc)
        } else {
            let head = items.remove(0);
            let u = f(head);
            acc.push(u);
            go(items, f, acc, k)
        }
    }
    go(items, f, Vec::new(), k)
}

/// Fold in CPS style.
pub fn fold_k<T, A, R>(
    items: Vec<T>,
    init: A,
    f: impl Fn(A, T) -> A + Clone,
    k: impl FnOnce(A) -> R,
) -> R {
    fn go<T, A, R>(
        mut items: Vec<T>,
        acc: A,
        f: impl Fn(A, T) -> A + Clone,
        k: impl FnOnce(A) -> R,
    ) -> R {
        if items.is_empty() {
            k(acc)
        } else {
            let head = items.remove(0);
            let new_acc = f(acc, head);
            go(items, new_acc, f, k)
        }
    }
    go(items, init, f, k)
}

/// Safe division with success and error continuations.
pub fn safe_div_k<R>(a: f64, b: f64, ok: impl FnOnce(f64) -> R, err: impl FnOnce(&str) -> R) -> R {
    if b == 0.0 {
        err("division by zero")
    } else {
        ok(a / b)
    }
}

/// Parse integer with continuation for success/failure.
pub fn parse_int_k<R>(s: &str, ok: impl FnOnce(i64) -> R, err: impl FnOnce(&str) -> R) -> R {
    match s.parse::<i64>() {
        Ok(n) => ok(n),
        Err(_) => err(s),
    }
}

/// Chained CPS operations.
pub fn chain_example<R>(s: &str, k: impl FnOnce(f64) -> R, err: impl FnOnce(&str) -> R) -> R {
    match s.parse::<i64>() {
        Ok(n) => safe_div_k(100.0, n as f64, k, err),
        Err(_) => err(s),
    }
}

/// Identity continuation - extract value from CPS.
pub fn run<T>(f: impl FnOnce(Box<dyn FnOnce(T) -> T>) -> T) -> T {
    f(Box::new(|x| x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_direct() {
        assert_eq!(fact(5), 120);
        assert_eq!(fact(10), 3_628_800);
    }

    #[test]
    fn test_fact_cps() {
        fact_k(5, Box::new(|n| assert_eq!(n, 120)));
        fact_k(10, Box::new(|n| assert_eq!(n, 3_628_800)));
    }

    #[test]
    fn test_fact_equivalence() {
        for n in 0..=12 {
            let direct = fact(n);
            fact_k(n, Box::new(move |cps| assert_eq!(cps, direct)));
        }
    }

    #[test]
    fn test_fib_direct() {
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn test_fib_cps() {
        fib_k(10, Box::new(|n| assert_eq!(n, 55)));
    }

    #[test]
    fn test_map_k() {
        map_k(
            vec![1, 2, 3],
            |x| x * 2,
            |result| {
                assert_eq!(result, vec![2, 4, 6]);
            },
        );
    }

    #[test]
    fn test_map_k_empty() {
        map_k(
            Vec::<i32>::new(),
            |x| x * 2,
            |result| {
                assert!(result.is_empty());
            },
        );
    }

    #[test]
    fn test_fold_k() {
        fold_k(
            vec![1, 2, 3, 4],
            0,
            |acc, x| acc + x,
            |sum| {
                assert_eq!(sum, 10);
            },
        );
    }

    #[test]
    fn test_safe_div_ok() {
        safe_div_k(
            10.0,
            2.0,
            |r| assert_eq!(r, 5.0),
            |_| panic!("unexpected error"),
        );
    }

    #[test]
    fn test_safe_div_err() {
        let mut got_error = false;
        safe_div_k(
            10.0,
            0.0,
            |_| panic!("unexpected success"),
            |_| got_error = true,
        );
        assert!(got_error);
    }

    #[test]
    fn test_parse_int_ok() {
        parse_int_k("42", |n| assert_eq!(n, 42), |_| panic!("unexpected error"));
    }

    #[test]
    fn test_parse_int_err() {
        let mut got_error = false;
        parse_int_k(
            "abc",
            |_| panic!("unexpected success"),
            |_| got_error = true,
        );
        assert!(got_error);
    }

    #[test]
    fn test_chain() {
        // "10" -> parse to 10 -> 100/10 = 10.0
        chain_example(
            "10",
            |r| assert_eq!(r, 10.0),
            |_| panic!("unexpected error"),
        );
    }

    #[test]
    fn test_chain_div_zero() {
        let mut got_error = false;
        chain_example("0", |_| panic!("unexpected success"), |_| got_error = true);
        assert!(got_error);
    }

    #[test]
    fn test_chain_parse_error() {
        let mut got_error = false;
        chain_example(
            "abc",
            |_| panic!("unexpected success"),
            |_| got_error = true,
        );
        assert!(got_error);
    }
}
