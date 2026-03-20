#![allow(clippy::all)]
// 980: Map over Async — Tokio version
// async { f(x.await) } is the idiom for mapping over futures

/// Base future
async fn base_value() -> i32 {
    5
}

/// Map: transform the output of a future
async fn map_double(fut: impl std::future::Future<Output = i32>) -> i32 {
    fut.await * 2
}

async fn map_to_string(fut: impl std::future::Future<Output = i32>) -> String {
    fut.await.to_string()
}

/// Compose maps via sequential await
async fn map_chain() -> String {
    let raw = base_value().await;          // 5
    let doubled = raw * 2;                 // 10
    let as_str = doubled.to_string();      // "10"
    as_str
}

/// map derived from bind (async = bind + return)
async fn map_via_bind<T, U, F>(fut: impl std::future::Future<Output = T>, f: F) -> U
where
    F: FnOnce(T) -> U,
{
    f(fut.await)
}

/// Tokio-specific: spawn map operations as concurrent tasks
async fn concurrent_map() -> Vec<i32> {
    let handles: Vec<_> = (1..=5)
        .map(|i| tokio::spawn(async move { i * i }))
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// Functor identity law
async fn identity_law() -> bool {
    let val = base_value().await;
    let mapped = async { base_value().await }.await;
    val == mapped
}

/// Functor composition law
async fn composition_law() -> bool {
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 3;

    let composed = async { f(g(base_value().await)) }.await;
    let chained = async { f(async { g(base_value().await) }.await) }.await;
    composed == chained
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_map_double() {
        assert_eq!(map_double(base_value()).await, 10);
    }

    #[tokio::test]
    async fn test_map_to_string() {
        assert_eq!(map_to_string(base_value()).await, "5");
    }

    #[tokio::test]
    async fn test_map_chain() {
        assert_eq!(map_chain().await, "10");
    }

    #[tokio::test]
    async fn test_map_via_bind() {
        assert_eq!(map_via_bind(base_value(), |x| x * x).await, 25);
    }

    #[tokio::test]
    async fn test_concurrent_map() {
        assert_eq!(concurrent_map().await, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_identity_law() {
        assert!(identity_law().await);
    }

    #[tokio::test]
    async fn test_composition_law() {
        assert!(composition_law().await);
    }

    #[tokio::test]
    async fn test_inline_map() {
        let result = async { base_value().await + 100 }.await;
        assert_eq!(result, 105);
    }
}
