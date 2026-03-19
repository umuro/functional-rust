#![allow(clippy::all)]
//! # Waker and Context
//!
//! How the executor knows to re-poll a future — `cx.waker().wake()` schedules a re-poll.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Shared state between a future and its resolver.
#[derive(Default)]
pub struct SharedState<T> {
    pub value: Option<T>,
    pub waker: Option<Waker>,
}

/// A future that completes when an external source provides a value.
pub struct ExternalFuture<T> {
    state: Arc<Mutex<SharedState<T>>>,
}

impl<T: Clone> Future for ExternalFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.value.clone() {
            Poll::Ready(value)
        } else {
            // Store the waker so the resolver can wake us later
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

/// A resolver that can fulfill the associated future.
pub struct Resolver<T> {
    state: Arc<Mutex<SharedState<T>>>,
}

impl<T> Resolver<T> {
    /// Fulfill the future with a value, waking the executor.
    pub fn fulfill(self, value: T) {
        let mut state = self.state.lock().unwrap();
        state.value = Some(value);

        // Wake the executor so it knows to re-poll
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

/// Create a linked future and resolver pair.
pub fn make_future<T>() -> (ExternalFuture<T>, Resolver<T>) {
    let state = Arc::new(Mutex::new(SharedState {
        value: None,
        waker: None,
    }));
    (
        ExternalFuture {
            state: Arc::clone(&state),
        },
        Resolver { state },
    )
}

/// A minimal executor that properly handles waker notifications.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::{RawWaker, RawWakerVTable};

    let ready = Arc::new(AtomicBool::new(true));

    // Waker vtable that operates on Arc<AtomicBool>
    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let arc = Arc::from_raw(ptr as *const AtomicBool);
        let cloned = Arc::clone(&arc);
        std::mem::forget(arc); // don't drop the original
        RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let arc = Arc::from_raw(ptr as *const AtomicBool);
        arc.store(true, Ordering::Release);
        // arc is dropped here (consumed)
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let arc = Arc::from_raw(ptr as *const AtomicBool);
        arc.store(true, Ordering::Release);
        std::mem::forget(arc); // don't drop
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Arc::from_raw(ptr as *const AtomicBool));
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, wake, wake_by_ref, drop_waker);

    let raw = RawWaker::new(Arc::into_raw(Arc::clone(&ready)) as *const (), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);

    loop {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
        // Wait until woken
        while !ready.swap(false, Ordering::AcqRel) {
            std::hint::spin_loop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_external_future() {
        let (fut, resolver) = make_future::<i32>();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            resolver.fulfill(42);
        });

        assert_eq!(block_on(fut), 42);
    }

    #[test]
    fn test_immediate_fulfill() {
        let (fut, resolver) = make_future::<String>();
        resolver.fulfill("hello".to_string());
        assert_eq!(block_on(fut), "hello");
    }

    #[test]
    fn test_multiple_futures() {
        let (fut1, res1) = make_future::<i32>();
        let (fut2, res2) = make_future::<i32>();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            res1.fulfill(1);
        });
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(3));
            res2.fulfill(2);
        });

        assert_eq!(block_on(fut1), 1);
        assert_eq!(block_on(fut2), 2);
    }

    #[test]
    fn test_shared_state_default() {
        let state: SharedState<i32> = SharedState::default();
        assert!(state.value.is_none());
        assert!(state.waker.is_none());
    }
}
