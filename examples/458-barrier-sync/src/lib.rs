// 458. Barrier for thread synchronization
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize,Ordering};
    #[test] fn test_all_arrive() {
        let n=4; let b=Arc::new(Barrier::new(n)); let c=Arc::new(AtomicUsize::new(0));
        thread::scope(|s| { for _ in 0..n { let b=Arc::clone(&b); let c=Arc::clone(&c); s.spawn(move || { c.fetch_add(1,Ordering::SeqCst); b.wait(); assert_eq!(c.load(Ordering::SeqCst),n); }); } });
    }
    #[test] fn test_one_leader() {
        let n=5; let b=Arc::new(Barrier::new(n)); let leaders=Arc::new(AtomicUsize::new(0));
        thread::scope(|s| { for _ in 0..n { let b=Arc::clone(&b); let l=Arc::clone(&leaders); s.spawn(move || { if b.wait().is_leader() { l.fetch_add(1,Ordering::SeqCst); } }); } });
        assert_eq!(leaders.load(Ordering::SeqCst),1);
    }
}
