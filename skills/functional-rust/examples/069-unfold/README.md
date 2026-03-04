# 069: Unfold

**Difficulty:** ⭐⭐  **Level:** Foundations

Generate sequences from a seed value — the functional dual of `fold`.

## The Problem This Solves

You want to produce a sequence by repeatedly applying a rule to some state. Fibonacci numbers start from `(0, 1)` and each step produces the next pair. A Collatz sequence starts from `n` and each step halves (if even) or triples-plus-one (if odd) until reaching 1. A countdown starts from `n` and decrements to 0.

The imperative approach: a `while` loop with mutation. That works, but it ties together "how to generate" and "what to do with the results." You can't lazily generate, can't reuse the generation logic, can't compose it with other iterators.

`unfold` separates those concerns. You provide: (1) an initial seed, and (2) a function that takes the current state and either produces `(value, next_state)` or `None` to stop. `unfold` does the rest.

## The Intuition

If `fold` is "consume a list, produce a value" (many → one), then `unfold` is "consume a seed, produce a list" (one → many). They're conceptual mirrors.

The function you pass to `unfold` answers: "given the current state, what's the next value to emit, and what's the next state?" Return `None` to stop. This is exactly `Seq.unfold` in OCaml, and `std::iter::successors` + `std::iter::from_fn` in Rust.

## How It Works in Rust

Custom `unfold` function:

```rust
pub fn unfold<T, S, F>(seed: S, f: F) -> Vec<T>
where
    F: Fn(S) -> Option<(T, S)>,
    S: Clone,
{
    let mut result = Vec::new();
    let mut state = seed;
    while let Some((value, next)) = f(state.clone()) {
        result.push(value);
        state = next;
    }
    result
}
```

Using it:

```rust
// Range: count from a to b
let range_1_5 = unfold(1, |i| if i > 5 { None } else { Some((i, i + 1)) });
// → [1, 2, 3, 4, 5]

// Collatz sequence from 6
let collatz_6 = unfold(6u64, |x| match x {
    0 => None,
    1 => Some((1, 0)),  // emit 1, stop next
    x if x % 2 == 0 => Some((x, x / 2)),
    x => Some((x, 3 * x + 1)),
});
// → [6, 3, 10, 5, 16, 8, 4, 2, 1]
```

Rust's built-in lazy version using `std::iter::successors`:

```rust
// Infinite Fibonacci iterator — lazy, no allocation until consumed
pub fn fibs() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}
```

## What This Unlocks

- **Sequence generation** — any series defined by a recurrence (Fibonacci, primes, Collatz, geometric series)
- **State machine unrolling** — generate all states of a system from an initial state and a transition function
- **Lazy infinite sequences** — use `successors` to define a sequence, then `take(n)` to consume only what you need

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in unfold | `Seq.unfold f seed` (lazy) | `std::iter::successors` (lazy) |
| Custom unfold | Recursive with `::` cons | Loop with `Vec::push` |
| Termination | Return `None` | Return `None` |
| Eager vs lazy | `List.of_seq (Seq.unfold ...)` | `unfold(...)` for Vec, `successors` for lazy |
