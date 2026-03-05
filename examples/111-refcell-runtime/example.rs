// Example 111: RefCell<T> — Runtime Borrow Checking
//
// RefCell<T> enforces Rust's borrowing rules at runtime instead of compile time.
// It enables interior mutability: mutating through a shared reference.
// Panics if you violate the rule: you cannot have both &mut and & at the same time.

use std::cell::RefCell;

// ─── Approach 1: Interior mutability ─────────────────────────────────────────
pub fn collect_items() -> Vec<String> {
    let items: RefCell<Vec<String>> = RefCell::new(Vec::new());
    items.borrow_mut().push("first".to_string());
    items.borrow_mut().push("second".to_string());
    items.borrow_mut().push("third".to_string());
    let borrowed = items.borrow();
    borrowed.clone()
}

// ─── Approach 2: Interior-mutable Stack ──────────────────────────────────────
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

// ─── Approach 3: Event logger — immutable receiver, mutable interior ─────────
pub struct EventLog {
    events: RefCell<Vec<String>>,
}

impl EventLog {
    pub fn new() -> Self {
        EventLog {
            events: RefCell::new(Vec::new()),
        }
    }
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

fn main() {
    // Approach 1
    let items = collect_items();
    println!("collect_items() = {:?}", items);

    // Approach 2
    let stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("stack peek = {:?}", stack.peek());
    println!("stack pop  = {:?}", stack.pop());
    println!("stack len  = {}", stack.len());

    // Approach 3
    let log = EventLog::new();
    log.record("connect");
    log.record("query");
    log.record("disconnect");
    println!("log entries = {:?}", log.entries());
    println!("log count   = {}", log.count());

    // try_borrow — safe fallback
    let cell: RefCell<i32> = RefCell::new(42);
    let _writer = cell.borrow_mut();
    match cell.try_borrow() {
        Ok(v) => println!("got: {}", *v),
        Err(e) => println!("try_borrow failed (expected): {e}"),
    }
}

/* Output:
   collect_items() = ["first", "second", "third"]
   stack peek = Some(30)
   stack pop  = Some(30)
   stack len  = 2
   log entries = ["connect", "query", "disconnect"]
   log count   = 3
   try_borrow failed (expected): already mutably borrowed
*/
