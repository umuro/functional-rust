#![allow(clippy::all)]
//! # Closure Move Semantics — Ownership Transfer

use std::thread;

/// Move closure for threads
pub fn spawn_with_data(data: Vec<i32>) -> thread::JoinHandle<i32> {
    thread::spawn(move || {
        data.iter().sum() // data moved into closure
    })
}

/// Move individual values
pub fn move_multiple() -> impl FnOnce() -> (String, Vec<i32>) {
    let s = String::from("hello");
    let v = vec![1, 2, 3];

    move || (s, v) // Both moved
}

/// Partial move
pub fn partial_move() {
    let data = (String::from("hello"), 42);

    let f = move || {
        let (s, n) = data; // Takes ownership of both
        println!("{} {}", s, n);
    };

    f();
}

/// Clone before move
pub fn clone_then_move(s: String) -> (impl Fn() -> usize, String) {
    let cloned = s.clone();
    let f = move || cloned.len();
    (f, s) // Return both the closure and original
}

/// Force move with move keyword
pub fn force_move() -> impl Fn() -> i32 {
    let x = 42;
    move || x // x is Copy, but move forces ownership transfer semantics
}

/// Move into async block (conceptual)
pub fn move_for_async_like() -> impl FnOnce() -> String {
    let data = String::from("async data");
    move || {
        // Simulates async - data must be owned
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_with_data() {
        let data = vec![1, 2, 3, 4, 5];
        let handle = spawn_with_data(data);
        assert_eq!(handle.join().unwrap(), 15);
    }

    #[test]
    fn test_move_multiple() {
        let f = move_multiple();
        let (s, v) = f();
        assert_eq!(s, "hello");
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_clone_then_move() {
        let s = String::from("test");
        let (f, original) = clone_then_move(s);
        assert_eq!(f(), 4);
        assert_eq!(original, "test");
    }

    #[test]
    fn test_force_move() {
        let f = force_move();
        assert_eq!(f(), 42);
        assert_eq!(f(), 42); // Can still call because i32 is Copy
    }

    #[test]
    fn test_async_like() {
        let f = move_for_async_like();
        assert_eq!(f(), "async data");
    }
}
