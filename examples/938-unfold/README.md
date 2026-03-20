📖 **[View on hightechmind.io →](https://hightechmind.io/rust/938-unfold)**

---

# 938-unfold — Unfold
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Fold (catamorphism) consumes a recursive structure into a value. Unfold (anamorphism) generates a recursive structure from a seed value. They are dual operations. Unfold takes a seed `S` and a function `f: S -> Option<(T, S)>`: if `f(s) = None`, the sequence ends; if `f(s) = Some(value, next_state)`, emit `value` and continue with `next_state`. This generates sequences: ranges, Collatz sequences, Fibonacci, countdowns. Haskell has `unfoldr`; OCaml has `Seq.unfold`. Rust's `std::iter::from_fn` and custom iterators serve the same role. Unfold is the generative dual of fold.

## Learning Outcomes

- Implement the generic `unfold<T, S, F>` function
- Generate sequences from seed values: range, countdown, Collatz, Fibonacci
- Understand `Option<(T, S)>` as the protocol: `None` terminates, `Some((value, next))` continues
- Recognize unfold as the dual of fold (anamorphism vs catamorphism)
- Compare with OCaml's `Seq.unfold` and Haskell's `Data.List.unfoldr`

## Rust Application

`unfold<T, S, F>(seed, f)` loops: `while let Some((value, next)) = f(state.clone()) { result.push(value); state = next; }`. `range(a, b)` uses `unfold(a, |i| if i > b { None } else { Some((i, i+1)) })`. `countdown(n)` generates decreasing sequence. `collatz(n)` generates the full Collatz sequence until 1. The custom `unfold` returns a `Vec` for simplicity; a lazy version would return an iterator using `from_fn`.

## OCaml Approach

`Seq.unfold: ('a -> ('b * 'a) option) -> 'a -> 'b Seq.t` is the direct equivalent (since 4.11). `let range a b = Seq.unfold (fun i -> if i > b then None else Some(i, i+1)) a`. Fibonacci: `Seq.unfold (fun (a,b) -> Some(a, (b, a+b))) (0, 1)`. OCaml's `Seq.unfold` is lazy — elements are generated on demand. Converting to list: `Seq.take n seq |> List.of_seq`. Haskell's `unfoldr` uses `(a -> Maybe (b, a))` — same protocol, different Maybe syntax.

## Key Differences

1. **Laziness**: Rust's custom `unfold` is eager (returns `Vec`); OCaml's `Seq.unfold` is lazy; Rust's `from_fn` closure enables lazy unfold.
2. **State cloning**: The eager Rust version needs `S: Clone` to apply `f` and keep state; the lazy iterator version can take ownership.
3. **Protocol**: Both use `Option<(value, next_state)>` — same semantic protocol, just `Option` vs `None/Some`.
4. **Standard library**: OCaml has `Seq.unfold` since 4.11; Rust uses `std::iter::from_fn` or custom iterators — no direct `unfold` function in std.

## Exercises

1. Implement a lazy `unfold_iter<T, S, F: Fn(S) -> Option<(T, S)>>` using `std::iter::from_fn` that generates values on demand.
2. Use `unfold` to generate the sequence of perfect squares below 1000.
3. Implement `unfold_tree<T, S, F>(seed: S, f: F) -> Tree<T>` where `f(s) = None` creates a leaf and `f(s) = Some(value, left_seed, right_seed)` creates a node.
