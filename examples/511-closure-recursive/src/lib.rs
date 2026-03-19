#![allow(clippy::all)]
//! Recursive Closures (Y Combinator)
//!
//! Techniques for self-referential closures and the Y combinator in Rust.

/// Approach 1: Named recursive function (simplest — always prefer this)
pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Approach 2: Open recursion via higher-order function.
/// The step function receives itself as an argument.
pub fn factorial_open<F>(step: F, n: u64) -> u64
where
    F: Fn(&dyn Fn(u64) -> u64, u64) -> u64,
{
    fn apply<F>(step: &F, n: u64) -> u64
    where
        F: Fn(&dyn Fn(u64) -> u64, u64) -> u64,
    {
        step(&|m| apply(step, m), n)
    }
    apply(&step, n)
}

/// Approach 3: Y combinator using Box<dyn Fn>
pub struct Y<A, B>(pub Box<dyn Fn(&Y<A, B>, A) -> B>);

impl<A, B> Y<A, B> {
    pub fn call(&self, arg: A) -> B {
        (self.0)(self, arg)
    }
}

/// Create a Y combinator for factorial.
pub fn y_factorial() -> Y<u64, u64> {
    Y(Box::new(|y, n| if n <= 1 { 1 } else { n * y.call(n - 1) }))
}

/// Fibonacci using open recursion.
pub fn fib_open(n: u64) -> u64 {
    let step = |self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 {
            n
        } else {
            self_(n - 1) + self_(n - 2)
        }
    };
    factorial_open(step, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_named() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_factorial_open() {
        let step = |self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            if n <= 1 {
                1
            } else {
                n * self_(n - 1)
            }
        };
        assert_eq!(factorial_open(step, 6), 720);
    }

    #[test]
    fn test_y_factorial() {
        let fact = y_factorial();
        assert_eq!(fact.call(0), 1);
        assert_eq!(fact.call(5), 120);
        assert_eq!(fact.call(6), 720);
    }

    #[test]
    fn test_fib_open() {
        assert_eq!(fib_open(0), 0);
        assert_eq!(fib_open(1), 1);
        assert_eq!(fib_open(10), 55);
    }

    #[test]
    fn test_y_combinator_structure() {
        // Test that Y combinator works for custom function
        let double_until_100 = Y(Box::new(
            |y: &Y<i32, i32>, n: i32| {
                if n >= 100 {
                    n
                } else {
                    y.call(n * 2)
                }
            },
        ));
        assert_eq!(double_until_100.call(1), 128);
    }
}
