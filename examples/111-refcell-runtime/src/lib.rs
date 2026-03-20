#![allow(clippy::all)]
// Example 111: RefCell<T> — Runtime Borrow Checking
//
// RefCell<T> enforces Rust's borrowing rules at runtime instead of compile time,
// enabling interior mutability for non-Copy types.
//
// Rule: either multiple Ref<T> OR one RefMut<T> — never both.
// Violation = panic at runtime (same rule as borrow checker, just deferred).

use std::cell::RefCell;

// ---------------------------------------------------------------------------
// Approach 1: Interior mutability — mutate through a shared reference
//
// The `items` binding is immutable, but the Vec inside can be mutated.
// Useful when you need to share a collector across callbacks or closures
// without making the binding itself `mut`.
// ---------------------------------------------------------------------------
pub fn collect_items() -> Vec<String> {
    // Immutable binding — the RefCell *is* the mutability
    let items: RefCell<Vec<String>> = RefCell::new(Vec::new());

    // Each borrow_mut() returns a RefMut guard; it releases when dropped
    items.borrow_mut().push("first".to_string());
    items.borrow_mut().push("second".to_string());
    items.borrow_mut().push("third".to_string());

    // borrow() returns a shared Ref guard; bind before returning to avoid lifetime issue
    let borrowed = items.borrow();
    borrowed.clone()
}

// ---------------------------------------------------------------------------
// Approach 2: Shared mutable stack
//
// Stack<T> stores its data in a RefCell so push/pop can take &self
// instead of &mut self — multiple owners can share one Stack via Rc<Stack<T>>.
// ---------------------------------------------------------------------------
pub struct Stack<T> {
    data: RefCell<Vec<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, value: T) {
        self.data.borrow_mut().push(value);
    }

    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }

    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        self.data.borrow().last().cloned()
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Approach 3: Shared observer — multiple immutable handles, one mutable log
//
// Demonstrates why RefCell is indispensable: an observer that records events
// while appearing immutable to the subjects it watches.
// ---------------------------------------------------------------------------
pub struct EventLog {
    events: RefCell<Vec<String>>,
}

impl EventLog {
    pub fn new() -> Self {
        EventLog {
            events: RefCell::new(Vec::new()),
        }
    }

    // Takes &self — caller sees an immutable observer
    pub fn record(&self, event: &str) {
        self.events.borrow_mut().push(event.to_string());
    }

    pub fn entries(&self) -> Vec<String> {
        self.events.borrow().clone()
    }

    pub fn count(&self) -> usize {
        self.events.borrow().len()
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helper: demonstrate try_borrow to avoid panics
// ---------------------------------------------------------------------------
pub fn try_borrow_example() -> Result<usize, String> {
    let cell: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3]);

    // Exclusive borrow held across this scope
    let _writer = cell.borrow_mut();

    // try_borrow returns Err rather than panicking
    // Bind to local to avoid borrow-outlive-local issue
    let result = match cell.try_borrow() {
        Ok(r) => Ok(r.len()),
        Err(e) => Err(format!("borrow failed: {e}")),
    };
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_items_order() {
        let items = collect_items();
        assert_eq!(items, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_collect_items_length() {
        let items = collect_items();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_stack_push_pop() {
        let s: Stack<i32> = Stack::new();
        assert!(s.is_empty());

        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.len(), 3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_stack_peek_does_not_remove() {
        let s: Stack<&str> = Stack::new();
        s.push("hello");
        s.push("world");

        assert_eq!(s.peek(), Some("world"));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_stack_empty_pop() {
        let s: Stack<u8> = Stack::new();
        assert_eq!(s.pop(), None);
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_event_log_records_in_order() {
        let log = EventLog::new();
        log.record("connect");
        log.record("query");
        log.record("disconnect");

        assert_eq!(log.count(), 3);
        assert_eq!(log.entries(), vec!["connect", "query", "disconnect"]);
    }

    #[test]
    fn test_event_log_immutable_receiver() {
        let log = EventLog::new();
        let log_ref: &EventLog = &log;
        log_ref.record("event-a");
        log_ref.record("event-b");
        assert_eq!(log.count(), 2);
    }

    #[test]
    fn test_try_borrow_returns_err_when_mutably_borrowed() {
        let result = try_borrow_example();
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("borrow failed"));
    }

    #[test]
    fn test_multiple_shared_borrows_allowed() {
        let cell: RefCell<Vec<i32>> = RefCell::new(vec![10, 20, 30]);
        let r1 = cell.borrow();
        let r2 = cell.borrow();
        assert_eq!(r1.len(), r2.len());
        assert_eq!(*r1, *r2);
    }
}
