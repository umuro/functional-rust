# 090: Infinite Iterators

**Difficulty:** 2  **Level:** Intermediate

Generate endless data with `repeat`, `cycle`, `successors`, and `from_fn` — safe because Rust iterators are lazy and nothing is computed until consumed.

## The Problem This Solves

Many algorithms need infinite data sources: a constant stream of default values, a repeating pattern, a sequence defined by a recurrence relation. Without lazy infinite iterators, you'd need to pre-compute a large finite list and hope it's big enough — or write a custom struct for every infinite sequence you need.

Rust's standard library provides building blocks for infinite iterators: `repeat` for constant streams, `cycle` for repeating patterns, `successors` for recurrence relations (like OCaml's `iterate`), and `from_fn` for arbitrary state-machine sequences (like OCaml's `unfold`). All are safe because nothing runs until `.take(n)` or another consumer limits it.

The key insight: an infinite iterator is just a promise to keep producing values. The consumer decides when to stop.

## The Intuition

In Python, `itertools.repeat(x)`, `itertools.cycle(seq)`, and generator functions with infinite `while True` loops cover the same ground. In OCaml, `Stream.iterate` and `Seq.unfold` handle recurrences and stateful generation. In Haskell, `repeat`, `cycle`, and `iterate` are first-class built-ins.

Rust's versions work the same way — the only difference is that you must explicitly limit them (`.take(n)`) because Rust has no garbage collector and won't stop you from accidentally computing an infinite result. The compiler doesn't warn you; the type system doesn't prevent it. Convention and common sense apply: always pair infinite iterators with a terminating consumer.

## How It Works in Rust

```rust
// repeat: infinite stream of the same value
let fives: Vec<i32> = std::iter::repeat(5).take(3).collect();  // [5, 5, 5]

// cycle: repeat a finite sequence forever
let pattern: Vec<i32> = [1, 2, 3].iter().copied().cycle().take(7).collect();
// [1, 2, 3, 1, 2, 3, 1]

// repeat_with: call a closure for each element — useful for computed or mutable values
let mut n = 1.0_f64;
let doubles: Vec<f64> = std::iter::repeat_with(move || { let v = n; n *= 2.0; v })
    .take(5).collect();  // [1.0, 2.0, 4.0, 8.0, 16.0]
```

```rust
// successors: recurrence relation — like OCaml's iterate
// Some(next) continues; None terminates
fn doubles_from(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |&prev| Some(prev * 2))
}

// Naturally terminating with checked arithmetic
let powers: Vec<u64> = std::iter::successors(Some(1u64), |&n| n.checked_mul(2))
    .collect();  // terminates on overflow — finite but defined infinitely

// Collatz sequence — terminates when it reaches 1
let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
    if n == 1 { None } else if n % 2 == 0 { Some(n / 2) } else { Some(3 * n + 1) }
}).collect();
```

```rust
// from_fn with explicit state — like OCaml's unfold
fn unfold<T, S, F>(init: S, f: F) -> impl Iterator<Item = T>
where F: Fn(&S) -> Option<(T, S)>
{
    let mut state = Some(init);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}

// Fibonacci via unfold — clean state transition
let fib: Vec<u64> = unfold((0u64, 1u64), |&(a, b)| Some((a, (b, a + b))))
    .take(8).collect();  // [0, 1, 1, 2, 3, 5, 8, 13]
```

```rust
// Combine infinite iterators
let alternating: Vec<i32> = std::iter::repeat(0)
    .zip(std::iter::repeat(1))
    .flat_map(|(x, y)| std::iter::once(x).chain(std::iter::once(y)))
    .take(8).collect();  // [0, 1, 0, 1, 0, 1, 0, 1]
```

## What This Unlocks

- **Round-robin and rotation**: `cycle` over a list of servers, colors, or tokens — distribute load without tracking indices manually.
- **Recurrence sequences**: Fibonacci, Collatz, Newton's method — define the recurrence once with `successors`, consume however many steps you need.
- **Stateful generation**: `from_fn` with mutable captured state covers any sequence a generator function can express in Python.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Constant stream | `Seq.repeat` (not in stdlib, custom) | `std::iter::repeat(x)` |
| Cyclic repetition | Manual `Seq` wrapping | `.cycle()` on any finite iterator |
| Recurrence (iterate) | `Seq.unfold` or custom | `std::iter::successors(init, f)` |
| Stateful unfold | `Seq.unfold` | `std::iter::from_fn` with captured state |
| Safety | `Seq` is lazy, GC prevents runaway | Lazy — must use `.take(n)` or consumer |
| Interleave | Custom `Seq` composition | `.zip().flat_map(...)` |
