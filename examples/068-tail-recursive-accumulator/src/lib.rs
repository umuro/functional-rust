// 068: Tail-Recursive Accumulator
// Rust doesn't guarantee TCO — use loops or fold instead

// Approach 1: Recursive vs loop-based sum
fn sum_recursive(v: &[i32]) -> i32 {
    if v.is_empty() { 0 } else { v[0] + sum_recursive(&v[1..]) }
}

fn sum_loop(v: &[i32]) -> i32 {
    let mut acc = 0;
    for &x in v { acc += x; }
    acc
}

fn sum_fold(v: &[i32]) -> i32 {
    v.iter().fold(0, |acc, &x| acc + x)
}

// Approach 2: Factorial
fn fact_recursive(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * fact_recursive(n - 1) }
}

fn fact_loop(n: u64) -> u64 {
    let mut acc = 1u64;
    let mut i = n;
    while i > 1 {
        acc *= i;
        i -= 1;
    }
    acc
}

fn fact_fold(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

// Approach 3: Fibonacci with accumulator loop
fn fib_loop(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    a
}

fn fib_fold(n: u64) -> u64 {
    (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b)).0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum_recursive(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_loop(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_fold(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_fold(&[]), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(fact_recursive(5), 120);
        assert_eq!(fact_loop(5), 120);
        assert_eq!(fact_fold(5), 120);
        assert_eq!(fact_fold(0), 1);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fib_loop(0), 0);
        assert_eq!(fib_loop(1), 1);
        assert_eq!(fib_loop(10), 55);
        assert_eq!(fib_fold(10), 55);
    }

    #[test]
    fn test_large_input() {
        let large: Vec<i32> = vec![1; 100_000];
        assert_eq!(sum_loop(&large), 100_000);
    }
}
