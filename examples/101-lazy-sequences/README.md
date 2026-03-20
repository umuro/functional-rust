📖 **[View on hightechmind.io →](https://hightechmind.io/rust/101-lazy-sequences)**

---

# Example 101: Lazy Sequences

**Difficulty:** ⭐⭐
**Category:** Lazy/Infinite Sequences
**OCaml Source:** [OCaml Docs — Seq Module](https://ocaml.org/docs/)

## Problem Statement

Create infinite lazy sequences: natural numbers, Fibonacci numbers, and primes. Take only what you need without computing the rest.

## Learning Outcomes

- Map OCaml's `Seq` module to Rust's `Iterator` trait
- Use `std::iter::successors` and `std::iter::from_fn` for custom infinite iterators
- Understand that Rust iterators are lazy by default (no `Seq` module needed)
- Implement `unfold` — the dual of `fold`

## Rust Application

Three approaches build infinite iterators. `naturals(start)` uses a range `start..` directly — ranges implement `Iterator` and are lazy. `fibs()` uses `std::iter::successors(Some((0,1)), |&(a,b)| Some((b, a+b)))` to thread state through each step. `primes()` composes `(2..).filter(|&n| is_prime(n))` — the filter only runs `is_prime` for each value when the caller asks for the next prime. `unfold` mirrors OCaml's `Seq.unfold` using `std::iter::from_fn` with a mutable `Option<S>` closure.

## OCaml Approach

OCaml's `Seq` module (added in 4.07) represents a lazy sequence as a thunk: `type 'a t = unit -> 'a node` where `node = Nil | Cons of 'a * 'a t`. Each `Cons` cell is a closure that produces the next element only when forced. `Seq.unfold f seed` repeatedly applies `f` returning `Some (value, next_seed)` or `None`. Unlike Rust, OCaml lists are strict by default, so `Seq` is the explicit opt-in for laziness.

## Key Differences

1. **Default laziness:** Rust iterators are lazy by default — `(0..)` is already an infinite sequence. OCaml lists are strict; `Seq` is a separate lazy abstraction added explicitly
2. **Representation:** OCaml `Seq` uses closures (`unit -> node`); Rust uses `Iterator` trait objects or `impl Iterator` return types — both avoid computing values until demanded
3. **`successors` vs `unfold`:** Rust's `std::iter::successors` is the direct equivalent of OCaml's `Seq.unfold`; both thread state through a step function returning `Option`
4. **Memory:** OCaml `Seq` nodes are heap-allocated closures; Rust iterators can be stack-allocated state machines with zero heap overhead

## Key Insight

OCaml added lazy sequences (`Seq`) in 4.07 as an explicit opt-in. Rust's iterators are lazy by default — `(0..)` creates an infinite range that only computes values when consumed. This means Rust doesn't need a separate "lazy" abstraction; the standard iterator *is* the lazy sequence.

## Exercises

1. Implement a `cycle` iterator that repeats a finite `Vec<T>` infinitely using `std::iter::from_fn`
2. Implement `zip_with` that combines two infinite iterators element-by-element with a function, producing a third infinite iterator
3. Implement a lazy `sieve_of_eratosthenes` that generates primes without the `is_prime` predicate — instead, filter out multiples of each new prime as it is discovered
