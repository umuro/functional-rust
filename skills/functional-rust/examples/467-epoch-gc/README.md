# 467: Epoch-Based Garbage Collection

**Difficulty:** 3  **Level:** Intermediate

Safe memory reclamation for lock-free data structures — no GC, no reference counting overhead.

## The Problem This Solves

Lock-free data structures avoid mutexes, but they create a hard memory problem: you can't free a node the moment you unlink it, because another thread might still hold a pointer to it. With a GC language this is automatic. In Rust, with raw `AtomicPtr`, you need to know *when* it's safe to deallocate.

Reference counting (`Arc`) works but adds atomic increments/decrements to every read — exactly the overhead you avoided by going lock-free.

Epoch-based reclamation (EBR) is the industry solution: threads declare when they're *inside* a critical section by pinning the current epoch. A retired object is only freed when the global epoch has advanced past every thread that could have seen it. Read-side cost is a single atomic load — almost free.

## The Intuition

Imagine a library that recycles returned books. You can only recycle a book once every borrower from *that checkout period* has returned it. The library uses semester numbers (epochs): when a new semester starts and all old borrowers are gone, books from two semesters ago can be safely recycled.

Threads "check in" to the current epoch when they start a read operation and "check out" when done. Retired objects wait until all threads have advanced past the epoch where the object was retired.

## How It Works in Rust

1. **Global epoch counter** — a shared `AtomicUsize` advances periodically.
2. **Thread-local state** — each thread tracks its current pinned epoch.
3. **Pin on enter**:
   ```rust
   let guard = epoch::pin(); // from crossbeam-epoch
   // now safe to read lock-free structures
   ```
4. **Defer deallocation**:
   ```rust
   guard.defer_destroy(ptr); // runs when epoch advances past current
   ```
5. **Epoch advance** — crossbeam-epoch advances the global epoch when all threads have moved on; deferred destructors run.

The `crossbeam-epoch` crate implements this with carefully tuned memory orderings. The `crossbeam-deque` work-stealing queues use it internally.

## What This Unlocks

- **Read-side performance** — pinning costs one atomic load; no locks, no reference count bumps.
- **Correct memory safety** — the borrow checker plus epoch guards ensures you can't access freed memory.
- **Lock-free data structures in practice** — EBR is what makes production lock-free code viable in systems languages.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memory reclamation | Garbage collector | Epoch-based (`crossbeam-epoch`) |
| Read-side overhead | GC write barriers | Single atomic load (pin) |
| Reclaim timing | GC decides | When all threads past epoch |
| API | Automatic | Explicit `guard.defer_destroy()` |
