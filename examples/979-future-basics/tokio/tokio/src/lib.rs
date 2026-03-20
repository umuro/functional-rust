#![allow(clippy::all)]
// 979: Future/Promise Basics — Tokio version
// Using tokio runtime instead of manual block_on

/// Compute a value asynchronously
async fn compute_value() -> i32 {
    42
}

/// Chain: compute then add
async fn compute_and_add() -> i32 {
    let x = compute_value().await;
    x + 1
}

/// Chain: compute, add, double
async fn double_result() -> i32 {
    let x = compute_and_add().await;
    x * 2
}

/// Pipeline of async transformations
async fn pipeline(input: i32) -> i32 {
    let step1 = async { input * 2 }.await;
    let step2 = async { step1 + 10 }.await;
    let step3 = async { step2.to_string().len() as i32 }.await;
    step3
}

/// Spawn futures as independent tasks on the tokio runtime
async fn spawned_computation() -> i32 {
    let handle1 = tokio::spawn(compute_value());
    let handle2 = tokio::spawn(compute_and_add());

    let v1 = handle1.await.unwrap();
    let v2 = handle2.await.unwrap();
    v1 + v2 // 42 + 43 = 85
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_value() {
        assert_eq!(compute_value().await, 42);
    }

    #[tokio::test]
    async fn test_compute_and_add() {
        assert_eq!(compute_and_add().await, 43);
    }

    #[tokio::test]
    async fn test_double_result() {
        assert_eq!(double_result().await, 86);
    }

    #[tokio::test]
    async fn test_pipeline() {
        // 5*2=10, 10+10=20, len("20")=2
        assert_eq!(pipeline(5).await, 2);
    }

    #[tokio::test]
    async fn test_spawned_computation() {
        assert_eq!(spawned_computation().await, 85);
    }

    #[tokio::test]
    async fn test_async_is_lazy() {
        let _fut = compute_value(); // not executed yet
        let result = compute_value().await;
        assert_eq!(result, 42);
    }
}
