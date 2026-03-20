📖 **[View on hightechmind.io →](https://hightechmind.io/rust/343-producer-consumer)**

---

# 343: Producer-Consumer Pattern

## Problem Statement

When data is produced at a variable rate and consumed at a different variable rate, a bounded buffer between them smooths out the mismatch — producers slow down when the buffer is full (backpressure), consumers wait when it's empty. This pattern, formalized by Dijkstra (1965) as the "bounded buffer problem," underlies logging pipelines, work queues, streaming data processing, and I/O scheduling. Without a bounded buffer, fast producers can exhaust memory; without the blocking discipline, consumers busy-wait and waste CPU. The Rust implementation uses `Mutex` + `Condvar` to achieve efficient blocking on both conditions.

## Learning Outcomes

- Implement a bounded buffer using `Mutex<VecDeque<T>>` + two `Condvar` variables
- Use `Condvar::wait()` to block a thread until a condition becomes true
- Use `Condvar::notify_one()` to wake exactly one waiting thread after a state change
- Wrap the buffer in `Arc` to share it between producer and consumer threads
- Understand why two `Condvar`s are needed: one for "not empty", one for "not full"
- Compare this to channel-based producer-consumer with `mpsc::sync_channel`

## Rust Application

```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct BoundedBuffer<T> {
    data: Mutex<VecDeque<T>>,
    capacity: usize,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            data: Mutex::new(VecDeque::new()),
            capacity,
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        })
    }

    pub fn put(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        while data.len() >= self.capacity {
            data = self.not_full.wait(data).unwrap(); // blocks, releases lock
        }
        data.push_back(item);
        self.not_empty.notify_one();
    }

    pub fn take(&self) -> T {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = self.not_empty.wait(data).unwrap();
        }
        let item = data.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }
}
```

`Condvar::wait` atomically releases the mutex and suspends the thread. It returns the re-acquired lock guard. The `while` loop (not `if`) guards against spurious wakeups — POSIX allows `wait` to return without notification; always re-check the condition.

## OCaml Approach

OCaml's `Mutex` + `Condition` maps directly:

```ocaml
type 'a buffer = {
  data: 'a Queue.t;
  capacity: int;
  not_empty: Condition.t;
  not_full: Condition.t;
  mutex: Mutex.t;
}

let put buf item =
  Mutex.lock buf.mutex;
  while Queue.length buf.data >= buf.capacity do
    Condition.wait buf.not_full buf.mutex
  done;
  Queue.push item buf.data;
  Condition.signal buf.not_empty;
  Mutex.unlock buf.mutex
```

The structure is identical: lock, check condition in a loop, wait (releases lock), modify, signal, unlock. OCaml's `Condition.signal` is equivalent to Rust's `notify_one`.

## Key Differences

| Aspect | Rust `Condvar` | OCaml `Condition` |
|--------|---------------|-------------------|
| API style | Method on `Condvar`, takes guard | Free function, takes mutex |
| Guard integration | `wait` takes and returns `MutexGuard` | Separate `lock`/`unlock` calls |
| Spurious wakeup | Must use `while` loop | Must use `while` loop |
| Simpler alternative | `mpsc::sync_channel(capacity)` | `Event.channel` (synchronous) |
| Type safety | `T: Send` required for cross-thread | Polymorphic, GC handles it |

## Exercises

1. **Multi-producer multi-consumer**: Extend the example to support 3 producers and 2 consumers; verify that all items are consumed exactly once using a `HashSet` to track them.
2. **Channel equivalent**: Implement the same bounded producer-consumer using `mpsc::sync_channel(capacity)` instead of `Condvar`; compare code complexity and performance.
3. **Poison pill shutdown**: Add a sentinel value (e.g., `Option<T>` where `None` signals shutdown) so producers can signal consumers to stop cleanly; implement and test with a finite work queue.
