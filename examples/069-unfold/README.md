📖 **[View on hightechmind.io →](https://hightechmind.io/rust/069-unfold)**

---

# 069 — Unfold — Generate a Sequence from a Seed

## Problem Statement

`unfold` is the dual of `fold`: where `fold` reduces a sequence to a value, `unfold` generates a sequence from a seed value. Given an initial state and a step function `f: S -> Option<(T, S)>`, `unfold` produces `T` values until `f` returns `None`. It is the anamorphism to fold's catamorphism.

`unfold` appears as `Seq.unfold` in OCaml 4.11+, `List.unfold` in Haskell, and `std::iter::successors` in Rust (slightly restricted version). It is used to generate ranges, Fibonacci sequences, Collatz sequences, iteration traces, and pagination responses.

## Learning Outcomes

- Implement `unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T>`
- Use `std::iter::successors` as Rust's built-in unfold-like combinator
- Generate finite sequences by returning `None` when done
- Recognize unfold as the dual of fold (catamorphism vs anamorphism)
- Implement classic sequences (range, Fibonacci, Collatz) using unfold

## Rust Application

Manual `unfold`: loop while `f(state)` returns `Some((value, next_state))`, pushing `value` and updating state. `range(a, b)` via unfold: seed=a, step `|i| if i >= b { None } else { Some((i, i+1)) }`. `fibs_up_to(limit)` tracks `(a, b)` as state. `std::iter::successors(Some(n), |&x| ...)` is the built-in version: generates `n, f(n), f(f(n)), ...` until `None`.

## OCaml Approach

OCaml 4.11+ has `Seq.unfold f seed`: `Seq.unfold (fun i -> if i >= b then None else Some (i, i + 1)) a |> List.of_seq`. Earlier versions define it manually: `let rec unfold f s = match f s with None -> [] | Some (x, s') -> x :: unfold f s'`. The `Collatz` sequence: `unfold (fun n -> if n = 1 then None else Some (n, if n mod 2 = 0 then n/2 else 3*n+1)) start`.

## Key Differences

1. **`successors` vs `unfold`**: Rust's `successors(Some(seed), f)` where `f: &T -> Option<T>` — the state IS the value. OCaml's `Seq.unfold f seed` where `f: S -> Option<(T, S)>` — the state can differ from the output type. `successors` is less general.
2. **From_fn**: `std::iter::from_fn(|| ...)` with a closure captures mutable state — the most general Rust unfold. It can produce any sequence by closing over `mut` variables.
3. **Laziness**: Rust's `successors`/`from_fn` are lazy iterators. OCaml's `Seq.unfold` is lazy (thunk-based). The manual `unfold` function in this example is eager (returns `Vec`).
4. **Termination**: Both versions terminate when the step function returns `None`. Infinite sequences are possible by never returning `None` — limit them with `.take(n)`.

## Exercises

1. **Binary representation**: Write `binary_digits(n: u64) -> Vec<u8>` using unfold that generates the binary digits of n from least significant to most significant.
2. **Newton's method**: Write `newton_sqrt(n: f64) -> impl Iterator<Item=f64>` that generates successive approximations of √n using Newton's method: `x_next = (x + n/x) / 2`. Use `successors`.
3. **Iterate**: Write `iterate<T: Clone, F: Fn(T) -> T>(f: F, seed: T) -> impl Iterator<Item=T>` — the infinite sequence `seed, f(seed), f(f(seed)), ...`. This is Haskell's `iterate` function.
