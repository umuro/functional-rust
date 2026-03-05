//! # 529. Async Closures
//! Patterns for async callbacks using closures that return Futures.
//! Note: True `async |x| {...}` is nightly-only; we use `|x| async { ... }`.

use std::future::Future;
use std::pin::Pin;

/// Type alias for a boxed future (common in async callback patterns)
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Accept an async callback: closure returns a Future
fn for_each_async<T, F, Fut>(items: Vec<T>, f: F)
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = ()>,
{
    // In real async code, you'd use an executor here
    // For demonstration, we show the type pattern
    let _futures: Vec<Fut> = items.into_iter().map(f).collect();
    println!("[for_each_async: would await all futures in real runtime]");
}

/// Async map: transform items with an async function
async fn async_map<T, U, F, Fut>(items: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = U>,
{
    let mut results = Vec::new();
    for item in items {
        results.push(f(item).await);
    }
    results
}

/// Retry pattern with async closure
async fn retry<T, E, F, Fut>(max_attempts: usize, f: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut last_err = None;
    for attempt in 0..max_attempts {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                println!("Attempt {} failed, retrying...", attempt + 1);
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap())
}

/// Simulated async fetch (returns immediately in this demo)
async fn fake_fetch(url: &str) -> Result<String, String> {
    // In real code: HTTP request
    if url.contains("fail") {
        Err(format!("Failed to fetch {}", url))
    } else {
        Ok(format!("Data from {}", url))
    }
}

/// Async closure pattern: |x| async move { ... }
fn make_async_processor(prefix: String) -> impl Fn(i32) -> BoxFuture<'static, String> {
    move |x: i32| {
        let prefix = prefix.clone();
        Box::pin(async move {
            // Simulated async work
            format!("{}: {}", prefix, x * 2)
        })
    }
}

fn main() {
    // Create a minimal async runtime for demo
    let rt = std::sync::Arc::new(std::sync::Mutex::new(()));

    // Show async closure types
    println!("=== Async closure patterns ===");

    // Pattern 1: closure returning async block
    let async_double = |x: i32| async move { x * 2 };
    println!("async_double type created (returns Future<Output=i32>)");

    // Pattern 2: closure returning BoxFuture (for dynamic dispatch)
    let async_fmt: Box<dyn Fn(i32) -> BoxFuture<'static, String>> =
        Box::new(|x| Box::pin(async move { format!("value={}", x) }));
    println!("dynamic async closure created");

    // Pattern 3: async map via block_on simulation
    // (In real code: tokio::main or async_std::main)
    println!("\n=== Simulating async pipeline ===");

    // Show the patterns without actually running a runtime
    let processor = make_async_processor("result".to_string());
    println!("Processor created — would process: [1, 2, 3, 4, 5]");

    // Demonstrate async retry pattern (conceptual)
    println!("\nRetry pattern: would retry up to 3 times on failure");
    println!("Pattern: retry(3, || async {{ fetch(url).await }})");

    // for_each_async demonstration
    for_each_async(vec![1, 2, 3], |x| async move {
        println!("Processing: {}", x);
    });

    // Drop suppression
    let _ = async_double;
    let _ = async_fmt;
    let _ = rt;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_closure_type() {
        // Verify the closure compiles and returns a Future
        let f = |x: i32| async move { x + 1 };
        let future = f(41);
        // Can't easily run without a runtime in tests without tokio
        // Just verify it compiles and is a valid Future type
        let _ = future;
    }

    #[test]
    fn test_make_async_processor() {
        let proc = make_async_processor("test".to_string());
        let _future = proc(5); // returns Future<Output=String>
        // Just verify it compiles
    }

    // With tokio as dev-dependency, you could write:
    // #[tokio::test]
    // async fn test_async_map() {
    //     let results = async_map(vec![1,2,3], |x| async move { x * 2 }).await;
    //     assert_eq!(results, vec![2,4,6]);
    // }
}
