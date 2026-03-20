**Difficulty:** ⭐  
**Category:** Functional Programming  

[semaphore on hightechmind.io](https://hightechmind.io/posts/functional-rust/semaphore)

---

## Problem Statement

Implement a counting semaphore using `Mutex<usize>` + `Condvar`. The semaphore limits the number of concurrent operations: `acquire` blocks when the count reaches zero; `release` increments the count and wakes a waiting thread. Provide a RAII `with_permit` helper that acquires and releases automatically.

## Learning Outcomes

- Implement `Semaphore { count: Mutex<usize>, cond: Condvar, max: usize }`
- Implement `acquire` using `Condvar::wait(guard)` in a `while *count == 0` loop (spurious wakeup protection)
- Implement `release` that increments count and calls `Condvar::notify_one()`
- Implement `with_permit<T, F: FnOnce() -> T>(&self, f: F) -> T` for RAII acquire/release
- Understand the concurrency pattern: semaphore(1) = mutex, semaphore(N) = N-slot license

## Rust Application

```rust
struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
    max: usize,
}

impl Semaphore {
    fn new(n: usize) -> Self {
        Semaphore { count: Mutex::new(n), cond: Condvar::new(), max: n }
    }

    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        if *count < self.max {
            *count += 1;
            self.cond.notify_one();
        }
    }

    fn with_permit<T, F: FnOnce() -> T>(&self, f: F) -> T {
        self.acquire();
        let result = f();
        self.release();
        result
    }
}
```

`Condvar::wait(guard)` atomically releases the mutex and parks the thread. When `notify_one` is called, the thread re-acquires the mutex and re-checks the condition. The `while *count == 0` loop guards against spurious wakeups — the OS may wake a thread without an actual `notify`.

`with_permit` uses RAII semantics but manually — it acquires before `f()` and releases after. For full RAII (panic safety), wrap in a guard struct with `Drop`. The current implementation drops the permit even if `f()` panics due to Rust's stack unwinding — the `release` call runs before `with_permit` returns.

## OCaml Approach

```ocaml
type t = {
  mutable count: int;
  max: int;
  mutex: Mutex.t;
  cond: Condition.t;
}

let create n = { count = n; max = n; mutex = Mutex.create (); cond = Condition.create () }

let acquire s =
  Mutex.lock s.mutex;
  while s.count = 0 do
    Condition.wait s.cond s.mutex
  done;
  s.count <- s.count - 1;
  Mutex.unlock s.mutex

let release s =
  Mutex.lock s.mutex;
  if s.count < s.max then begin
    s.count <- s.count + 1;
    Condition.signal s.cond
  end;
  Mutex.unlock s.mutex

let with_permit s f =
  acquire s;
  Fun.protect ~finally:(fun () -> release s) f
```

OCaml's `Condition.wait` is the direct analog of `Condvar::wait`. `Fun.protect ~finally` ensures `release` runs even if `f` raises an exception — the OCaml equivalent of Rust's drop-on-unwind.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Condition variable | `Condvar` — paired with `Mutex` | `Condition.t` — paired with `Mutex.t` |
| Spurious wakeup | `while *count == 0` loop | `while count = 0 do ... done` loop |
| RAII acquire | `with_permit` method | `Fun.protect ~finally` |
| Atomic wait+sleep | `cond.wait(guard)` — releases mutex atomically | `Condition.wait cond mutex` |

Counting semaphores model "N concurrent resources" — N database connections, N parallel downloads, N worker slots. `tokio::sync::Semaphore` provides the async equivalent for non-blocking workloads.

## Exercises

1. Add a `try_acquire() -> bool` that returns immediately without blocking.
2. Add `acquire_timeout(duration: Duration) -> bool` using `Condvar::wait_timeout`.
3. Implement a RAII guard `SemaphoreGuard` with `Drop` that calls `release` — ensuring release even on panic.
4. Use the semaphore to limit concurrent HTTP requests: spawn 20 threads, but only allow 5 to run simultaneously.
5. Verify that `semaphore(1)` behaves identically to `Mutex` for exclusive access.
