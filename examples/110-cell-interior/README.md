📖 **[View on hightechmind.io →](https://hightechmind.io/rust/110-cell-interior)**

---

# 110-cell-interior — Cell<T>: Interior Mutability for Copy Types

## Problem Statement

Rust's borrow checker prevents mutation through shared references (`&T`). But sometimes you need to update a field in a struct that is shared by multiple callers — a memoized cache, a call counter, or a lazy-initialized field — without needing `&mut self`. Interior mutability provides a controlled escape hatch.

`Cell<T>` is the simplest interior mutability primitive: it allows mutation through a shared reference by enforcing a rule that prevents multiple mutable accesses simultaneously (via the `get`/`set` API that never hands out references to the interior).

## Learning Outcomes

- Understand what interior mutability is and why it is needed
- Use `Cell<T>` to mutate a field through a shared `&self` reference
- Know that `Cell<T>` works only with `Copy` types (no references handed out)
- Contrast `Cell<T>` with `RefCell<T>` (works with non-Copy, runtime borrow check)
- Apply `Cell<T>` to lazy counters, memoization flags, and generation counters

## Rust Application

`src/lib.rs` demonstrates `counter_demo()` using a `Cell<u32>` incremented twice through `let counter = Cell::new(0)` (immutable binding, yet the value changes). The `Config` struct has `call_count: Cell<u32>` that increments in `process()` which takes `&self` — enabling shared callers to observe the count without needing `&mut self`. A `Node` in a tree uses `Cell<bool>` for a "dirty" flag that can be set during immutable traversal.

`Cell<T>` is single-threaded. For thread-safe interior mutability, use `AtomicU32` or `Mutex`.

## OCaml Approach

OCaml's `ref` is the direct equivalent: a mutable cell that can be updated through any binding:

```ocaml
let counter = ref 0   (* mutable ref — no 'mut' annotation needed *)
counter := !counter + 1
counter := !counter + 1
Printf.printf "%d
" !counter  (* 2 *)
```

OCaml record fields can be declared `mutable`:

```ocaml
type config = { name: string; mutable call_count: int }
let process cfg = cfg.call_count <- cfg.call_count + 1
```

There is no distinction between `Cell` and `RefCell` in OCaml — all mutable state is accessible through any binding.

## Key Differences

1. **Explicitness**: Rust's `Cell<T>` makes interior mutability explicit at the type level; OCaml's `ref` and `mutable` fields are natural and pervasive.
2. **Borrow-check bypass**: `Cell` bypasses the borrow checker's mutation rules; OCaml has no equivalent restriction to bypass.
3. **Thread safety**: `Cell<T>` is `!Sync` (not thread-safe); OCaml's `ref` is also not thread-safe but the GC protects from use-after-free.
4. **`Copy` restriction**: `Cell<T>` requires `T: Copy` because `get` returns by value (never handing out `&T`); `RefCell<T>` removes this restriction with runtime cost.

## Exercises

1. Implement a lazy-initialized field using `Cell<Option<i32>>` that computes on first access and caches the result.
2. Create a `GenerationCounter` struct with an immutable `&self` `increment()` method using `Cell<u32>`.
3. Show why `Cell<String>` does not work and how `RefCell<String>` solves it — demonstrate the runtime borrow-check panic.
