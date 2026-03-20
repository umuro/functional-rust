📖 **[View on hightechmind.io →](https://hightechmind.io/rust/454-compare-exchange)**

---

# 454: Compare-and-Exchange (CAS)

## Problem Statement

Atomic increment (`fetch_add`) handles the simple case, but complex atomic updates — "set to max", "set if value is X", "conditional swap" — require `compare_and_exchange` (CAS). CAS is the universal primitive for lock-free algorithms: read the current value, compute the new value, atomically swap old→new only if the value is still what you read. If another thread changed it, retry. This retry loop is the foundation of all lock-free data structures: lock-free stacks, queues, linked lists, and hash maps.

CAS appears in `Arc`'s reference count decrement (is-zero check), lock-free queue implementations, optimistic concurrency control, and `AtomicPtr` pointer swaps.

## Learning Outcomes

- Understand the CAS loop pattern: load → compute → compare_exchange → retry on failure
- Learn the difference between `compare_exchange` (strong, no spurious failure) and `compare_exchange_weak` (weak, may spuriously fail — preferred in loops)
- See how `atomic_max` uses CAS to atomically update a maximum value
- Understand the `(success_ordering, failure_ordering)` parameters
- Learn the ABA problem and why some lock-free algorithms need versioned pointers

## Rust Application

In `src/lib.rs`, `cas_increment` shows the classic CAS loop: `load` the current value, `compare_exchange_weak(cur, cur+1, AcqRel, Relaxed)` — if successful, done; on `Err(actual)`, update `cur` to the actual value and retry. `atomic_max` adds a guard: if `v <= cur`, skip the CAS entirely. The `compare_exchange_weak` variant is preferred in loops because spurious failure triggers a retry (loop handles it), while `compare_exchange` guarantees success if the values match (slightly higher cost on ARM).

## OCaml Approach

OCaml 5.x's `Atomic.compare_and_set old_val new_val at` is the CAS primitive. It returns a `bool` rather than `Result<_, actual>`, so you reload the value after failure. OCaml's CAS uses sequential consistency. A CAS-based counter: `let cas_inc a = while not (let cur = Atomic.get a in Atomic.compare_and_set a cur (cur+1)) do () done`.

## Key Differences

1. **Result vs. bool**: Rust's `compare_exchange` returns `Result<old_val, actual_val>`; OCaml returns `bool`, requiring a separate `Atomic.get` to get the actual value on failure.
2. **Weak variant**: Rust has `compare_exchange_weak` for loop use; OCaml's single `compare_and_set` corresponds roughly to `compare_exchange` (strong).
3. **ABA problem**: Both languages' CAS operations are susceptible to the ABA problem; solutions require versioned pointers (`u128` packing tag + pointer) or hazard pointers.
4. **Ordering**: Rust separates success and failure orderings; OCaml uses sequential consistency for all atomic operations.

## Exercises

1. **Atomic stack push**: Implement lock-free stack push using CAS on `AtomicPtr<Node<T>>`. `push` allocates a node, sets its next to the current head, then CAS-swaps the head from old_head to new_node. Repeat on failure.
2. **Versioned CAS (DCAS)**: On platforms with 128-bit atomics (using `AtomicU128` or a crate), implement an `(version, value)` CAS that increments the version on every successful update, preventing the ABA problem.
3. **CAS vs. mutex benchmark**: Implement a shared counter using CAS loop and one using `Mutex<u64>`. Benchmark both with 8 threads each doing 1M increments. Plot throughput vs. number of threads from 1 to 16.
