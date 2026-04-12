📖 **[View on hightechmind.io →](https://hightechmind.io/rust/069-unfold)**

---

# 069 — Unfold — Generate a Sequence from a Seed
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`unfold` is the categorical dual of `fold`: where `fold` (catamorphism) reduces a sequence to a single value by repeatedly applying a combining function, `unfold` (anamorphism) generates a sequence from a seed value by repeatedly applying a step function. Given an initial state `s` and a function `f: S -> Option<(T, S)>`, `unfold` produces values of type `T` and evolves the state `S` until `f` returns `None`.

This pattern appears throughout computing: database cursor iteration yields one row at a time from a state machine; pagination APIs return one page at a time, advancing a token; compilers lex one token at a time from a character stream; and stream decoders decode one frame at a time from a byte buffer. Any situation where you generate a sequence incrementally from evolving state is an `unfold`.

`unfold` appears as `Seq.unfold` in OCaml 4.11+, `unfoldr` in Haskell's `Data.List`, and `std::iter::successors` in Rust (a slightly restricted version where the state equals the output). Understanding unfold gives you a principled vocabulary for all sequence generation.

## Learning Outcomes

- Implement `unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T>` as a loop
- Use `std::iter::successors` as Rust's built-in unfold-like combinator when state equals output
- Recognize the difference between `successors` (state = value) and full unfold (state can differ)
- Generate finite sequences by returning `None` from the step function when done
- Recognize unfold as the dual of fold: catamorphism vs anamorphism in category theory
- Implement classic sequences (range, Fibonacci, Collatz) using the unfold pattern

## Rust Application

The manual `unfold` function uses a `while let` loop:
- While `f(state)` returns `Some((value, next_state))`, push `value` and advance state
- When `f` returns `None`, the loop ends — the sequence is complete

`range(a, b)` uses seed=a and step `|i| if i >= b { None } else { Some((i, i+1)) }`. `fibs_up_to(limit)` tracks `(a, b)` as state, yielding `a` and transitioning to `(b, a+b)`.

`std::iter::successors(Some(seed), |x| ...)` is Rust's built-in version where the state equals the output. For the Collatz sequence: `successors(Some(n), |&x| if x == 1 { None } else { Some(if x % 2 == 0 { x/2 } else { 3*x+1 }) })`.

`std::iter::from_fn(|| ...)` closes over `mut` variables — the most general Rust unfold for complex evolving state.

## OCaml Approach

OCaml 4.11+ has `Seq.unfold f seed` in the standard library returning a lazy sequence:

```ocaml
(* Range using Seq.unfold *)
let range a b = Seq.unfold (fun i -> if i >= b then None else Some (i, i+1)) a

(* Fibonacci using unfold *)
let fibs = Seq.unfold (fun (a, b) -> Some (a, (b, a+b))) (0, 1)
```

Earlier versions require a manual definition: `let rec unfold f s = match f s with | None -> [] | Some (x, s') -> x :: unfold f s'`. This is eager and not tail-recursive, so it can stack-overflow on very long sequences. The `Seq.unfold` version is lazy (thunk-based), so it handles infinite sequences safely.

## Key Differences

1. **`successors` vs full unfold**: Rust's `successors(Some(seed), f)` where `f: &T -> Option<T>` means the state is the same type as the output. OCaml's `Seq.unfold f seed` where `f: S -> Option<(T, S)>` separates state from output — more general but syntactically heavier.
2. **`from_fn` as general unfold**: `std::iter::from_fn(|| ...)` with closed-over mutable state is Rust's most general unfold. It handles cases where `successors` is too restrictive.
3. **Eagerness vs laziness**: The manual `unfold` in this example is eager — it collects everything into a `Vec`. Rust's `successors` and `from_fn` are lazy iterators. OCaml's `Seq.unfold` is lazy (thunk-based). Laziness matters for infinite sequences.
4. **Termination**: Both versions terminate when the step function returns `None`. Infinite sequences are handled by never returning `None` and using `.take(n)` to limit consumption.
5. **Stack safety**: OCaml's naive recursive `unfold` is not tail-recursive and will overflow on long sequences. Rust's loop-based implementation and lazy iterators are stack-safe.

## Exercises

1. **Binary representation**: Write `binary_digits(n: u64) -> Vec<u8>` using unfold that generates binary digits of n from least significant to most significant: seed=n, step `|x| if x == 0 { None } else { Some((x & 1, x >> 1)) }`.
2. **Newton's method**: Write `newton_sqrt(n: f64) -> impl Iterator<Item=f64>` that generates successive approximations of √n using `successors`: `x_next = (x + n/x) / 2`. Stop (in a consumer) when two successive values differ by less than 1e-10.
3. **Iterate combinator**: Write `iterate<T: Clone, F: Fn(&T) -> T>(f: F, seed: T) -> impl Iterator<Item=T>` producing the infinite sequence `seed, f(seed), f(f(seed)), ...`. This is Haskell's `iterate` and OCaml's `Seq.iterate`. Implement it using `successors`.
