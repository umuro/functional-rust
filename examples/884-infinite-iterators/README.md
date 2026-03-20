📖 **[View on hightechmind.io →](https://hightechmind.io/rust/884-infinite-iterators)**

---

# 884-infinite-iterators — Infinite Iterators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Many algorithms require generating sequences without a predetermined endpoint: cycling through a palette, repeating a default value, generating an arithmetic sequence of any length. In languages without lazy evaluation, this requires explicit loop counters and early exits. Rust provides `std::iter::repeat`, `repeat_with`, `cycle`, and `successors` as built-in infinite iterator constructors. These integrate naturally with `.take(n)` to produce finite results. Haskell's `iterate` and OCaml's `Seq.unfold` serve the same role. This example surveys all the standard infinite iterator patterns in Rust.

## Learning Outcomes

- Use `repeat`, `repeat_with`, and `cycle` for constant and periodic sequences
- Use `std::iter::successors` to implement OCaml's `iterate` function
- Use `from_fn` with mutable state for stateful infinite generators
- Implement a generic `unfold` using `from_fn` for functional-style generation
- Recognize when infinite iterators simplify code compared to explicit loops

## Rust Application

`repeat(42).take(5)` produces five copies. `cycle([1,2,3]).take(7)` cycles through the array. `repeat_with` uses a mutable closure for stateful generation (doubling sequence). `successors(Some(init), move |prev| Some(f(prev)))` implements Haskell's `iterate`. `doubles_from(n)` uses successors for `n, 2n, 4n, ...`. The generic `unfold<T, S, F>` wraps `from_fn` with explicit state threading — the functional dual of fold. `fibonacci_unfold` demonstrates unfold for the Fibonacci sequence.

## OCaml Approach

OCaml's equivalent is `Seq.unfold: ('a -> ('b * 'a) option) -> 'a -> 'b Seq.t`. `Seq.repeat x = Seq.unfold (fun () -> Some(x, ())) ()`. `Seq.cycle` is not in standard OCaml but trivially expressible via `Seq.unfold` with modular index state. `iterate f x = Seq.unfold (fun s -> Some(s, f s)) x`. The OCaml approach is more uniform (everything is `unfold`) while Rust provides specialized constructors for common patterns.

## Key Differences

1. **API breadth**: Rust has `repeat`, `repeat_with`, `cycle`, `successors`, `from_fn` as distinct constructors; OCaml unifies everything under `Seq.unfold`.
2. **Termination**: Rust `successors` terminates when the closure returns `None`; `repeat` and `cycle` never terminate — `.take(n)` is required.
3. **Stateful generators**: Rust `from_fn` captures mutable state in a closure; OCaml threads state as the unfold seed.
4. **Performance**: Rust's `repeat` and `cycle` are zero-allocation; OCaml's `Seq` allocates a new closure thunk per step.

## Exercises

1. Implement `powers_of_two` as an infinite iterator and use it to find the first power of 2 exceeding one million.
2. Use `cycle` to implement `round_robin<T: Clone>(vecs: &[Vec<T>]) -> impl Iterator<Item = T>` that cycles through elements in rotation.
3. Implement `collatz_lengths()` as an infinite iterator yielding the Collatz sequence length for n = 1, 2, 3, ...
