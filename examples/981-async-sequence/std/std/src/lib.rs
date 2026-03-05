// 981: Sequential Async Chain
// Rust: sequential .await calls — like OCaml's let* x = ... in let* y = ...

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn block_on<F: Future>(mut fut: F) -> F::Output {
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
    fn noop(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker { RawWaker::new(p, &VT) }
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) };
    let mut cx = Context::from_waker(&waker);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("not ready"),
    }
}

// --- Simulated async data-fetch functions ---
async fn fetch_user_id() -> u32 { 42 }
async fn fetch_user_name(_id: u32) -> String { "Alice".to_string() }
async fn fetch_user_email(_name: &str) -> String { "alice@example.com".to_string() }

// --- Approach 1: Sequential let-binding with await ---
// Each .await = one let* step in OCaml
async fn full_lookup() -> (u32, String, String) {
    let id = fetch_user_id().await;
    let name = fetch_user_name(id).await;
    let email = fetch_user_email(&name).await;
    (id, name, email)
}

// --- Approach 2: Accumulating through a pipeline ---
async fn step1(x: i32) -> i32 { x + 10 }
async fn step2(x: i32) -> i32 { x * 2 }
async fn step3(x: i32) -> i32 { x - 5 }

async fn pipeline_seq(input: i32) -> (i32, i32, i32, i32) {
    let a = step1(input).await;
    let b = step2(a).await;
    let c = step3(b).await;
    (input, a, b, c)
}

// --- Approach 3: Error-aware sequence with ? operator ---
async fn guarded_div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 { Err("division by zero") } else { Ok(a / b) }
}

async fn safe_pipeline() -> Result<i32, &'static str> {
    let x = 100;
    let y = guarded_div(x, 4).await?;   // let*? — short-circuits on Err
    let z = guarded_div(y, 5).await?;
    Ok(z)
}

async fn bad_pipeline() -> Result<i32, &'static str> {
    let x = 100;
    let _y = guarded_div(x, 0).await?;  // short-circuits here
    Ok(999)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_lookup() {
        let (id, name, email) = block_on(full_lookup());
        assert_eq!(id, 42);
        assert_eq!(name, "Alice");
        assert_eq!(email, "alice@example.com");
    }

    #[test]
    fn test_pipeline_seq() {
        let (orig, a, b, c) = block_on(pipeline_seq(5));
        assert_eq!(orig, 5);
        assert_eq!(a, 15);   // 5+10
        assert_eq!(b, 30);   // 15*2
        assert_eq!(c, 25);   // 30-5
    }

    #[test]
    fn test_safe_pipeline() {
        assert_eq!(block_on(safe_pipeline()), Ok(5)); // 100/4=25, 25/5=5
    }

    #[test]
    fn test_bad_pipeline_short_circuits() {
        assert_eq!(block_on(bad_pipeline()), Err("division by zero"));
    }

    #[test]
    fn test_sequential_order() {
        // Values from earlier awaits are available in later ones
        let result = block_on(async {
            let a = step1(10).await;  // 20
            let b = step2(a).await;   // 40 — uses a
            let c = step3(b).await;   // 35 — uses b
            c
        });
        assert_eq!(result, 35);
    }
}
