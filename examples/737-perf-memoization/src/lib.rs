//! # Memoization
//! Caching computed results

use std::collections::HashMap;
use std::hash::Hash;

/// Memoized function wrapper
pub struct Memoize<A, R, F> {
    cache: HashMap<A, R>,
    func: F,
}

impl<A: Eq + Hash + Clone, R: Clone, F: Fn(A) -> R> Memoize<A, R, F> {
    pub fn new(func: F) -> Self { Memoize { cache: HashMap::new(), func } }
    
    pub fn call(&mut self, arg: A) -> R {
        if let Some(result) = self.cache.get(&arg) {
            return result.clone();
        }
        let result = (self.func)(arg.clone());
        self.cache.insert(arg, result.clone());
        result
    }
}

/// Fibonacci with memoization
pub fn fib_memo(n: u64) -> u64 {
    fn fib_inner(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&v) = cache.get(&n) { return v; }
        let result = if n <= 1 { n } else { fib_inner(n - 1, cache) + fib_inner(n - 2, cache) };
        cache.insert(n, result);
        result
    }
    fib_inner(n, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memoize() {
        let mut m = Memoize::new(|x: i32| x * x);
        assert_eq!(m.call(5), 25);
        assert_eq!(m.call(5), 25); // cached
    }
    #[test]
    fn test_fib() { assert_eq!(fib_memo(10), 55); }
}
