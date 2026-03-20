[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 090 — Infinite Iterators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Create infinite iterators using Rust's standard library combinators: `cycle` (repeat a finite sequence forever), `repeat` (emit one value forever), `from_fn` (stateful generator), and `repeat_with` (side-effecting generator). Always bound with `.take(n)` before collecting. Compare with OCaml's `Seq`-based infinite sequence representations.

## Learning Outcomes

- Use `.cycle()` to loop a finite iterator indefinitely
- Use `std::iter::repeat(v)` for constant infinite streams
- Use `std::iter::from_fn(move || …)` for stateful generation
- Use `std::iter::repeat_with(move || …)` for side-effecting generation
- Always apply `.take(n)` before consuming an infinite iterator
- Map Rust's infinite iterator combinators to OCaml's `Seq` recursive thunks

## Rust Application

`[1, 2, 3].iter().copied().cycle().take(7)` repeats `[1, 2, 3]` until 7 elements are emitted. `std::iter::repeat(42).take(4)` emits four `42`s. `from_fn` captures a mutable `n: i32` and increments on each call — returning `Some(n)` always makes it infinite. `repeat_with` is similar but intended for cases where the closure has side effects (like a RNG call). All four use the same consumer: `.take(n).collect::<Vec<_>>()`.

## OCaml Approach

OCaml's `cycle` wraps a list as a `Seq` and restarts from the list when the current pointer is `Nil`. `repeat` is `let rec aux () = Seq.Cons(x, aux)`. `counter_from` uses a mutable `ref` and a thunk. The implementations mirror Rust's semantics — the difference is structural: Rust combinators are zero-cost structs; OCaml uses heap-allocated thunks.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Cycle | `.cycle()` adapter | Custom recursive `Seq` |
| Repeat | `std::iter::repeat(v)` | `let rec aux () = Seq.Cons(v, aux)` |
| From closure | `from_fn(move \|\| …)` | Closure returning `Seq.Cons(…, thunk)` |
| Bounding | `.take(n)` | `Seq.take n` |
| Side effects | `repeat_with(move \|\| …)` | Same closure pattern |
| Memory | Zero-cost structs | Heap thunks |

The key rule with infinite iterators: never call `collect()` or `sum()` without a prior `.take(n)` or another bounding consumer. Rust's type system does not prevent infinite loops — the programmer must ensure termination.

## Exercises

1. Generate an infinite sequence of even numbers using `(0..).step_by(2)` and take the first 10.
2. Implement a `zip_cycle` that zips a finite slice with an infinite cycle: `[1,2,3]` zipped with `[a,b,c,a,b,c,…]`.
3. Use `repeat_with` with `rand::random::<f64>()` to generate 100 random floats and compute their mean.
4. Create an infinite geometric sequence `a, ar, ar², ar³, …` using `successors`.
5. In OCaml, implement `cycle_seq : 'a list -> 'a Seq.t` and verify it produces the correct repeated sequence.
