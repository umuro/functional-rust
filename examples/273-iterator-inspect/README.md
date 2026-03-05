📖 **[View on hightechmind.io →](https://hightechmind.io/rust/273-iterator-inspect)**

---

# 273: Iterator inspect()

**Difficulty:** 1  **Level:** Beginner

Insert a side-effect observation point anywhere in an iterator chain without altering the values or breaking the pipeline.

## The Problem This Solves

You have a multi-step iterator chain — filter, map, take — and something's wrong. You don't know whether the bug is in the filter predicate or the map transformation. To debug it, you'd have to break the chain apart, store intermediates in variables, and add print statements, then reassemble the pipeline once you're done. That's tedious and creates noise.

`inspect` solves this by letting you tap into the pipeline *in place*. Add `.inspect(|x| println!("{x:?}"))` between any two steps and you see exactly what's flowing through at that point — without changing the data, without restructuring the code, and without removing the surrounding adapters.

This is the `.tap()` pattern from Haskell and RxJS: observe without consuming. In production code, `inspect` also sees use for logging, metrics counting, and tracing — any case where you want to observe elements as they flow through without affecting them.

## The Intuition

Pass each element through unchanged while calling a side-effect closure on it — like a one-way mirror inserted into the pipeline.

## How It Works in Rust

```rust
let result: Vec<i32> = (1..=10)
    .inspect(|x| print!("[in:{x}] "))      // see all elements entering
    .filter(|x| x % 2 == 0)
    .inspect(|x| print!("[even:{x}] "))    // see only elements that passed filter
    .map(|x| x * x)
    .collect();
// prints: [in:1] [in:2] [even:2] [in:3] [in:4] [even:4] ...

// Count elements at each pipeline stage
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
let count_in  = AtomicUsize::new(0);
let count_out = AtomicUsize::new(0);

let filtered: Vec<i32> = (1..=20)
    .inspect(|_| { count_in.fetch_add(1, SeqCst); })
    .filter(|x| x % 3 == 0)
    .inspect(|_| { count_out.fetch_add(1, SeqCst); })
    .collect();
// count_in: 20, count_out: 6
```

`inspect` receives a shared reference `&T` — it can observe but not modify. The closure runs lazily, only when the element is pulled through.

## What This Unlocks

- **Pipeline debugging:** Insert print statements between any two adapters without restructuring the chain — remove or leave them when done.
- **Metrics and logging:** Count elements entering each stage, log suspicious values, trace which elements reach the end.
- **Testing internal behavior:** Use `inspect` with a `Vec::push` in tests to verify what flows between stages without exposing intermediate state.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Equivalent | Manual `tap` wrapper function | `.inspect(f)` built into stdlib |
| Side effects in pipelines | Generally avoided | Accepted for debug/logging |
| Where in chain | Anywhere (manual wrap) | Anywhere — just insert |
| Modifies values | No (must return value) | No (receives `&T`, can't mutate) |
| Production use | Typically removed | Sometimes kept for logging |
