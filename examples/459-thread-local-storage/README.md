📖 **[View on hightechmind.io →](https://hightechmind.io/rust/459-thread-local-storage)**

---

# 459: Thread-Local Storage
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Some per-request or per-thread state shouldn't be shared: random number generator seeds, per-thread error codes, per-thread profiling counters, locale settings. Global shared state requires synchronization; passing context through every function is verbose. Thread-local storage (TLS) provides a third option: each thread has its own independent copy of a variable, accessible without synchronization. Accessing TLS is as fast as a local variable with OS thread support, and Rust's `thread_local!` makes it safe and ergonomic.

TLS appears in Rust's panic handling (`PANIC_COUNT`), allocator state, async executor task-local values, and per-thread performance counters.

## Learning Outcomes

- Understand how `thread_local!` creates per-thread variable storage
- Learn how `ThreadLocalKey::with(|val| ...)` provides safe access to TLS values
- See how `Cell<T>` (for `Copy` types) and `RefCell<T>` (for complex types) work in TLS
- Understand why TLS values can't escape their thread (they're `!Send`)
- Learn the initialization: TLS values initialize on first access per thread

## Rust Application

In `src/lib.rs`, `static COUNTER: Cell<usize>` and `static BUFFER: RefCell<String>` are thread-local declarations. `increment_counter()` uses `COUNTER.with(|c| c.get() + 1)` — the closure provides access to the thread-local within its scope, preventing the value from escaping. `Cell` is used for `usize` (copy type); `RefCell` is needed for `String` (non-copy). Tests verify that different threads have independent counters.

## OCaml Approach

OCaml 4.x uses `let state = ref initial_value` per thread — module-level references are per-thread since each thread has its own OCaml runtime state in `Thread` contexts (this is subtler in OCaml 4.x). OCaml 5.x provides `Domain.DLS.get/set` (domain-local storage) as the explicit per-domain storage mechanism, analogous to thread-local storage for domains.

## Key Differences

1. **Ergonomics**: Rust's `thread_local!` is a language-level macro with clear semantics; OCaml 5.x's `Domain.DLS` requires explicit key creation and lookup.
2. **Cell types**: Rust uses `Cell<T>` or `RefCell<T>` for TLS to provide interior mutability; OCaml uses `ref` values which are always mutable.
3. **Lifetime**: Rust's TLS values live for the thread's duration; OCaml 5.x's `Domain.DLS` values live for the domain's duration.
4. **Non-escaping**: Rust's `with` callback prevents TLS references from escaping the thread; OCaml has no such enforcement.

## Exercises

1. **Per-thread RNG**: Create `thread_local! { static RNG: RefCell<XorShift> = ... }` where `XorShift` is a simple random number generator seeded with the thread ID. Verify that different threads produce different random sequences.
2. **Allocation tracking**: Use `thread_local! { static ALLOC_COUNT: Cell<usize> = const { Cell::new(0) } }` to count allocations per thread. Wrap allocation-using code and verify counts are independent per thread.
3. **Context propagation**: Implement a "request context" pattern: `thread_local! { static REQUEST_ID: Cell<u64> = const { Cell::new(0) } }`. Write `with_request(id, || ...)` that sets the ID for the duration of the closure, then restores it — enabling implicit context propagation through function calls.
