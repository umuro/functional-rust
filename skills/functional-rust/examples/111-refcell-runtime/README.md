# 111: RefCell\<T\> — Runtime Borrow Checking

**Difficulty:** 2  **Level:** Intermediate

`RefCell<T>` enforces Rust's borrowing rules at runtime instead of compile time — enabling interior mutability for non-`Copy` types when the borrow checker can't see the pattern is safe.

## The Problem This Solves

Sometimes you're writing a data structure where the ownership pattern is correct but the borrow checker can't prove it. A tree traversal that modifies nodes as it visits them. A mock object in tests that records calls made to it while appearing immutable to the code being tested. A recursive data structure where a node needs to reference its parent.

The borrow checker proves safety conservatively — if it can't prove something is safe at compile time, it rejects it. `RefCell<T>` is the escape hatch for cases where you, the programmer, know the borrow rules are respected at runtime even though the compiler can't verify it statically.

Unlike `Cell<T>` (which only works for `Copy` types by copying values), `RefCell<T>` hands out actual references to its interior — but tracks them at runtime. At any point, there can be either multiple `Ref<T>` (shared borrows) or one `RefMut<T>` (exclusive borrow), but not both. If you violate this, the program panics — same rule as the borrow checker, just checked when the borrow happens rather than at compile time.

## The Intuition

`RefCell<T>` moves the borrow checker's "one writer or multiple readers" rule from compile time to runtime — you get interior mutability for any type, paying a small runtime cost and risking a panic if you accidentally break the rule.

## How It Works in Rust

```rust
use std::cell::RefCell;

// Mock logger: must appear immutable to callers, but records internally
struct MockLogger {
    log: RefCell<Vec<String>>,
}

impl MockLogger {
    fn new() -> Self {
        MockLogger { log: RefCell::new(vec![]) }
    }
    
    fn write(&self, msg: &str) { // &self — looks immutable
        self.log.borrow_mut().push(msg.to_string()); // runtime check: get exclusive borrow
    }
    
    fn entries(&self) -> Vec<String> {
        self.log.borrow().clone() // runtime check: get shared borrow
    }
}

fn demo() {
    let logger = MockLogger::new();
    logger.write("first");
    logger.write("second");
    println!("{:?}", logger.entries()); // ["first", "second"]
}

// Runtime panic if borrow rules violated
fn demo_panic() {
    let cell = RefCell::new(vec![1, 2, 3]);
    
    let borrow1 = cell.borrow();      // shared borrow — ok
    let borrow2 = cell.borrow();      // second shared borrow — ok
    // let borrow_mut = cell.borrow_mut(); // PANIC: already borrowed as immutable
    
    drop(borrow1);
    drop(borrow2);
    let mut borrow_mut = cell.borrow_mut(); // now ok — no shared borrows
    borrow_mut.push(4);
}

// try_borrow returns Result instead of panicking
fn safe_borrow(cell: &RefCell<Vec<i32>>) {
    match cell.try_borrow_mut() {
        Ok(mut v) => v.push(99),
        Err(_) => eprintln!("Already borrowed — skip"),
    }
}

// Common pattern: Rc<RefCell<T>> for shared mutable ownership
use std::rc::Rc;
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);
*clone.borrow_mut() += 1;
println!("{}", shared.borrow()); // 1
```

## What This Unlocks

- **Interior mutability for any type** — where `Cell<T>` only works for `Copy` types, `RefCell<T>` works for `String`, `Vec`, or any complex struct.
- **Testing with mock objects** — a mock that records its interactions can implement a trait that takes `&self`, with the recording happening internally.
- **`Rc<RefCell<T>>` pattern** — the standard way to have shared, mutable ownership in single-threaded code when you genuinely need it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable state in immutable context | `ref` type — always available | `RefCell<T>` — explicit opt-in |
| When rules are checked | N/A (GC, no borrow rules) | Compile time (normal) vs runtime (`RefCell`) |
| Violation consequence | N/A | Runtime panic (vs compile error normally) |
| For Copy types | N/A | Prefer `Cell<T>` — lighter, no references |
| Thread safety | N/A | `RefCell` is NOT thread-safe; use `Mutex<T>` for threads |
