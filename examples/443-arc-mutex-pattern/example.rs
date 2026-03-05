// 443. Arc<Mutex<T>> shared state
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let handles: Vec<_> = (0..10).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || { for _ in 0..100 { *c.lock().unwrap() += 1; } })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("Counter: {} (expected 1000)", *counter.lock().unwrap());

    // Shared Vec
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let handles: Vec<_> = (0..4).map(|i| {
        let log = Arc::clone(&log);
        thread::spawn(move || log.lock().unwrap().push(format!("thread-{}", i)))
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("Log entries: {}", log.lock().unwrap().len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_counter() {
        let c = Arc::new(Mutex::new(0u64));
        let hs: Vec<_> = (0..4).map(|_| { let c=Arc::clone(&c); thread::spawn(move || { for _ in 0..250 { *c.lock().unwrap()+=1; } }) }).collect();
        for h in hs { h.join().unwrap(); }
        assert_eq!(*c.lock().unwrap(), 1000);
    }
    #[test] fn test_try_lock() { let m=Mutex::new(0); let _g=m.lock().unwrap(); assert!(m.try_lock().is_err()); }
}
