# 339: Semaphore Async

**Difficulty:** 3  **Level:** Advanced

Limit how many tasks run concurrently — rate limiting, connection pool sizing, and throttling.

## The Problem This Solves

A mutex allows exactly one accessor at a time. But sometimes you want to allow *N* concurrent operations — not one, not unlimited. Classic examples: a database connection pool allows at most 20 concurrent queries; an HTTP client throttles to 10 parallel requests; a file processor limits concurrent file handles to avoid EMFILE errors.

A semaphore holds a counter initialized to N. `acquire()` decrements the counter (blocking if zero); `release()` increments it and wakes a waiting acquirer. At any point, at most N tasks hold a permit. This is the building block for connection pools, rate limiters, and resource guards.

In async Rust, `tokio::sync::Semaphore` provides `acquire().await` — it suspends the task when the count is zero rather than blocking a thread.

## The Intuition

Like a parking lot with N spaces. You wait at the entrance if it's full. When a car leaves, the barrier opens and the next waiting car enters. The lot never has more than N cars — that's the invariant the semaphore enforces.

Python's `asyncio.Semaphore(n)` works identically:
```python
sem = asyncio.Semaphore(3)
async with sem:
    await do_work()
```

## How It Works in Rust

```rust
struct Semaphore {
    count: Mutex<usize>,  // current available permits
    cond: Condvar,        // wakes waiters when a permit is released
}

impl Semaphore {
    fn acquire(&self) {
        let mut c = self.count.lock().unwrap();
        while *c == 0 {
            c = self.cond.wait(c).unwrap();  // sleep until woken
        }
        *c -= 1;  // take a permit
    }

    fn release(&self) {
        *self.count.lock().unwrap() += 1;
        self.cond.notify_one();  // wake one waiter
    }
}

// RAII permit: automatically released when dropped
struct Permit<'a>(&'a Semaphore);
impl<'a> Drop for Permit<'a> { fn drop(&mut self) { self.0.release(); } }

impl Semaphore {
    fn permit(&self) -> Permit<'_> { self.acquire(); Permit(self) }
}
```

Usage: `let _permit = sem.permit();` — the permit is dropped at the end of the scope, releasing the slot automatically. No need to manually call `release()`.

The test verifies `peak_concurrency <= 2` when the semaphore is initialized with 2 — even with 6 competing threads.

## What This Unlocks

- **Connection pools** — cap database or HTTP connections at N; tasks queue up rather than overloading the DB.
- **Rate limiting** — allow at most N API calls in flight at any time (complement with a timer-based refill for req/sec limits).
- **Parallel test isolation** — run at most N integration tests in parallel to avoid port conflicts or I/O saturation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Semaphore | No stdlib semaphore; use `Lwt_pool` or Mutex + condition | `std::sync` (manual) or `tokio::sync::Semaphore` |
| RAII permit | Manual acquire/release | `Permit` struct with `Drop` impl |
| Wait mechanism | `Condition.wait` / Lwt scheduler | `Condvar::wait` (sync) / `.await` (async) |
| Async version | `Lwt_pool.use_` | `semaphore.acquire().await` |
