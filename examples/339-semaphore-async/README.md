📖 **[View on hightechmind.io →](https://hightechmind.io/rust/339-semaphore-async)**

---

# 339: Semaphore Async

**Difficulty:** 3  **Level:** Advanced

Limit how many tasks run concurrently — rate limiting, connection pool sizing, and throttling.

## The Problem This Solves

A mutex allows exactly one. A semaphore allows N concurrent operations. Classic examples: connection pools (max 20 queries), HTTP throttling (10 parallel requests), file handle limits.

## The Intuition

Like a parking lot with N spaces. You wait at the entrance if full. When a car leaves, the next enters. Never more than N cars.

```python
sem = asyncio.Semaphore(3)
async with sem:
    await do_work()
```

## How It Works in Rust

```rust
struct Semaphore {
    count: Mutex<usize>,
    cond: Condvar,
}

impl Semaphore {
    fn acquire(&self) {
        let mut c = self.count.lock().unwrap();
        while *c == 0 {
            c = self.cond.wait(c).unwrap();
        }
        *c -= 1;
    }

    fn release(&self) {
        *self.count.lock().unwrap() += 1;
        self.cond.notify_one();
    }
}

// RAII permit: automatically released when dropped
struct Permit<'a>(&'a Semaphore);
impl Drop for Permit<'_> { fn drop(&mut self) { self.0.release(); } }
```

Usage: `let _permit = sem.permit();` — auto-releases at scope end.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Semaphore | No stdlib | Manual or `tokio::sync::Semaphore` |
| RAII permit | Manual | `Permit` with `Drop` |
| Wait mechanism | Condition.wait | `Condvar::wait` |
