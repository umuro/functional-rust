// 979: Future/Promise Basics
// Rust async fn + await — showing the monad connection in pure std code

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// --- A minimal synchronous executor (no tokio needed) ---
fn block_on<F: Future>(mut fut: F) -> F::Output {
    // Safety: we pin the future on the stack
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    // Create a no-op waker
    fn noop(_: *const ()) {}
    fn noop_clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);

    // For simple futures that resolve immediately, one poll is enough
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("Future not ready — use a real executor for async I/O"),
    }
}

// --- Approach 1: async fn is syntactic sugar for impl Future ---
async fn compute_value() -> i32 {
    42
}

async fn compute_and_add() -> i32 {
    let x = compute_value().await; // bind: unwrap the future
    x + 1
}

async fn double_result() -> i32 {
    let x = compute_and_add().await;
    x * 2 // map: transform the value
}

// --- Approach 2: async block as lambda ---
async fn pipeline(input: i32) -> i32 {
    // Sequential monadic chain via .await
    let step1 = async { input * 2 }.await;
    let step2 = async { step1 + 10 }.await;
    let step3 = async { step2.to_string().len() as i32 }.await;
    step3
}

// --- Approach 3: Manual Future implementing the trait ---
struct ImmediateFuture<T>(Option<T>);

impl<T: Unpin> Future for ImmediateFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("polled after completion"))
    }
}

fn immediate<T>(val: T) -> ImmediateFuture<T> {
    ImmediateFuture(Some(val))
}

async fn use_manual_future() -> i32 {
    immediate(100).await + immediate(23).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_value() {
        assert_eq!(block_on(compute_value()), 42);
    }

    #[test]
    fn test_compute_and_add() {
        assert_eq!(block_on(compute_and_add()), 43);
    }

    #[test]
    fn test_double_result() {
        assert_eq!(block_on(double_result()), 86);
    }

    #[test]
    fn test_pipeline() {
        // 5*2=10, 10+10=20, len("20")=2
        assert_eq!(block_on(pipeline(5)), 2);
    }

    #[test]
    fn test_manual_future() {
        assert_eq!(block_on(use_manual_future()), 123);
    }

    #[test]
    fn test_async_is_lazy() {
        // Creating a future does NOT run it — laziness like OCaml's thunk
        let _fut = compute_value(); // nothing runs here
        let result = block_on(_fut);
        assert_eq!(result, 42);
    }
}
