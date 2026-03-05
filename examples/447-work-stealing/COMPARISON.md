# OCaml vs Rust: Work Stealing

## Basic Concept

Work stealing: each worker has its own queue. When a worker's queue is empty,
it "steals" work from another worker's queue (from the back).

### OCaml (Simplified shared queue)
```ocaml
let deque = ref []
let mutex = Mutex.create ()

let steal () =
  Mutex.lock mutex;
  let r = match !deque with
    | [] -> None
    | x::rest -> deque := rest; Some x
  in
  Mutex.unlock mutex; r
```

### Rust
```rust
type Queue = Arc<Mutex<VecDeque<u32>>>;

fn worker(own: Queue, others: Vec<Queue>) {
    loop {
        // Own queue: pop from front
        if let Some(job) = own.lock().unwrap().pop_front() {
            process(job);
            continue;
        }
        // Steal: pop from back of others
        for q in &others {
            if let Ok(mut g) = q.try_lock() {
                if let Some(job) = g.pop_back() {
                    process(job);
                    break;
                }
            }
        }
    }
}
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Per-worker queue | No (shared) | Yes (VecDeque per worker) |
| Pop strategy | Front only | Front (own) / Back (steal) |
| Lock contention | High (shared) | Low (try_lock + skip) |
| Data structure | List | `VecDeque` (double-ended) |

## Why Pop from Back?

```
Worker 1's queue: [A, B, C, D, E]
                   ^           ^
                   |           |
               own pop      steal pop
              (front)       (back)

- Worker 1 processes A, B, C...
- Worker 2 steals E, D...
- Less contention: both ends accessed
```

## Non-blocking Steal

### OCaml
```ocaml
(* No try_lock in stdlib — must block *)
Mutex.lock mutex;
(* ... *)
Mutex.unlock mutex
```

### Rust
```rust
// try_lock: don't wait if locked
if let Ok(mut guard) = other_queue.try_lock() {
    if let Some(job) = guard.pop_back() {
        // Got work
    }
}
// If locked, skip to next queue
```
