#![allow(clippy::all)]
// 453. Memory ordering: Relaxed, Acquire, Release
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_release_acquire() {
        let d = Arc::new(AtomicUsize::new(0));
        let f = Arc::new(AtomicBool::new(false));
        let (dc, fc) = (Arc::clone(&d), Arc::clone(&f));
        thread::spawn(move || {
            dc.store(42, Ordering::Relaxed);
            fc.store(true, Ordering::Release);
        })
        .join()
        .unwrap();
        assert!(f.load(Ordering::Acquire));
        assert_eq!(d.load(Ordering::Relaxed), 42);
    }
}
