// Auto traits and negative impls in Rust
// Note: negative impls require nightly for custom types.
// This example shows the concept and std library behavior.

use std::cell::Cell;
use std::sync::Arc;
use std::rc::Rc;

// Check Send/Sync at compile time
fn require_send<T: Send>(_: T) {}
fn require_sync<T: Sync>(_: &T) {}

// Custom type: unsafe impl to assert thread safety
struct MyRawBuffer {
    // Contains a raw pointer — not automatically Send
    ptr: *mut u8,
    len: usize,
}

// We know this is safe because we manage access properly
unsafe impl Send for MyRawBuffer {}
unsafe impl Sync for MyRawBuffer {}

// A type that should NOT be Send (contains Cell)
struct NonSendable {
    // Cell<T> is !Sync because it allows interior mutability without locks
    data: Cell<i32>,
}
// NonSendable is automatically !Sync because Cell<i32> is !Sync
// If we tried: require_sync(&NonSendable { data: Cell::new(0) }), it would fail.

fn demonstrate_auto_traits() {
    // i32: Send + Sync (automatically)
    require_send(42i32);
    require_sync(&42i32);

    // Arc<i32>: Send + Sync (thread-safe reference counting)
    let arc = Arc::new(42i32);
    require_send(arc.clone());
    require_sync(&arc);

    // Rc<i32>: !Send (non-atomic ref counting)
    // This would NOT compile:
    // let rc = Rc::new(42i32);
    // require_send(rc); // ERROR: Rc<i32> cannot be sent between threads safely

    println!("Auto trait checks passed!");
}

fn demonstrate_propagation() {
    // A struct is Send iff all fields are Send
    struct AllSend { x: i32, y: String, z: Arc<f64> }
    // AllSend is automatically Send

    struct HasRc { x: i32, rc: Rc<i32> }
    // HasRc is automatically !Send because Rc<i32> is !Send

    require_send(AllSend { x: 1, y: "hi".to_string(), z: Arc::new(1.0) });
    // require_send(HasRc { x: 1, rc: Rc::new(1) }); // Would fail!
    println!("Propagation: struct is Send iff all fields are Send");
}

fn main() {
    demonstrate_auto_traits();
    demonstrate_propagation();

    // MyRawBuffer is Send due to unsafe impl
    let buf = MyRawBuffer { ptr: std::ptr::null_mut(), len: 0 };
    require_send(buf);
    println!("MyRawBuffer: unsafe impl Send works");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_not_send<T>() where T: ?Sized {} // just to document

    #[test]
    fn test_send_types() {
        assert_send::<i32>();
        assert_send::<String>();
        assert_send::<Arc<i32>>();
        assert_send::<MyRawBuffer>();
    }

    #[test]
    fn test_sync_types() {
        assert_sync::<i32>();
        assert_sync::<Arc<i32>>();
    }
}
