[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 089 — Lazy Sequences
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Create infinite lazy sequences in Rust using `std::iter::from_fn` and `std::iter::successors` — without custom structs. Generate natural numbers, Fibonacci numbers, and powers of two using closures that capture mutable state. Compare with OCaml's `Seq` thunk-based lazy sequences.

## Learning Outcomes

- Use `std::iter::from_fn(move || …)` to create a stateful iterator from a closure
- Use `std::iter::successors(seed, step)` for sequences defined by recurrence
- Apply `.take(n)` to safely bound infinite iterators
- Understand why `move` is required — the closure must own its mutable state
- Map `from_fn` to OCaml's `Seq.Cons(v, thunk)` construction
- Choose between `from_fn` (general), `successors` (recurrence), and a struct iterator

## Rust Application

`std::iter::from_fn` takes a `move` closure returning `Option<T>` — it yields `Some(v)` indefinitely or terminates with `None`. The natural number generator captures `n: i32` and increments on each call. Fibonacci uses `std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))` — each step transforms the current pair into the next; `.map(|(a, _)| a)` extracts the first element. Powers of two use `from_fn` with a termination condition (`if exp >= 4 { None }`). All are combined with `.take(n).collect()` to materialise a bounded prefix.

## OCaml Approach

OCaml's `Seq` type requires explicit thunk construction: `let rec aux n () = Seq.Cons(n, aux (n+1))`. The `from_fn` equivalent uses a mutable `ref` cell and a `Seq.Cons(v, aux)` where `aux` is the recursive thunk. `Seq.take n s` materialises the first `n` elements. The key difference is that OCaml's sequences share structure via closures (not mutable state), while Rust's `from_fn` uses mutated captured variables.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Generator | `from_fn(move \|\| …)` | `let rec aux () = Seq.Cons(…)` |
| Recurrence | `successors(seed, step)` | `let rec aux a b () = Seq.Cons(a, aux b (a+b))` |
| State | `move` captured mutable variable | Recursive argument or `ref` |
| Termination | Return `None` | Return `Seq.Nil` |
| Bounding | `.take(n)` | `Seq.take n` |
| Sharing | No sharing (new state per call) | Structural sharing via thunks |

`from_fn` and `successors` are the quickest way to create custom lazy sequences in Rust without defining a struct. Use `successors` for single-state recurrences (Fibonacci, powers), `from_fn` for more general stateful generation, and a struct iterator for complex multi-field state.

## Exercises

1. Generate the Collatz sequence for a given start value using `successors(Some(n), |&n| if n == 1 { None } else { Some(next) })`.
2. Create a lazy sieve of Eratosthenes by combining `from_fn` with a `HashSet` of composites.
3. Implement a `zip_lazy` function that takes two `from_fn`-style sequences and yields pairs.
4. Use `std::iter::repeat_with` to generate random numbers and compare it to `from_fn` with the same effect.
5. In OCaml, implement a lazy merge of two sorted infinite sequences into a single sorted sequence using `Seq`.
