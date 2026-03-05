📖 **[View on hightechmind.io →](https://hightechmind.io/rust/515-closure-once-cell)**

---

# 515: Lazy Evaluation with OnceLock

**Difficulty:** 3  **Level:** Intermediate

Initialize a value exactly once, on first access, and cache it forever.

## The Problem This Solves

Some values are expensive to compute but only needed sometimes, and when needed, needed many times. Computing them eagerly wastes resources if they're never used. Recomputing them every access wastes time. The solution is lazy initialization: compute once on first access, cache the result, return the cached value on all subsequent accesses.

In Rust, global statics can't be initialized with arbitrary runtime expressions. `OnceLock<T>` solves this: declare a `static` of type `OnceLock<T>`, and call `.get_or_init(|| ...)` to compute and store the value the first time it's accessed. Every subsequent call returns the cached value instantly. No `unsafe`, no external crates, thread-safe by design.

The same pattern works for struct fields. A `LazyConfig` that parses its raw string only when `.items()` is first called — not in the constructor — uses `OnceLock` fields. The parse cost is paid once, on demand, and the result is cached for the struct's lifetime.

## The Intuition

Think of `OnceLock<T>` as a box with a one-way lock. It starts empty. The first person to call `get_or_init` runs the initializer, puts the result in the box, and locks it. Everyone after that just reads from the box — the lock prevents any second initialization. The box can never go back to empty.

## How It Works in Rust

1. **Global static** — `static VALUE: OnceLock<T> = OnceLock::new();` declares an uninitialized global.
2. **`get_or_init`** — `.get_or_init(|| expensive_computation())` initializes on first call, returns a `&T`; subsequent calls return the same `&T` instantly.
3. **Struct fields** — embed `OnceLock<T>` in a struct; initialize in a method that takes `&self`; the field is effectively a lazy-computed cached property.
4. **Thread safety** — `OnceLock<T>` is `Sync`; multiple threads can race on `get_or_init`, only one will run the initializer, others will wait and then read the result.
5. **`Arc<T>` compatible** — wrap a struct containing `OnceLock` in `Arc` for multi-threaded sharing; initialization is safe across threads.

## What This Unlocks

- Initialize expensive globals (regex, config, lookup tables) without `unsafe` or external crates.
- Add lazy cached properties to structs without `RefCell` or `Mutex` — `OnceLock` is `Sync`.
- Defer parsing, compilation, or database connection setup to first use without penalizing startup time.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lazy evaluation | `lazy_t` library or `Lazy.t` (Jane Street); GC-managed | `OnceLock<T>` in std; `once_cell::Lazy<T>` crate for more ergonomics |
| Global lazy values | `let x = lazy (fun () -> ...)` | `static X: OnceLock<T> = OnceLock::new()` + `get_or_init` |
| Thread safety | GC handles memory; `Mutex` for concurrency | `OnceLock` is `Sync`; initializes exactly once across threads |
| Lazy struct fields | Mutable record + option field manually | `OnceLock<T>` field; computed in `&self` methods |
