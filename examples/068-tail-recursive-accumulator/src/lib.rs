#![allow(dead_code)]
// 068: Tail-Recursive Accumulator
// Transform naive recursion into tail-recursive form by carrying an accumulator.
// Rust does NOT guarantee TCO — `iter().fold()` and explicit loops are the
// idiomatic replacement for accumulator recursion on large inputs.

// --- Sum ---

// Naive: the `+` happens AFTER the recursive call returns, so the call is not
// in tail position. Each frame stays on the stack.
fn sum_naive(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        [x, rest @ ..] => *x + sum_naive(rest),
    }
}

// Tail-recursive: the recursive call is the last operation; the accumulator
// carries the running total forward. Matches the OCaml `aux acc lst` idiom.
fn sum_tail(v: &[i32]) -> i32 {
    fn aux(acc: i32, v: &[i32]) -> i32 {
        match v {
            [] => acc,
            [x, rest @ ..] => aux(acc + *x, rest),
        }
    }
    aux(0, v)
}

// Idiomatic Rust: `.sum()` is the accumulator pattern compiled to a loop —
// stack-safe for any input size.
fn sum_fold(v: &[i32]) -> i32 {
    v.iter().sum()
}

// --- Factorial ---

fn fact_naive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * fact_naive(n - 1)
    }
}

fn fact_tail(n: u64) -> u64 {
    fn aux(acc: u64, n: u64) -> u64 {
        if n <= 1 {
            acc
        } else {
            aux(acc * n, n - 1)
        }
    }
    aux(1, n)
}

fn fact_fold(n: u64) -> u64 {
    (1..=n).product()
}

// --- Fibonacci ---

fn fib_naive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib_naive(n - 1) + fib_naive(n - 2)
    }
}

// Accumulator recursion: `a` is the current Fibonacci number, `b` is the next.
// Each step shifts the pair forward — O(n) time vs exponential for the naive form.
fn fib_tail(n: u64) -> u64 {
    fn aux(a: u64, b: u64, n: u64) -> u64 {
        if n == 0 {
            a
        } else {
            aux(b, a + b, n - 1)
        }
    }
    aux(0, 1, n)
}

fn fib_fold(n: u64) -> u64 {
    (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum_naive(&[]), 0);
        assert_eq!(sum_tail(&[]), 0);
        assert_eq!(sum_fold(&[]), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum_naive(&[42]), 42);
        assert_eq!(sum_tail(&[42]), 42);
        assert_eq!(sum_fold(&[42]), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum_naive(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_tail(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_fold(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(fact_naive(0), 1);
        assert_eq!(fact_tail(0), 1);
        assert_eq!(fact_naive(5), 120);
        assert_eq!(fact_tail(5), 120);
        assert_eq!(fact_fold(5), 120);
        assert_eq!(fact_fold(10), 3_628_800);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fib_tail(0), 0);
        assert_eq!(fib_tail(1), 1);
        assert_eq!(fib_tail(10), 55);
        assert_eq!(fib_fold(10), 55);
        assert_eq!(fib_naive(10), 55);
        assert_eq!(fib_tail(50), 12_586_269_025);
    }

    #[test]
    fn test_large_input_fold_is_stack_safe() {
        // `sum_fold` handles 100,000 elements — iterators compile to loops.
        // `sum_tail` would overflow the stack here because Rust does not
        // guarantee tail-call optimisation.
        let large: Vec<i32> = vec![1; 100_000];
        assert_eq!(sum_fold(&large), 100_000);
    }
}
