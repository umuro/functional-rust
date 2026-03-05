//! # Blocking in Async
//! How to safely run blocking operations in async contexts.

use std::thread;
use std::time::Duration;

pub fn blocking_computation(n: u64) -> u64 {
    thread::sleep(Duration::from_millis(10));
    (1..=n).product()
}

pub fn spawn_blocking<F, R>(f: F) -> thread::JoinHandle<R>
where F: FnOnce() -> R + Send + 'static, R: Send + 'static {
    thread::spawn(f)
}

pub fn run_blocking_batch<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
where T: Send + 'static, R: Send + 'static, F: Fn(T) -> R + Send + Sync + Clone + 'static {
    let handles: Vec<_> = items.into_iter().map(|item| {
        let f = f.clone();
        thread::spawn(move || f(item))
    }).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn blocking_works() { assert_eq!(blocking_computation(5), 120); }
    #[test] fn spawn_blocking_works() {
        let h = spawn_blocking(|| 2 + 2);
        assert_eq!(h.join().unwrap(), 4);
    }
    #[test] fn batch_blocking() {
        let results = run_blocking_batch(vec![1,2,3], |x| x * 2);
        assert_eq!(results, vec![2, 4, 6]);
    }
}
