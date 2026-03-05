use std::sync::{Arc, Mutex};
use std::thread;

fn sync_mutex_demo() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || { *c.lock().unwrap() += 1; })
    }).collect();
    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}

fn correct_pattern() {
    let shared = Arc::new(Mutex::new(vec![1i32,2,3]));
    // CORRECT: get value, release lock, then do work
    let sum = { shared.lock().unwrap().iter().sum::<i32>() }; // guard drops here
    println!("Sum (lock released): {sum}");
}

fn main() {
    println!("Counter: {}", sync_mutex_demo());
    correct_pattern();
    // Poison recovery
    let m = Arc::new(Mutex::new(0));
    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || { let _g = m2.lock().unwrap(); panic!("poison!"); }).join();
    match m.lock() {
        Ok(v) => println!("Ok: {v}"),
        Err(p) => println!("Recovered: {}", p.into_inner()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn increments_correctly() { assert_eq!(sync_mutex_demo(), 10); }
    #[test] fn basic_lock() { let m = Mutex::new(0); *m.lock().unwrap() = 42; assert_eq!(*m.lock().unwrap(), 42); }
    #[test] fn contention() {
        let m = Arc::new(Mutex::new(0));
        let hs: Vec<_> = (0..100).map(|_| { let m = Arc::clone(&m); thread::spawn(move || { *m.lock().unwrap() += 1; }) }).collect();
        for h in hs { h.join().unwrap(); }
        assert_eq!(*m.lock().unwrap(), 100);
    }
}
