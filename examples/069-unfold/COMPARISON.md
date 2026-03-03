# Unfold — OCaml vs Rust Comparison

## Core Insight

Unfold is the categorical dual of fold: where fold consumes a structure into a summary, unfold generates a structure from a seed. OCaml expresses it naturally as a recursive function returning a list. Rust can do the same but also has built-in lazy unfold via `std::iter::successors`.

## OCaml Approach

A simple recursive function: call `f seed`, if `Some(value, next)` then cons `value` onto the recursive result. Clean and elegant, but not tail-recursive — deep sequences can overflow the stack. OCaml's `Seq.unfold` provides a lazy version.

## Rust Approach

Two options: (1) A custom `unfold` function that collects into `Vec` — eagerly evaluates the entire sequence. (2) `std::iter::successors` or `std::iter::from_fn` for lazy evaluation — generates values on demand. The lazy version is more idiomatic for potentially large or infinite sequences.

## Comparison Table

| Aspect        | OCaml                           | Rust                                  |
|---------------|---------------------------------|---------------------------------------|
| **Memory**    | Cons cells (linked list)        | Vec (contiguous) or iterator (lazy)   |
| **Null safety** | `Option` for termination     | `Option` for termination              |
| **Errors**    | Stack overflow on deep unfold   | Vec grows dynamically (no overflow)   |
| **Iteration** | Recursive                       | `while let` loop or iterator chain    |
| **Laziness**  | `Seq.unfold` (separate module)  | `successors` / `from_fn` (built-in)   |

## Things Rust Learners Should Notice

1. **`while let Some(...)` pattern** — idiomatic for consuming option-producing functions
2. **`S: Clone` bound** — needed because the state must be passed to `f` while we check the result
3. **`std::iter::successors`** — built-in lazy unfold, returns an iterator
4. **Eager vs lazy** — our `unfold` builds a Vec immediately; `successors` is lazy
5. **Termination** — `None` from the function signals end of sequence in both languages

## Further Reading

- [std::iter::successors](https://doc.rust-lang.org/std/iter/fn.successors.html)
- [std::iter::from_fn](https://doc.rust-lang.org/std/iter/fn.from_fn.html)
- [Anamorphism (Wikipedia)](https://en.wikipedia.org/wiki/Anamorphism)
