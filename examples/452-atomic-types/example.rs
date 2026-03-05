// 452. Atomic types: AtomicBool, AtomicUsize
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..4).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || { for _ in 0..1000 { c.fetch_add(1, Ordering::Relaxed); } })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("counter: {} (expected 4000)", counter.load(Ordering::SeqCst));

    let running = Arc::new(AtomicBool::new(true));
    let work    = Arc::new(AtomicUsize::new(0));
    let (r,w)   = (Arc::clone(&running), Arc::clone(&work));
    let worker  = thread::spawn(move || { while r.load(Ordering::Relaxed) { w.fetch_add(1,Ordering::Relaxed); thread::yield_now(); } });
    thread::sleep(Duration::from_millis(5));
    running.store(false, Ordering::Relaxed);
    worker.join().unwrap();
    println!("work done: {}", work.load(Ordering::SeqCst));

    let a = AtomicUsize::new(10);
    let old = a.fetch_add(5, Ordering::SeqCst);
    println!("fetch_add: old={} new={}", old, a.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_counter() {
        let c = Arc::new(AtomicUsize::new(0));
        let hs:Vec<_>=(0..4).map(|_|{let c=Arc::clone(&c); thread::spawn(move || { for _ in 0..100 { c.fetch_add(1,Ordering::Relaxed); } })}).collect();
        for h in hs { h.join().unwrap(); }
        assert_eq!(c.load(Ordering::SeqCst), 400);
    }
    #[test] fn test_bool() {
        let f = AtomicBool::new(false);
        f.store(true, Ordering::SeqCst); assert!(f.load(Ordering::SeqCst));
        let prev = f.swap(false, Ordering::SeqCst); assert!(prev);
    }
}
