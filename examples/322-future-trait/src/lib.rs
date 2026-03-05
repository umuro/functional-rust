//! # The Future Trait and Poll
//!
//! Understanding the core Future trait: `poll`, `Poll::Ready`, `Poll::Pending`,
//! and how to implement custom futures manually.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A future that returns a value after being polled a certain number of times.
/// Demonstrates the manual implementation of the Future trait.
pub struct DelayedValue {
    value: i32,
    remaining_polls: u32,
}

impl DelayedValue {
    /// Create a new delayed value that will be ready after `polls` poll calls.
    pub fn new(value: i32, polls: u32) -> Self {
        Self {
            value,
            remaining_polls: polls,
        }
    }
}

impl Future for DelayedValue {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining_polls == 0 {
            Poll::Ready(self.value)
        } else {
            self.remaining_polls -= 1;
            // Schedule a wakeup so the runtime knows to poll again
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// A future that is immediately ready with a value.
pub struct Ready<T> {
    value: Option<T>,
}

impl<T> Ready<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut().value.take() {
            Some(v) => Poll::Ready(v),
            None => panic!("Ready polled after completion"),
        }
    }
}

/// A future that counts how many times it was polled before returning.
pub struct PollCounter {
    target: u32,
    current: u32,
}

impl PollCounter {
    pub fn new(target: u32) -> Self {
        Self { target, current: 0 }
    }
}

impl Future for PollCounter {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.current += 1;
        if self.current >= self.target {
            Poll::Ready(self.current)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// A minimal single-threaded executor that blocks until a future completes.
/// This is a simplified version - real executors are much more sophisticated.
pub fn block_on<F: Future>(mut fut: F) -> F::Output {
    // Create a no-op waker (simplest possible implementation)
    unsafe fn clone(ptr: *const ()) -> RawWaker {
        RawWaker::new(ptr, &VTABLE)
    }
    unsafe fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);

    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);

    // SAFETY: We never move `fut` after pinning
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    // Keep polling until ready
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delayed_value_immediate() {
        let future = DelayedValue::new(42, 0);
        assert_eq!(block_on(future), 42);
    }

    #[test]
    fn test_delayed_value_with_polls() {
        let future = DelayedValue::new(100, 5);
        assert_eq!(block_on(future), 100);
    }

    #[test]
    fn test_ready_immediate() {
        let future = Ready::new("hello");
        assert_eq!(block_on(future), "hello");
    }

    #[test]
    fn test_poll_counter_counts_correctly() {
        let future = PollCounter::new(3);
        assert_eq!(block_on(future), 3);
    }

    #[test]
    fn test_poll_counter_single_poll() {
        let future = PollCounter::new(1);
        assert_eq!(block_on(future), 1);
    }

    #[test]
    fn test_delayed_value_preserves_value() {
        let future1 = DelayedValue::new(-42, 2);
        let future2 = DelayedValue::new(i32::MAX, 1);
        assert_eq!(block_on(future1), -42);
        assert_eq!(block_on(future2), i32::MAX);
    }
}
