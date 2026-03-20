📖 **[View on hightechmind.io →](https://hightechmind.io/rust/222-histomorphism)**

---

# Histomorphism — Cata with Full History
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Sometimes a fold needs not just the immediate child's result, but the full history of all previous results. Fibonacci is the classic example: `fib(n) = fib(n-1) + fib(n-2)` — the current result depends on two previous results. A catamorphism can only see the immediately adjacent result. A histomorphism provides access to all previous results via a "Cofree" annotation: each node carries its folded result and a pointer to the previous annotation, forming a linked history.

## Learning Outcomes

- Understand histomorphisms as catamorphisms with access to the full fold history
- Learn the `Cofree` type: `Cofree<F, A> { value: A, sub: F<Cofree<F, A>> }`
- See how Fibonacci in O(n) is a natural histomorphism over natural numbers
- Understand the performance benefit: `histo` computes Fibonacci in O(n) vs. naive recursion O(2ⁿ)

## Rust Application

`Cofree<A>` represents "a node annotated with `A`, plus annotated children." The histomorphism:
`histo<A>(alg: impl Fn(NatF<Cofree<A>>) -> A) -> impl Fn(FixNat) -> A`. The Fibonacci algebra: for `ZeroF` return `0`; for `SuccF(cofree)` return `cofree.value + cofree.sub.value` — the two most recent values. No HashMap memo table is needed — the Cofree structure IS the memo table.

## OCaml Approach

OCaml's histomorphism:
```ocaml
type 'a cofree = Cofree of 'a * 'a cofree nat_f

let rec histo alg (Fix nf) =
  let cf = map_nat_f (fun child ->
    let result = histo alg child in
    Cofree(result, map_nat_f (histo alg) nf)
  ) nf in
  alg cf
```
OCaml's pattern matching on `Cofree` accesses the history naturally. The `histo` scheme computes Fibonacci without memoization tables.

## Key Differences

1. **Implicit memoization**: `histo` achieves O(n) Fibonacci through the Cofree structure, not through an explicit HashMap — the history IS the cache.
2. **Cofree comonad**: `Cofree<F, A>` is the Cofree comonad over functor `F` (example 245) — the recursion scheme and comonad concepts are deeply connected.
3. **Performance**: Naive recursive Fibonacci is O(2ⁿ); `histo` Fibonacci is O(n) with O(n) stack space; iterative is O(n) with O(1) space.
4. **History depth**: `histo` gives access to ALL previous results; `para` gives access only to the immediate sub-structure.

## Exercises

1. Implement Lucas numbers using `histo`: `L(0)=2, L(1)=1, L(n)=L(n-1)+L(n-2)`.
2. Verify that `histo_fibonacci(30)` produces the correct result and is faster than the naive recursive version.
3. Implement a sliding-window maximum using `histo` that returns the max over the last k elements.
