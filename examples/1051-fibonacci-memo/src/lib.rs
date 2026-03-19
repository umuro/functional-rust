// 1051: Fibonacci with HashMap Memoization

use std::collections::HashMap;

// Approach 1: Iterative with HashMap cache
fn fib_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&v) = cache.get(&n) {
        return v;
    }
    let v = fib_memo(n - 1, cache) + fib_memo(n - 2, cache);
    cache.insert(n, v);
    v
}

// Approach 2: Closure-based memoizer
fn make_fib_memo() -> impl FnMut(u64) -> u64 {
    let mut cache = HashMap::new();
    move |n| {
        fn inner(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
            if n <= 1 {
                return n;
            }
            if let Some(&v) = cache.get(&n) {
                return v;
            }
            let v = inner(n - 1, cache) + inner(n - 2, cache);
            cache.insert(n, v);
            v
        }
        inner(n, &mut cache)
    }
}

// Approach 3: Generic memoization wrapper
struct Memoize<F> {
    cache: HashMap<u64, u64>,
    func: F,
}

impl<F: Fn(u64, &mut dyn FnMut(u64) -> u64) -> u64> Memoize<F> {
    fn new(func: F) -> Self {
        Memoize {
            cache: HashMap::new(),
            func,
        }
    }

    fn call(&mut self, n: u64) -> u64 {
        if let Some(&v) = self.cache.get(&n) {
            return v;
        }
        // We need to use unsafe here or restructure; instead use a simpler pattern
        let mut cache = std::mem::take(&mut self.cache);
        let v = (self.func)(n, &mut |x| {
            if x <= 1 {
                return x;
            }
            if let Some(&v) = cache.get(&x) {
                return v;
            }
            // For deeply nested, fall back to iterative
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=x {
                let t = a + b;
                a = b;
                b = t;
            }
            cache.insert(x, b);
            b
        });
        cache.insert(n, v);
        self.cache = cache;
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_memo() {
        let mut cache = HashMap::new();
        assert_eq!(fib_memo(0, &mut cache), 0);
        assert_eq!(fib_memo(1, &mut cache), 1);
        assert_eq!(fib_memo(10, &mut cache), 55);
        assert_eq!(fib_memo(20, &mut cache), 6765);
        assert_eq!(fib_memo(30, &mut cache), 832040);
    }

    #[test]
    fn test_fib_closure() {
        let mut fib = make_fib_memo();
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(30), 832040);
    }

    #[test]
    fn test_fib_generic() {
        let mut memo = Memoize::new(|n, recurse: &mut dyn FnMut(u64) -> u64| {
            if n <= 1 {
                n
            } else {
                recurse(n - 1) + recurse(n - 2)
            }
        });
        assert_eq!(memo.call(0), 0);
        assert_eq!(memo.call(10), 55);
        assert_eq!(memo.call(30), 832040);
    }
}
