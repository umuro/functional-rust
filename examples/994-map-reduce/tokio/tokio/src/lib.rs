// 994: MapReduce — Tokio version
// Parallel async map with tokio::spawn, collect results, reduce

use std::sync::Arc;

/// Parallel map using tokio::spawn
async fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let f = Arc::clone(&f);
            tokio::spawn(async move { f(item) })
        })
        .collect();

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

/// MapReduce: parallel async map + sequential reduce
async fn map_reduce<T, U, R, F, G>(items: Vec<T>, map_fn: F, reduce_fn: G, init: R) -> R
where
    T: Send + 'static,
    U: Send + 'static,
    R: 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
    G: Fn(R, U) -> R,
{
    let mapped = parallel_map(items, map_fn).await;
    mapped.into_iter().fold(init, reduce_fn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parallel_map_squares() {
        let nums: Vec<i32> = (1..=5).collect();
        let squares = parallel_map(nums, |x| x * x).await;
        assert_eq!(squares, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn test_map_reduce_sum() {
        let nums: Vec<i64> = (1..=20).collect();
        let sum = map_reduce(nums, |x| x * x, |a, b| a + b, 0).await;
        assert_eq!(sum, 2870);
    }

    #[tokio::test]
    async fn test_map_reduce_word_count() {
        let sentences = vec!["the quick brown fox", "jumps over the lazy", "dog today"];
        let count = map_reduce(
            sentences,
            |s: &str| s.split_whitespace().count(),
            |a, b| a + b,
            0,
        ).await;
        assert_eq!(count, 10);
    }

    #[tokio::test]
    async fn test_map_reduce_char_count() {
        let words = vec!["hello", "world", "ocaml", "functional", "programming"];
        let total = map_reduce(words, |w: &str| w.len(), |a, b| a + b, 0).await;
        assert_eq!(total, 36);
    }

    #[tokio::test]
    async fn test_parallel_map_empty() {
        let result: Vec<i32> = parallel_map(vec![], |x: i32| x * 2).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_map_reduce_string() {
        let items = vec!["a", "bb", "ccc"];
        let concat = map_reduce(
            items,
            |s: &str| s.to_uppercase(),
            |a: String, b| a + &b,
            String::new(),
        ).await;
        let mut chars: Vec<char> = concat.chars().collect();
        chars.sort();
        assert_eq!(chars, vec!['A', 'B', 'B', 'C', 'C', 'C']);
    }
}
