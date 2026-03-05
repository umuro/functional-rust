//! # Closure Memoization — Caching Results

use std::collections::HashMap;
use std::hash::Hash;

/// Simple memoization wrapper
pub struct Memoize<F, A, R>
where
    A: Eq + Hash + Clone,
    R: Clone,
{
    func: F,
    cache: HashMap<A, R>,
}

impl<F, A, R> Memoize<F, A, R>
where
    F: Fn(A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            cache: HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: A) -> R {
        if let Some(result) = self.cache.get(&arg) {
            return result.clone();
        }
        let result = (self.func)(arg.clone());
        self.cache.insert(arg, result.clone());
        result
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// Recursive memoization (e.g., fibonacci)
pub fn memoized_fib() -> impl FnMut(u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    
    fn fib_inner(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }
        let result = if n <= 1 {
            n
        } else {
            fib_inner(n - 1, cache) + fib_inner(n - 2, cache)
        };
        cache.insert(n, result);
        result
    }
    
    move |n| fib_inner(n, &mut cache)
}

/// Once-computed lazy value
pub struct Lazy<T, F: FnOnce() -> T> {
    value: Option<T>,
    init: Option<F>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self {
        Self {
            value: None,
            init: Some(init),
        }
    }

    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().expect("already initialized");
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoize() {
        let mut memo = Memoize::new(|x: i32| {
            x * x
        });

        assert_eq!(memo.call(5), 25);
        assert_eq!(memo.call(5), 25); // Cached
        assert_eq!(memo.call(6), 36);
        assert_eq!(memo.cache_size(), 2);
    }

    #[test]
    fn test_memoized_fib() {
        let mut fib = memoized_fib();
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(40), 102334155);
    }

    #[test]
    fn test_lazy() {
        let mut computed = false;
        let mut lazy = Lazy::new(|| {
            computed = true;
            42
        });

        assert!(!computed);
        assert_eq!(*lazy.get(), 42);
        assert!(computed);
        assert_eq!(*lazy.get(), 42); // Doesn't recompute
    }

    #[test]
    fn test_expensive_computation() {
        let mut memo = Memoize::new(|s: String| {
            s.chars().map(|c| c as u32).sum::<u32>()
        });

        let result1 = memo.call("hello".to_string());
        let result2 = memo.call("hello".to_string());
        assert_eq!(result1, result2);
    }
}
