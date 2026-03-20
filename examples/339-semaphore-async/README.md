📖 **[View on hightechmind.io →](https://hightechmind.io/rust/339-semaphore-async)**

---

# 339: Semaphore — Controlling Concurrency
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Connection pools, rate limiters, and resource-bounded operations need to limit concurrent access to at most N simultaneous operations. A mutex limits to 1; a semaphore limits to N. Common uses: limit concurrent HTTP requests to 10 to avoid overwhelming a third-party API, limit concurrent database connections to the pool size, or limit parallel file reads to avoid exhausting file descriptors.

## Learning Outcomes

- Implement a counting semaphore using `Mutex<usize>` + `Condvar` for signaling
- Use `acquire()` to block when the permit count reaches zero
- Use `release()` to increment the count and signal waiting tasks
- Recognize `tokio::sync::Semaphore` as the async-aware production alternative

## Rust Application

A semaphore using condvar for efficient waiting:

```rust
pub struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    pub fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cond.wait(count).unwrap();  // Block until released
        }
        *count -= 1;
    }

    pub fn release(&self) {
        *self.count.lock().unwrap() += 1;
        self.cond.notify_one();  // Wake one waiting acquirer
    }
}
```

## OCaml Approach

OCaml implements semaphores with `Mutex` + `Condition` similarly:

```ocaml
type semaphore = { mutable count: int; mutex: Mutex.t; cond: Condition.t }

let acquire s =
  Mutex.lock s.mutex;
  while s.count = 0 do Condition.wait s.cond s.mutex done;
  s.count <- s.count - 1;
  Mutex.unlock s.mutex
```

## Key Differences

1. **Condvar pattern**: Both Rust and OCaml use mutex + condition variable for efficient blocking — the same POSIX pattern.
2. **RAII acquisition**: `SemaphorePermit` (from `tokio::sync::Semaphore`) uses RAII — release happens automatically on drop; manual semaphores require explicit release.
3. **tokio::sync::Semaphore**: The production Tokio semaphore uses an async-aware permit system — `acquire().await` yields instead of blocking the thread.
4. **Rate limiting**: Combine with timing: `acquire()`, perform operation, `release()` after a minimum time — implements rate limiting.

## Exercises

1. Use a semaphore to implement a connection pool: limit concurrent "connections" to a fixed pool size, queuing requests beyond that.
2. Implement a rate limiter using a semaphore + scheduled release: acquire before each API call, release automatically after 1 second.
3. Measure the throughput difference between unlimited concurrent workers and workers limited to N by a semaphore.
