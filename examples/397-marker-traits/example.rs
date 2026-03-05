// Marker traits: Send, Sync, Copy in Rust
use std::sync::{Arc, Mutex};
use std::thread;

// Custom marker trait
trait ThreadSafe: Send + Sync {}
// Blanket: automatically implement for all Send + Sync types
impl<T: Send + Sync> ThreadSafe for T {}

// A Send type: can be moved to another thread
#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }
// Point is automatically Send + Sync + Copy (all fields are f64)

// A !Send type (simulated — real Rc is !Send)
// Rc<T> is !Send because it has non-atomic ref counting
// std::rc::Rc<i32> would not be Send

// Demonstrate Copy semantics
fn demonstrate_copy() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy — p1 is still valid!
    let p3 = p1; // Can copy again
    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);
}

// Demonstrate Send across threads
fn demonstrate_send() {
    let p = Point { x: 3.0, y: 4.0 };
    let handle = thread::spawn(move || {
        println!("In thread: {:?}", p);
        p.x + p.y
    });
    let result = handle.join().unwrap();
    println!("Thread result: {}", result);
}

// Demonstrate Sync: share reference across threads
fn demonstrate_sync() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut v = data_clone.lock().unwrap();
        v.push(4);
    });
    handle.join().unwrap();
    println!("Shared data: {:?}", data.lock().unwrap());
}

// Custom non-Copy type
struct NoCopy(String);
// NoCopy is Send but not Copy (String is not Copy)

fn main() {
    demonstrate_copy();
    demonstrate_send();
    demonstrate_sync();

    // Size is Copy:
    let s1 = 42u32;
    let s2 = s1; // Copy
    println!("s1={}, s2={}", s1, s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1;
        assert_eq!(p1.x, p2.x); // p1 still valid
    }

    #[test]
    fn test_send() {
        let p = Point { x: 0.0, y: 0.0 };
        let h = thread::spawn(move || p);
        let result = h.join().unwrap();
        assert_eq!(result.x, 0.0);
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn test_marker_bounds() {
        assert_send::<Point>();
        assert_sync::<Point>();
        assert_send::<i32>();
    }
}
