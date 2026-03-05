📖 **[View on hightechmind.io →](https://hightechmind.io/rust/287-iterator-successors)**

---

# 287: Recursive Sequences with successors()

**Difficulty:** 2  **Level:** Intermediate

Generate a sequence where each element is computed from the previous one — powers, Collatz, Newton's method.

## The Problem This Solves

Many sequences are defined recursively: each term is a function of the previous term. Powers of 2 (`n → 2n`). The Collatz sequence (`n → n/2` if even, `3n+1` if odd). Newton-Raphson iterations converging to a root. A string shrinking by one character at a time.

`from_fn` handles stateful generators, but when each new value is purely derived from the previous one, `successors` is more expressive. You provide the first element and a function `f(prev) -> Option<next>`. Returning `None` terminates the sequence. There's no mutable state to manage — the function receives the previous value directly.

In Haskell this is `iterate` (infinite) or `unfoldr` (with termination). In OCaml it's `Seq.unfold`. In Rust, `std::iter::successors` is the precise match.

## The Intuition

`successors(first, f)` generates: `first, f(first), f(f(first)), ...` — applying `f` repeatedly. Return `None` from `f` to stop. The first element is `Option<T>` — pass `None` to get an empty iterator.

```rust
let powers: Vec<u32> = std::iter::successors(Some(1u32), |&n| Some(n * 2))
    .take(8).collect();
// → [1, 2, 4, 8, 16, 32, 64, 128]
```

## How It Works in Rust

```rust
// Powers of 2 — terminate when past 512
let powers_of_2: Vec<u32> = std::iter::successors(Some(1u32), |&n| {
    if n < 512 { Some(n * 2) } else { None }  // None stops the sequence
}).collect();
// → [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]

// Collatz sequence from 6
let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
    if n == 1 { None }                          // terminate at 1
    else if n % 2 == 0 { Some(n / 2) }
    else { Some(3 * n + 1) }
}).collect();
// → [6, 3, 10, 5, 16, 8, 4, 2, 1]

// Newton's method — converge to sqrt(2)
let sqrt2: Vec<f64> = std::iter::successors(Some(1.0f64), |&x| {
    let next = 0.5 * (x + 2.0 / x);            // Newton step
    if (next - x).abs() < 1e-10 { None }        // converged — stop
    else { Some(next) }
}).collect();
// → [1.0, 1.5, 1.4166..., 1.41421356..., ...]

// Shrinking string — each step removes first character
let shrinking: Vec<String> = std::iter::successors(
    Some("hello".to_string()),
    |s| if s.is_empty() { None } else { Some(s[1..].to_string()) }
).collect();
// → ["hello", "ello", "llo", "lo", "o", ""]

// Empty start — passing None gives an empty iterator
let empty: Vec<i32> = std::iter::successors(None, |&_: &i32| Some(1)).collect();
// → []
```

The closure receives `&T` (a reference to the previous value), not `T` — destructure with `|&n|` for `Copy` types.

## What This Unlocks

- **Mathematical sequences** — powers, geometric series, Collatz, Fibonacci (though `from_fn` is simpler for Fibonacci).
- **Iterative numerical methods** — Newton-Raphson, bisection, any algorithm that refines an estimate step by step.
- **Shrinking / expanding transformations** — progressively modify a structure, emitting each stage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Unfold from previous | `Seq.unfold f seed` | `std::iter::successors(first, f)` |
| Terminate | Return `None` from unfold | Return `None` from `f` |
| Closure receives | Previous value + seed | Reference to previous value only |
| vs. `from_fn` | `from_fn` for independent state | `successors` when `next = f(prev)` |
| Infinite variant | `Seq.iterate f x` | `successors(Some(x), \|&v\| Some(f(v)))` |
