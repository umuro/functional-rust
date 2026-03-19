//! Async Closures
//!
//! Patterns for async callbacks using closures that return Futures.
//! Note: True `async |x| {...}` is nightly-only; we use `|x| async { ... }`.

use std::future::Future;

/// Async transform: closure returns a future.
pub fn async_transform<T, U, F, Fut>(value: T, f: F) -> impl Future<Output = U>
where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = U>,
{
    f(value)
}

/// Demonstrate async closure pattern (returns future).
pub async fn process_with_callback<T, F, Fut>(value: T, callback: F) -> T
where
    F: FnOnce(&T) -> Fut,
    Fut: Future<Output = ()>,
{
    callback(&value).await;
    value
}

/// Async map over a collection.
pub async fn async_map<T, U, F, Fut>(items: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = U>,
{
    let mut results = Vec::with_capacity(items.len());
    for item in items {
        results.push(f(item).await);
    }
    results
}

/// Async filter.
pub async fn async_filter<T, F, Fut>(items: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = bool>,
{
    let mut results = Vec::new();
    for item in items {
        if predicate(&item).await {
            results.push(item);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_transform_compiles() {
        // Just verify the types work - actual async test would need runtime
        let _future = async_transform(5, |x| async move { x * 2 });
    }

    #[test]
    fn test_closure_returning_future() {
        // Pattern: |x| async move { ... }
        let double = |x: i32| async move { x * 2 };
        let _fut = double(5);
        // In real code: assert_eq!(fut.await, 10);
    }

    #[test]
    fn test_async_map_compiles() {
        let _future = async_map(vec![1, 2, 3], |x| async move { x * 2 });
    }

    #[test]
    fn test_async_filter_compiles() {
        let _future = async_filter(vec![1, 2, 3, 4], |x| {
            let x = *x;
            async move { x % 2 == 0 }
        });
    }

    // Block-on test requires a runtime, showing pattern only
    #[test]
    fn test_pattern_demonstration() {
        // This demonstrates the closure-returning-future pattern
        let make_doubler = || |x: i32| async move { x * 2 };
        let doubler = make_doubler();
        let _fut = doubler(21);
        // With runtime: assert_eq!(fut.await, 42);
    }
}
