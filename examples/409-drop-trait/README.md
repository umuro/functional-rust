📖 **[View on hightechmind.io →](https://hightechmind.io/rust/409-drop-trait)**

---

# 409: Drop Trait and RAII
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Resource management is one of the oldest problems in systems programming. C requires manual `fclose(f)`, `free(ptr)`, `release_lock()` — calls that are easily forgotten, especially in error paths. C++ introduced RAII (Resource Acquisition Is Initialization): resources are tied to stack lifetimes, and destructors run automatically when objects go out of scope. Rust adopts this with the `Drop` trait: implement `fn drop(&mut self)` and it runs deterministically when the value is destroyed — at end of scope, when moved into a function that consumes it, or when explicitly dropped with `drop(val)`.

`Drop` powers `MutexGuard` (unlocks on drop), `File` (closes on drop), `Vec`/`String` (frees heap on drop), database transactions, and any pattern needing guaranteed cleanup.

## Learning Outcomes

- Understand how `Drop` enables deterministic resource cleanup in Rust
- Learn the RAII pattern: resource lifetime equals value lifetime
- See how `FileHandle` and `LockGuard` clean up automatically when they go out of scope
- Understand that `Drop` and `Copy` are mutually exclusive (copying would duplicate resources)
- Learn how `std::mem::drop(val)` enables early cleanup before scope end

## Rust Application

In `src/lib.rs`, `FileHandle` tracks open state via `Cell<bool>`. The `Drop` impl sets `is_open` to false, simulating file closure. `LockGuard` demonstrates lock release on drop — the RAII guard pattern used by `std::sync::MutexGuard`. The `Cell<bool>` is used instead of a plain `bool` to allow `drop` to modify state through a shared reference (Rust's `drop` signature is `&mut self`, but `Cell` enables interior mutability for the `is_open` check in other methods).

## OCaml Approach

OCaml's GC manages memory automatically but does not provide deterministic finalization. `Gc.finalise` attaches a finalizer that runs at some point after the value becomes unreachable, but timing is not guaranteed. The `Fun.protect ~finally` function provides RAII-like cleanup: `Fun.protect ~finally:(fun () -> close f) (fun () -> use f)`. OCaml's standard idiom for resource management is explicit `with_*` functions rather than RAII.

## Key Differences

1. **Determinism**: Rust's `Drop` runs at a known point (end of scope); OCaml's finalizers run at GC-determined times, which may be delayed.
2. **Error paths**: Rust's `Drop` runs even on panic (unless double-panic); OCaml's `Fun.protect ~finally` must be explicitly structured around every operation.
3. **Copy exclusion**: Rust's `Drop` and `Copy` are mutually exclusive — you can't copy a value with a destructor; OCaml has no such restriction.
4. **Explicit drop**: Rust allows `std::mem::drop(val)` for early cleanup; OCaml has no early finalization (only `Gc.compact` which is unpredictable).

## Exercises

1. **Connection pool**: Implement `Connection` (simulating a DB connection) and `ConnectionGuard` that wraps a `&'a mut Pool` and a `Connection`. On drop, return the connection to the pool. Show that the connection is always returned even when the code between acquisition and drop panics.
2. **Timed scope**: Create `TimedScope { name: String, start: Instant }` implementing `Drop` that prints elapsed time when the scope ends. Use it to measure how long a code block takes without explicit timing calls.
3. **Double-drop protection**: Implement a `SafeHandle` that tracks whether it has been dropped (via `Arc<AtomicBool>`) and panics if the `Drop` implementation is somehow called twice. Write tests verifying single-drop behavior.
