//! # 553. Self-Referential Structs (Pin)
//! Using Pin to enable self-referential data structures safely.

use std::pin::Pin;
use std::marker::PhantomPinned;

/// A self-referential struct — holds a pointer to its own `value` field
/// This is the canonical example for why Pin is needed
struct SelfRef {
    value: String,
    // SAFETY: this pointer is valid only while the struct is pinned
    self_ptr: *const String,
    _pin: PhantomPinned, // marks as !Unpin — cannot be moved
}

impl SelfRef {
    /// SAFETY: must pin before using self_ptr
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRef {
            value: s.to_string(),
            self_ptr: std::ptr::null(), // placeholder
            _pin: PhantomPinned,
        });

        // Now safe to set up the self-pointer (value is pinned)
        let ptr: *const String = &boxed.value;
        unsafe {
            // Get mutable access to the pinned value
            let this = boxed.as_mut().get_unchecked_mut();
            this.self_ptr = ptr;
        }

        boxed
    }

    fn get_value<'a>(self: Pin<&'a Self>) -> &'a str {
        // Safety: accessing value field is safe — Pin prevents movement
        unsafe { &self.get_ref().value }
    }

    fn get_value_via_ptr<'a>(self: Pin<&'a Self>) -> &'a str {
        // Safety: self_ptr is valid because self is pinned and value hasn't moved
        unsafe { &*self.get_ref().self_ptr }
    }
}

/// Simpler self-referential with an index (safe alternative)
struct SafeSelfRef {
    data: Vec<String>,
    current_index: usize, // "reference" via index — always valid after moves
}

impl SafeSelfRef {
    fn new(items: Vec<String>) -> Self {
        SafeSelfRef { data: items, current_index: 0 }
    }

    fn current(&self) -> &str {
        &self.data[self.current_index]
    }

    fn advance(&mut self) {
        self.current_index = (self.current_index + 1) % self.data.len();
    }
}

/// Pinning in async context (conceptual)
async fn async_self_ref_demo() {
    // Async state machines are self-referential — Pin makes them safe
    // Simulating with a simple future
    let x = 42;
    let y = &x; // y references x within the same async frame
    println!("async ref: {}", y);
    // When this future is polled, x and y must stay in place — Pin ensures this
}

fn main() {
    // Self-referential struct via Pin
    let pinned = SelfRef::new("Hello, Pin!");
    println!("via field: {}", pinned.as_ref().get_value());
    println!("via ptr:   {}", pinned.as_ref().get_value_via_ptr());

    // Demonstrate that moving is prevented:
    // let moved = *pinned; // ERROR: Pin<Box<T>> where T: !Unpin can't be dereffed to move

    // Safe index-based alternative
    let mut safe_ref = SafeSelfRef::new(
        vec!["one".to_string(), "two".to_string(), "three".to_string()]
    );
    println!("\nSafe self-ref:");
    for _ in 0..4 {
        print!("{} ", safe_ref.current());
        safe_ref.advance();
    }
    println!();

    // safe_ref can be moved — index-based refs are always valid
    let moved_ref = safe_ref;
    println!("after move: {}", moved_ref.current());

    // Pinning a primitive type (Unpin — can still move)
    let x = 42i32;
    let pinned_int = Pin::new(&x);
    println!("\npinned i32: {}", *pinned_int); // i32: Unpin, so Pin<&i32> is not very restrictive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_self_ref() {
        let p = SelfRef::new("test");
        assert_eq!(p.as_ref().get_value(), "test");
        assert_eq!(p.as_ref().get_value_via_ptr(), "test");
    }

    #[test]
    fn test_safe_self_ref() {
        let mut r = SafeSelfRef::new(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(r.current(), "a");
        r.advance();
        assert_eq!(r.current(), "b");
        r.advance();
        assert_eq!(r.current(), "a"); // wraps
    }
}
