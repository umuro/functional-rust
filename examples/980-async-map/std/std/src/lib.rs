#![allow(clippy::all)]
// 980: Map over Async
// Rust: async { f(x.await) } is the idiom for Lwt.map f promise

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

// The base future
async fn base_value() -> i32 { 5 }

// --- map: transform the output of a future ---
// Lwt.map (fun x -> x * 2) fut  ≡  async { fut.await * 2 }
async fn map_double(fut: impl Future<Output = i32>) -> i32 {
    fut.await * 2
}

async fn map_to_string(fut: impl Future<Output = i32>) -> String {
    fut.await.to_string()
}

// --- Functor-style: compose maps ---
async fn map_chain() -> String {
    let raw = base_value().await;          // 5
    let doubled = raw * 2;                 // 10  (map)
    let as_str = doubled.to_string();      // "10" (map)
    as_str
}

// --- map derived from bind (async block = bind + return) ---
async fn map_via_bind<T, U, F>(fut: impl Future<Output = T>, f: F) -> U
where
    F: FnOnce(T) -> U,
{
    // .await is bind, wrapping in async is return
    f(fut.await)
}

// --- Functor laws ---
async fn identity_law() -> bool {
    let val = base_value().await;
    let mapped = async { base_value().await }.await; // map id
    val == mapped
}

async fn composition_law() -> bool {
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 3;

    // map (f . g) fut
    let composed = async { f(g(base_value().await)) }.await;
    // map f (map g fut)
    let chained = async { f(async { g(base_value().await) }.await) }.await;
    composed == chained
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_double() {
        assert_eq!(block_on(map_double(base_value())), 10);
    }

    #[test]
    fn test_map_to_string() {
        assert_eq!(block_on(map_to_string(base_value())), "5");
    }

    #[test]
    fn test_map_chain() {
        assert_eq!(block_on(map_chain()), "10");
    }

    #[test]
    fn test_map_via_bind() {
        assert_eq!(block_on(map_via_bind(base_value(), |x| x * x)), 25);
    }

    #[test]
    fn test_identity_law() {
        assert!(block_on(identity_law()));
    }

    #[test]
    fn test_composition_law() {
        assert!(block_on(composition_law()));
    }

    #[test]
    fn test_inline_map() {
        // Inline Lwt.map style
        let result = block_on(async { base_value().await + 100 });
        assert_eq!(result, 105);
    }
}
