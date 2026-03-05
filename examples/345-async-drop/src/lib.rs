//! # Async Drop
//! Cleanup resources when async tasks are cancelled or complete.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Resource {
    id: usize,
    cleaned_up: Arc<AtomicBool>,
}

impl Resource {
    pub fn new(id: usize) -> (Self, Arc<AtomicBool>) {
        let flag = Arc::new(AtomicBool::new(false));
        (Self { id, cleaned_up: Arc::clone(&flag) }, flag)
    }
    pub fn id(&self) -> usize { self.id }
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.cleaned_up.store(true, Ordering::SeqCst);
    }
}

pub struct Guard<F: FnOnce()> { cleanup: Option<F> }

impl<F: FnOnce()> Guard<F> {
    pub fn new(cleanup: F) -> Self { Self { cleanup: Some(cleanup) } }
    pub fn disarm(mut self) { self.cleanup = None; }
}

impl<F: FnOnce()> Drop for Guard<F> {
    fn drop(&mut self) { if let Some(f) = self.cleanup.take() { f(); } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn resource_cleanup_on_drop() {
        let flag;
        { let (r, f) = Resource::new(1); flag = f; assert_eq!(r.id(), 1); }
        assert!(flag.load(Ordering::SeqCst));
    }
    #[test] fn guard_runs_cleanup() {
        let called = Arc::new(AtomicBool::new(false));
        let c = Arc::clone(&called);
        { let _g = Guard::new(move || c.store(true, Ordering::SeqCst)); }
        assert!(called.load(Ordering::SeqCst));
    }
}
