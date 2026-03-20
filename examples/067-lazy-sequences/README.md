📖 **[View on hightechmind.io →](https://hightechmind.io/rust/067-lazy-sequences)**

---

# 067 — Lazy Sequences (Seq Module)

## Problem Statement

Lazy sequences represent potentially infinite collections where elements are computed on demand. OCaml 4.07 added the `Seq` module for this purpose; Haskell uses lazy lists by default. The key insight: you can define `let naturals = 0, 1, 2, ...` as an infinite sequence and then `take 10` to get the first 10 elements — without ever computing element 11.

Lazy sequences appear in: streaming data processing (reading lines from a file without loading it all), infinite mathematical sequences (primes, Fibonacci), event streams, and any scenario where the full sequence may not be needed or may not fit in memory. Rust's `Iterator` trait is inherently lazy.

## Learning Outcomes

- Use `std::iter::successors` to generate sequences from a seed and step function
- Use `std::iter::from_fn` for sequences with mutable state
- Implement infinite sequences that only compute values when consumed
- Chain lazy operations (`filter`, `map`, `take`) without intermediate allocation
- Implement a finite `unfold` that stops when the state function returns `None`

## Rust Application

`naturals(start)` returns `start..` — Rust's built-in infinite range. `fibs()` uses `successors(Some((0, 1)), |&(a, b)| Some((b, a + b))).map(|(a, _)| a)` — generates Fibonacci pairs lazily. `primes()` uses `(2..).filter(|&n| (2..=sqrt(n)).all(|i| n % i != 0))` — trial division, computed per-element. `unfold` manually drives a state function.

## OCaml Approach

OCaml 4.07+ `Seq` module: `let rec naturals n () = Seq.Cons (n, naturals (n + 1))`. Taking n elements: `Seq.take 10 (naturals 0) |> List.of_seq`. Fibonacci: `let rec fibs a b () = Seq.Cons (a, fibs b (a + b))`. OCaml's `Seq` is a thunk-based lazy sequence: each element is a `unit -> Seq.node` function, computed on demand.

## Key Differences

1. **Iterator vs Seq**: Rust's `Iterator` is inherently lazy — any iterator pipeline is a lazy sequence. OCaml's lists are eager; `Seq` is the explicit lazy alternative.
2. **Infinite iterators**: Rust's `0..` is an infinite range. OCaml's lists are always finite; `Seq` is needed for infinite sequences.
3. **`successors` vs recursive Seq**: Rust's `successors(seed, f)` generates a sequence by repeatedly applying `f`. OCaml's recursive `Seq` nodes use closures/thunks to delay computation.
4. **`take` and `collect`**: Rust: `.take(n).collect::<Vec<_>>()`. OCaml: `Seq.take n seq |> List.of_seq`. Both materialize n elements from a potentially infinite sequence.

## Exercises

1. **Sieve stream**: Write `prime_stream()` returning an infinite iterator of primes using the sieve of Eratosthenes implemented as a lazy stream (not trial division). Each new prime filters multiples from the remaining stream.
2. **Collatz stream**: Write `collatz_seq(n: u64) -> impl Iterator<Item=u64>` that yields the Collatz sequence starting from n until 1. Use `successors`.
3. **Memoized Fibonacci**: Write a Fibonacci iterator that caches previously computed values using a `Vec` internally. Compare performance with the `successors`-based version for large n.
