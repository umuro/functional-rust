use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Default)]
struct SharedState { value: Option<i32>, waker: Option<Waker> }

struct ExternalFuture { state: Arc<Mutex<SharedState>> }

impl Future for ExternalFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        let mut s = self.state.lock().unwrap();
        if let Some(v) = s.value { Poll::Ready(v) }
        else { s.waker = Some(cx.waker().clone()); Poll::Pending }
    }
}

struct Resolver { state: Arc<Mutex<SharedState>> }

impl Resolver {
    fn fulfill(self, value: i32) {
        let mut s = self.state.lock().unwrap();
        s.value = Some(value);
        if let Some(w) = s.waker.take() { w.wake(); }
    }
}

fn make_future() -> (ExternalFuture, Resolver) {
    let s = Arc::new(Mutex::new(SharedState::default()));
    (ExternalFuture{state:Arc::clone(&s)}, Resolver{state:s})
}

fn block_on<F:Future>(fut: F) -> F::Output {
    use std::task::{RawWaker,RawWakerVTable};
    use std::sync::atomic::{AtomicBool,Ordering};
    let ready = Arc::new(AtomicBool::new(true));
    let r2 = Arc::clone(&ready);
    let wake_fn: Arc<dyn Fn()+Send+Sync> = Arc::new(move || { r2.store(true, Ordering::Release); });
    unsafe fn cw(p:*const())->RawWaker{let a=Arc::from_raw(p as *const(dyn Fn()+Send+Sync));let c=Arc::clone(&a);std::mem::forget(a);RawWaker::new(Arc::into_raw(c) as *const(),&V)}
    unsafe fn w(p:*const()){let a=Arc::from_raw(p as *const(dyn Fn()+Send+Sync));a();}
    unsafe fn wbr(p:*const()){let a=Arc::from_raw(p as *const(dyn Fn()+Send+Sync));a();std::mem::forget(a);}
    unsafe fn dw(p:*const()){drop(Arc::from_raw(p as *const(dyn Fn()+Send+Sync)));}
    static V: RawWakerVTable = RawWakerVTable::new(cw,w,wbr,dw);
    let raw = RawWaker::new(Arc::into_raw(wake_fn) as *const(), &V);
    let waker = unsafe{std::task::Waker::from_raw(raw)};
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) { return v; }
        while !ready.swap(false, Ordering::AcqRel) { std::hint::spin_loop(); }
    }
}

fn main() {
    use std::thread; use std::time::Duration;
    let (fut, res) = make_future();
    thread::spawn(move || { thread::sleep(Duration::from_millis(10)); res.fulfill(99); });
    println!("Got: {}", block_on(fut));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn external_future() {
        use std::thread; use std::time::Duration;
        let (fut, res) = make_future();
        thread::spawn(move || { thread::sleep(Duration::from_millis(5)); res.fulfill(42); });
        assert_eq!(block_on(fut), 42);
    }
}
