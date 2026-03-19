#![allow(clippy::all)]
// 981: Sequential Async Chain — Tokio version
// Sequential .await calls — like OCaml's let* x = ... in let* y = ...

/// Simulated async data-fetch functions
async fn fetch_user_id() -> u32 { 42 }
async fn fetch_user_name(_id: u32) -> String { "Alice".to_string() }
async fn fetch_user_email(_name: &str) -> String { "alice@example.com".to_string() }

/// Sequential let-binding with await
async fn full_lookup() -> (u32, String, String) {
    let id = fetch_user_id().await;
    let name = fetch_user_name(id).await;
    let email = fetch_user_email(&name).await;
    (id, name, email)
}

/// Pipeline of async steps
async fn step1(x: i32) -> i32 { x + 10 }
async fn step2(x: i32) -> i32 { x * 2 }
async fn step3(x: i32) -> i32 { x - 5 }

async fn pipeline_seq(input: i32) -> (i32, i32, i32, i32) {
    let a = step1(input).await;
    let b = step2(a).await;
    let c = step3(b).await;
    (input, a, b, c)
}

/// Error-aware sequence with ? operator
async fn guarded_div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 { Err("division by zero") } else { Ok(a / b) }
}

async fn safe_pipeline() -> Result<i32, &'static str> {
    let x = 100;
    let y = guarded_div(x, 4).await?;
    let z = guarded_div(y, 5).await?;
    Ok(z)
}

async fn bad_pipeline() -> Result<i32, &'static str> {
    let x = 100;
    let _y = guarded_div(x, 0).await?;
    Ok(999)
}

/// Tokio-specific: spawn sequential pipeline steps
async fn spawned_pipeline(input: i32) -> i32 {
    let a = tokio::spawn(async move { step1(input).await }).await.unwrap();
    let b = tokio::spawn(async move { step2(a).await }).await.unwrap();
    let c = tokio::spawn(async move { step3(b).await }).await.unwrap();
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_full_lookup() {
        let (id, name, email) = full_lookup().await;
        assert_eq!(id, 42);
        assert_eq!(name, "Alice");
        assert_eq!(email, "alice@example.com");
    }

    #[tokio::test]
    async fn test_pipeline_seq() {
        let (orig, a, b, c) = pipeline_seq(5).await;
        assert_eq!(orig, 5);
        assert_eq!(a, 15);
        assert_eq!(b, 30);
        assert_eq!(c, 25);
    }

    #[tokio::test]
    async fn test_safe_pipeline() {
        assert_eq!(safe_pipeline().await, Ok(5));
    }

    #[tokio::test]
    async fn test_bad_pipeline_short_circuits() {
        assert_eq!(bad_pipeline().await, Err("division by zero"));
    }

    #[tokio::test]
    async fn test_spawned_pipeline() {
        // 5+10=15, 15*2=30, 30-5=25
        assert_eq!(spawned_pipeline(5).await, 25);
    }

    #[tokio::test]
    async fn test_sequential_order() {
        let result = async {
            let a = step1(10).await;  // 20
            let b = step2(a).await;   // 40
            let c = step3(b).await;   // 35
            c
        }.await;
        assert_eq!(result, 35);
    }
}
