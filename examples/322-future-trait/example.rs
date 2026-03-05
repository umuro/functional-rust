use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct DelayedValue { value: i32, remaining: u32 }

impl DelayedValue {
    fn new(value: i32, polls: u32) -> Self { Self { value, remaining: polls } }
}

impl Future for DelayedValue {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 { Poll::Ready(self.value) }
        else { self.remaining -= 1; cx.waker().wake_by_ref(); Poll::Pending }
    }
}

fn block_on<F: Future>(mut fut: F) -> F::Output {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    unsafe fn clone(p: *const ()) -> RawWaker { RawWaker::new(p, &VTABLE) }
    unsafe fn noop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
    loop { if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) { return v; } }
}

fn main() {
    println!("Got: {}", block_on(DelayedValue::new(42, 3)));
    println!("Immediate: {}", block_on(DelayedValue::new(99, 0)));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn delayed() { assert_eq!(block_on(DelayedValue::new(10, 5)), 10); }
    #[test] fn immediate() { assert_eq!(block_on(DelayedValue::new(7, 0)), 7); }
}
