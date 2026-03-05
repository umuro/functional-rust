//! # Arc<Mutex<T>> Pattern
//! Thread-safe shared mutable state: Arc gives shared ownership, Mutex ensures exclusive access.

use std::sync::{Arc, Mutex};
use std::thread;

pub fn shared_counter(num_threads: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..num_threads).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || { *c.lock().unwrap() += 1; })
    }).collect();
    for h in handles { h.join().unwrap(); }
    let result = *counter.lock().unwrap();
    result
}

pub struct ThreadSafeCache<T> { data: Arc<Mutex<Vec<T>>> }

impl<T: Clone> ThreadSafeCache<T> {
    pub fn new() -> Self { Self { data: Arc::new(Mutex::new(Vec::new())) } }
    pub fn push(&self, item: T) { self.data.lock().unwrap().push(item); }
    pub fn get_all(&self) -> Vec<T> { self.data.lock().unwrap().clone() }
    pub fn len(&self) -> usize { self.data.lock().unwrap().len() }
}

impl<T: Clone> Default for ThreadSafeCache<T> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn counter_works() { assert_eq!(shared_counter(10), 10); }
    #[test] fn cache_thread_safe() {
        let cache = Arc::new(ThreadSafeCache::<i32>::new());
        let handles: Vec<_> = (0..5).map(|i| {
            let c = Arc::clone(&cache);
            thread::spawn(move || c.push(i))
        }).collect();
        for h in handles { h.join().unwrap(); }
        assert_eq!(cache.len(), 5);
    }
}
