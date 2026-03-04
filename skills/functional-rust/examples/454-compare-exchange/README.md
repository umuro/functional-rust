# 454: Compare-and-Swap — Atomic Conditional Updates

**Difficulty:** 3  **Level:** Intermediate

Atomically read, compare, and conditionally update a value — the primitive that underlies every lock-free algorithm.

## The Problem This Solves

`fetch_add` atomically increments. But what if you need to update a value only if it's still what you expect — for example, implementing an atomic maximum, a lock-free stack push, or optimistic concurrency? You need to read the current value, compute the new value, and write it, but only if nothing changed between your read and write.

Without CAS, this is a race: read 5, compute max(5, 7) = 7, but another thread changed it to 9 before you write — you'd overwrite 9 with 7. With CAS: "set to 7, but only if it's still 5". If another thread changed it, CAS fails and returns the actual current value (9), and you retry with the correct starting point. The retry loop converges because at least one thread succeeds per round.

This "optimistic" pattern — try, check if it worked, retry if not — is the basis of all lock-free data structures and algorithms. It avoids locks by turning the race into a correctness check.

## The Intuition

`compare_exchange(expected, new_value, success_ordering, failure_ordering)` does atomically: "if the current value equals `expected`, replace it with `new_value` and return `Ok(expected)`. Otherwise, return `Err(actual_value)`."

The CPU guarantees that between the compare and the exchange, no other thread can sneak in and modify the value. That's the whole mechanism.

There are two variants:
- `compare_exchange` — guaranteed to succeed if the value matches. Use for one-shot checks.
- `compare_exchange_weak` — may spuriously fail even if values match (on LL/SC architectures like ARM). Always use in a retry loop — it's faster on ARM and generates identical code on x86.

## How It Works in Rust

```rust
use std::sync::atomic::{AtomicUsize, AtomicI64, Ordering};

fn cas_increment(a: &AtomicUsize) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        // Try to swap cur for cur+1
        match a.compare_exchange_weak(
            cur,          // expected value
            cur + 1,      // desired value
            Ordering::AcqRel,  // success ordering
            Ordering::Relaxed, // failure ordering (just re-read)
        ) {
            Ok(_)       => break,         // succeeded — we wrote cur+1
            Err(actual) => cur = actual,  // someone else changed it; retry
        }
    }
}

// Atomic maximum — not a built-in operation
fn atomic_max(a: &AtomicI64, v: i64) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        if v <= cur { break; } // our value isn't a new max — nothing to do
        match a.compare_exchange_weak(cur, v, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_)        => break,
            Err(actual)  => cur = actual, // another thread wrote a higher value
        }
    }
}

// Single-shot check (not in a loop) — use compare_exchange
let a = AtomicUsize::new(5);
// Try to set to 0 only if it's 99 — should fail
assert_eq!(a.compare_exchange(99, 0, Ordering::SeqCst, Ordering::SeqCst), Err(5));
```

The failure ordering must be ≤ the success ordering. `AcqRel` on success + `Relaxed` on failure is a common pattern: you need ordering when you succeed (you're publishing a change), but a failure is just a re-read.

## What This Unlocks

- **Lock-free algorithm building block** — push/pop on a lock-free stack, enqueue/dequeue on a lock-free queue — all use CAS loops.
- **Custom atomic operations** — any "read-modify-write" that isn't a built-in (`fetch_add`, `fetch_or`, etc.) can be implemented as a CAS loop.
- **Optimistic concurrency** — database-style "read, compute, update only if unchanged" without any locks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| CAS | `Atomic.compare_and_set a exp des` | `a.compare_exchange(exp, new, succ_ord, fail_ord)` |
| Return value | `bool` | `Result<T, T>` — `Ok(old)` or `Err(actual)` |
| Weak CAS | same as strong in OCaml 5 | `compare_exchange_weak` — may fail spuriously; use in loops |
| Retry idiom | `while not (CAS ...) do re-read` | `loop { match cex_weak { Ok(_) => break, Err(v) => exp = v } }` |
| On failure | re-read manually | failure value returned directly — no extra load needed |
