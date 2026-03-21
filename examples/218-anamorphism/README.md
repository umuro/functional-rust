📖 **[View on hightechmind.io →](https://hightechmind.io/rust/218-anamorphism)**

---

# Anamorphism — Unfold to Build Recursive Structures
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The dual of a catamorphism is the anamorphism (`ana`) — it unfolds a seed into a recursive structure. Where `cata` tears down a structure bottom-up, `ana` builds one top-down. `unfold` for lists is an anamorphism: starting from a seed, repeatedly apply a coalgebra to produce the next element and the new seed, until reaching the base case. This is how infinite streams, number sequences, and tree generation are expressed functionally.

## Learning Outcomes

- Understand anamorphisms as the dual of catamorphisms (unfold vs. fold)
- Learn how a coalgebra `seed -> F<seed>` drives top-down unfolding
- See examples: unfolding an integer range into a list, unfolding a tree from a specification
- Understand the connection to codata (potentially infinite structures) in functional programming

## Rust Application

`ana<S>(coalg: impl Fn(S) -> ListF<S>) -> impl Fn(S) -> FixList` implements the universal unfold. Coalgebras: `range_coalg(n: i64) -> ListF<i64>` = `if n <= 0 { NilF } else { ConsF(n, n-1) }` — counts down to zero. `fibonacci_coalg((a, b)) -> ListF<(i64, i64)>` = `ConsF(a, (b, a+b))` — generates Fibonacci numbers. `ana(range_coalg)(5)` produces `[5, 4, 3, 2, 1]`.

## OCaml Approach

OCaml's anamorphism:
```ocaml
let rec ana coalg seed = Fix (map_list_f (ana coalg) (coalg seed))
(* Range example: *)
let range_coalg n = if n <= 0 then NilF else ConsF (n, n-1)
let range n = ana range_coalg n
```
OCaml's lazy sequences (`Seq.t`) provide the standard mechanism for anamorphism-style generation — each `Seq.cons` is one step of an anamorphism with the continuation as the tail thunk.

## Key Differences

1. **Duality**: `ana` and `cata` are mathematical duals — `cata` = fold (teardown), `ana` = unfold (buildup); this symmetry appears throughout category theory.
2. **Infinite structures**: Anamorphisms naturally express codata (infinite lists, streams); in strict languages, lazy evaluation is needed to prevent infinite unfolding.
3. **OCaml `Seq`**: OCaml's `Seq.unfold : ('b -> ('a * 'b) option) -> 'b -> 'a Seq.t` is the standard anamorphism for lazy sequences — directly matching the coalgebra pattern.
4. **Rust iterators**: Rust's `std::iter::successors(init, f)` and `unfold(init, f)` (in `itertools`) implement the iterator version of anamorphisms.

## Exercises

1. Implement `take_n(coalg, seed, n)` that generates at most `n` elements — preventing infinite loops.
2. Write an anamorphism that generates a binary tree from a list of values (each level takes the next value).
3. Use `ana` to implement `repeat(x)` — an infinite list of the same value.
