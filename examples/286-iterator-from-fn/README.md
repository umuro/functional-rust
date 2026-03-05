# 286: Creating Iterators with from_fn()

**Difficulty:** 2  **Level:** Intermediate

Create a stateful iterator from a closure — the lightweight alternative to implementing the `Iterator` trait.

## The Problem This Solves

You need a custom iterator but implementing a full struct with `Iterator` is overkill. You want to generate a Fibonacci sequence for a test. Parse tokens from a string one at a time. Simulate reading from a buffer. For these one-off sequences, defining a named struct feels like ceremony — you name a type that's used once, define fields, implement a trait, and repeat a lot of boilerplate.

`std::iter::from_fn` lets you skip all of that: pass a closure that captures its mutable state and returns `Option<T>`. Return `Some(value)` to yield, `None` to terminate. The closure *is* the iterator.

In OCaml, you'd use `Seq.unfold` or `Seq.of_dispenser`. In Haskell, `unfoldr`. In Rust, `from_fn` is the direct equivalent for the case where each step doesn't need the previous value to compute the next.

## The Intuition

`std::iter::from_fn(closure)` creates an iterator that calls the closure on each `next()`. The closure captures its mutable state in the closure environment. Return `Some(value)` to emit; return `None` to stop.

```rust
let mut n = 0;
let counter = std::iter::from_fn(move || {
    n += 1;
    if n <= 5 { Some(n) } else { None }
});
// → yields 1, 2, 3, 4, 5
```

## How It Works in Rust

```rust
// Fibonacci — state lives inside the closure (no struct needed)
let fib = {
    let (mut a, mut b) = (0u64, 1u64);
    std::iter::from_fn(move || {
        let val = a;
        let next = a + b;
        a = b;       // advance state
        b = next;
        Some(val)    // always Some — infinite sequence, use take()
    })
};
let first_10: Vec<u64> = fib.take(10).collect();
// → [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

// Parse tokens one at a time from an external iterator
let input = "42 17 99 3 55";
let mut words = input.split_whitespace();
let numbers: Vec<u32> = std::iter::from_fn(|| {
    words.next().and_then(|w| w.parse().ok())  // None on parse fail or exhaustion
}).collect();
// → [42, 17, 99, 3, 55]

// Simulate buffer reads
let buffer = vec![1u8, 2, 3, 4, 5];
let mut idx = 0;
let reader = std::iter::from_fn(|| {
    if idx < buffer.len() {
        let v = buffer[idx];
        idx += 1;
        Some(v)
    } else {
        None
    }
});
let bytes: Vec<u8> = reader.collect();
```

`from_fn` requires the closure to be `FnMut` — it's called repeatedly. Use `move` to capture owned state. For sequences where each element depends on the previous, also consider `successors()`.

## What This Unlocks

- **Throwaway generators** — Fibonacci, counters, random sequences without defining a named struct.
- **Wrapping external state** — turn a mutable external cursor (database row pointer, file position) into an iterator.
- **On-demand parsing** — consume tokens from a shared iterator inside the closure, producing parsed results.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterator from closure | `Seq.of_dispenser` | `std::iter::from_fn(closure)` |
| Infinite sequence | `Seq.unfold` | `from_fn(|| Some(next))` + `.take(n)` |
| State storage | Closure captures / `ref` | Closure captures (via `move`) |
| vs. custom struct | `Seq.unfold` is more general | `from_fn` for no prev-value dependency; `successors` for chained |
| Termination | Return `None` node | Return `None` from closure |
