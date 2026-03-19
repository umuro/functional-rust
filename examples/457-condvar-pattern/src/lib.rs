// 457. Condvar for thread notification
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_notify() {
        let p = Arc::new((Mutex::new(false), Condvar::new()));
        let pp = Arc::clone(&p);
        let w = thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            *pp.0.lock().unwrap() = true;
            pp.1.notify_one();
        });
        let g = p.1.wait_while(p.0.lock().unwrap(), |&mut v| !v).unwrap();
        assert!(*g);
        w.join().unwrap();
    }
}
