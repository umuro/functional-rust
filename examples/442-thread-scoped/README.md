# 442: Scoped Threads — Borrow Stack Data Across Threads

**Difficulty:** 3  **Level:** Intermediate

Use `thread::scope` to spawn threads that borrow local data directly — no `Arc`, no cloning, no heap allocation.

## The Problem This Solves

`thread::spawn` requires a `'static` closure: everything the thread touches must live for the entire program lifetime or be owned by the thread. That rules out the most natural pattern — passing a reference to a local vector, slice, or string into a thread for parallel processing. The workaround is `Arc`, but Arc means heap allocation, atomic reference counting, and boilerplate cloning everywhere.

The deeper problem is safety. The compiler's `'static` requirement exists because a spawned thread can outlive the scope that created it. If you could borrow local data freely, the thread might read freed memory after the enclosing function returns. Early Rust APIs (including the infamous `std::thread::scoped` that was removed for soundness) got this wrong.

`thread::scope` (stable since Rust 1.63) solves this correctly: all threads spawned inside the scope are **automatically joined when the scope exits**, before any local variables go out of scope. The compiler knows this, so it allows `&T` borrows inside scoped threads. Zero-copy parallel access to local data becomes safe and trivially expressible.

## The Intuition

Think of `thread::scope` as a structured parallel block. You enter it, spawn as many threads as you want — passing plain `&T` references to local data — and when the block ends, all threads are joined. No thread can escape the block. That guarantee is what makes borrowing safe.

In Java or Python, parallelising a local list means copying it or wrapping it in a shared structure. In Go, slices are reference types so you can pass them, but the race detector is the only thing stopping concurrent writes. In Rust, `thread::scope` makes the borrow checker do the work: a `&mut` slice can only go to one thread; `&` slices can go to many.

## How It Works in Rust

```rust
use std::thread;

fn parallel_sum(data: &[i64]) -> i64 {
    let (left, right) = data.split_at(data.len() / 2);
    let mut ls = 0i64;
    let mut rs = 0i64;

    thread::scope(|s| {
        // s.spawn accepts &T borrows — no 'static required
        let t1 = s.spawn(|| left.iter().sum::<i64>());
        let t2 = s.spawn(|| right.iter().sum::<i64>());
        // Both threads are joined when scope exits at `}`
        ls = t1.join().unwrap();
        rs = t2.join().unwrap();
    }); // <-- guaranteed join point

    ls + rs
}

// Borrow a local String without Arc
let message = String::from("hello from stack");
thread::scope(|s| {
    s.spawn(|| println!("{}", message));       // shared &String
    s.spawn(|| println!("len={}", message.len())); // fine — both read-only
});
// message is still owned here — no move needed
```

The closure passed to `thread::scope` receives a `&Scope<'_>` that ties thread lifetimes to the surrounding scope. The compiler uses this to approve borrows that would otherwise be rejected.

## What This Unlocks

- **Zero-copy data parallelism** — split a slice, process halves in parallel, merge results without any heap allocation or Arc overhead.
- **Parallel iteration over local collections** — process a `Vec<T>` in chunks across N threads, where each thread borrows its chunk directly.
- **Simpler parallel code** — eliminate `Arc::clone`, `Mutex`, and `.to_owned()` calls that exist only to satisfy `'static` bounds.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrow data in thread | unsafe — GC manages heap, not stack | `s.spawn(\|\| use_local_ref(&data))` |
| Thread lifetime | unbounded by default | bounded to scope — auto-joined on exit |
| Stack slice sharing | array copy usually needed | `&[T]` directly — zero-copy |
| Mutable split | manual coordination | `split_at_mut` — borrow checker enforces disjointness |
| `'static` requirement | N/A | lifted inside `thread::scope` |
