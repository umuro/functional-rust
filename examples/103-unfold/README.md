📖 **[View on hightechmind.io →](https://hightechmind.io/rust/103-unfold)**

---

# 103-unfold — Unfold: Generating Sequences from Seeds

## Problem Statement

`fold` reduces a sequence to a single value. `unfold` is its dual: it generates a sequence from a seed value by repeatedly applying a step function until the function returns `None`. This is the mathematical concept of anamorphism — the category-theoretic dual of catamorphism (fold).

`unfold` appears in Haskell's `Data.List.unfoldr`, OCaml's `Seq.unfold`, and Rust's `std::iter::from_fn` and `Iterator::scan`. It elegantly expresses infinite sequences, state machines, and recursive data generation.

## Learning Outcomes

- Understand `unfold` as the dual of `fold`
- Implement `unfold` using a seed value and step function
- Generate finite and infinite sequences with `unfold`
- Recognize `unfold` in Rust's `std::iter::from_fn` and `Iterator::scan`
- Apply `unfold` to classic sequences: ranges, Collatz, Fibonacci

## Rust Application

`src/lib.rs` implements `unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T>`. Each call to `f` either yields the next value plus the new state, or returns `None` to end the sequence. `range`, `countdown`, and `collatz` are built from this one combinator.

Rust's standard library provides `std::iter::from_fn(|| ...)` for stateful generators and `std::iter::successors(first, |prev| ...)` for sequences where each element depends on the previous — these are specialized forms of unfold.

## OCaml Approach

OCaml's `Seq.unfold` does this lazily:

```ocaml
let range a b =
  Seq.unfold (fun i -> if i > b then None else Some (i, i + 1)) a
  |> List.of_seq

let collatz n =
  Seq.unfold (function
    | 0 -> None
    | 1 -> Some (1, 0)
    | n -> Some (n, if n mod 2 = 0 then n / 2 else 3 * n + 1)
  ) n |> List.of_seq
```

OCaml's version is lazy by default — the sequence is only computed as far as it is consumed, enabling infinite sequences without running out of memory.

## Key Differences

1. **Laziness**: OCaml's `Seq.unfold` is lazy; Rust's `unfold` in this example is strict (collects into `Vec`). Rust's `from_fn` / `successors` are lazy iterators.
2. **Infinite sequences**: OCaml's lazy `Seq` handles infinite sequences naturally; Rust requires lazy iterators (`from_fn`) and explicit `take(n)` to truncate.
3. **State type**: Both take a seed/state and a step function; the type signature is identical in spirit.
4. **Standard library**: OCaml's `Seq.unfold` is in stdlib since 4.11; Rust's `std::iter::successors` is the closest built-in equivalent.

## Exercises

1. Use `std::iter::successors` to implement `fibonacci_iter() -> impl Iterator<Item=u64>` as an infinite sequence.
2. Implement `unfold_lazy<S: Clone, T>(seed: S, f: impl Fn(&S) -> Option<(T, S)>) -> impl Iterator<Item=T>` using `from_fn`.
3. Use `unfold` to generate the binary representation of a number (most significant digit first) by repeatedly dividing by 2.
