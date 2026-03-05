//! # Trampoline Pattern
//!
//! Stack-safe recursion using trampolines to convert recursive calls to iteration.

/// Trampoline type - either done with a value or needs another step.
pub enum Bounce<T> {
    Done(T),
    More(Box<dyn FnOnce() -> Bounce<T>>),
}

/// Run a trampolined computation to completion.
pub fn run<T>(mut b: Bounce<T>) -> T {
    loop {
        match b {
            Bounce::Done(v) => return v,
            Bounce::More(th) => b = th(),
        }
    }
}

/// Stack-safe factorial using trampoline.
pub fn fact_t(n: u64, acc: u64) -> Bounce<u64> {
    if n == 0 {
        Bounce::Done(acc)
    } else {
        Bounce::More(Box::new(move || fact_t(n - 1, n * acc)))
    }
}

/// Factorial entry point.
pub fn factorial(n: u64) -> u64 {
    run(fact_t(n, 1))
}

/// Stack-safe even check using mutual recursion.
pub fn even_t(n: u64) -> Bounce<bool> {
    if n == 0 {
        Bounce::Done(true)
    } else {
        Bounce::More(Box::new(move || odd_t(n - 1)))
    }
}

/// Stack-safe odd check.
pub fn odd_t(n: u64) -> Bounce<bool> {
    if n == 0 {
        Bounce::Done(false)
    } else {
        Bounce::More(Box::new(move || even_t(n - 1)))
    }
}

/// Check if a number is even (stack-safe).
pub fn is_even(n: u64) -> bool {
    run(even_t(n))
}

/// Check if a number is odd (stack-safe).
pub fn is_odd(n: u64) -> bool {
    run(odd_t(n))
}

/// Stack-safe countdown.
pub fn count_t(n: u64) -> Bounce<u64> {
    if n == 0 {
        Bounce::Done(0)
    } else {
        Bounce::More(Box::new(move || count_t(n - 1)))
    }
}

/// Sum using trampoline.
pub fn sum_t(n: u64, acc: u64) -> Bounce<u64> {
    if n == 0 {
        Bounce::Done(acc)
    } else {
        Bounce::More(Box::new(move || sum_t(n - 1, acc + n)))
    }
}

pub fn sum_to(n: u64) -> u64 {
    run(sum_t(n, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(100));
        assert!(!is_even(101));
    }

    #[test]
    fn test_is_odd() {
        assert!(!is_odd(0));
        assert!(!is_odd(100));
        assert!(is_odd(101));
    }

    #[test]
    fn test_deep_recursion() {
        // This would stack overflow without trampoline
        assert_eq!(run(count_t(50_000)), 0);
    }

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(10), 55);
        assert_eq!(sum_to(100), 5050);
    }
}
