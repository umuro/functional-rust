[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 098 — Church Numerals

## Problem Statement

Encode natural numbers as higher-order functions (Church numerals): zero applies `f` zero times, one applies once, `succ(n)` wraps `n` to apply one more time. Implement `to_int` (apply `+1` starting from 0) and arithmetic operations. Compare OCaml's naturally polymorphic Church encoding with Rust's `Box<dyn Fn>` approach.

## Learning Outcomes

- Understand Church numerals as functions: `n f x` applies `f` to `x` exactly `n` times
- Use `Box<dyn Fn(…) -> …>` to represent higher-order functions as values in Rust
- Understand why Rust's ownership makes direct Church numeral composition difficult
- Implement `to_int` by applying `|x| x + 1` to `0` via the Church numeral
- Compare Rust's `Box<dyn Fn>` verbosity with OCaml's terse polymorphic functions
- Recognise the `ChurchNum(usize)` practical encoding as a pragmatic alternative

## Rust Application

`type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>` is the Rust type. `zero()` returns a closure that ignores `f` and returns the identity. `to_int` applies `n(Box::new(|x| x + 1))` and calls the resulting closure with `0`. Direct `succ` composition is difficult because each `Box<dyn Fn>` consumes its closure, so `succ` falls back to integer manipulation (`to_int_inner(n) + 1`). The practical `ChurchNum(usize)` stores the count and applies `f` in a loop — semantically identical but sidestepping the ownership challenge.

## OCaml Approach

OCaml Church numerals are trivial: `let zero _f x = x` and `let succ n f x = f (n f x)`. The polymorphic type `('a -> 'a) -> 'a -> 'a` is inferred automatically. `add`, `mul`, and `exp` are single-line definitions. This is one of the most striking comparisons between the two languages: OCaml's rank-2 polymorphism handles Church numerals directly; Rust's monomorphic closures require boxing.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Type | `Box<dyn Fn(…) -> …>` (boxed) | `('a -> 'a) -> 'a -> 'a` (polymorphic) |
| `zero` | Multi-line with Box | `let zero _f x = x` |
| `succ` | Falls back to integer | `let succ n f x = f (n f x)` |
| `add` | Integer arithmetic | `let add m n f x = m f (n f x)` |
| Composition | Ownership prevents direct | Natural function application |
| Practical alt | `ChurchNum(usize)` struct | Not needed |

Church numerals illustrate a fundamental limitation: Rust's ownership system and lack of rank-2 polymorphism make the "pure" lambda-calculus encoding awkward. OCaml's Hindley-Milner type inference with polymorphic functions is a natural fit.

## Exercises

1. Implement `church_add(m: &Church, n: &Church) -> Church` using the integer-backed `from_int(to_int(m) + to_int(n))` approach.
2. Implement `church_mul` similarly and verify `church_mul(two, three)` equals `from_int(6)`.
3. Using the `ChurchNum` struct, implement `Mul<ChurchNum> for ChurchNum` using the `std::ops::Mul` trait.
4. In OCaml, implement `exp m n = n m` (Church exponentiation: `m^n`) and verify `exp two three = to_int 8`.
5. Research why Rust cannot express Church numerals directly (hint: requires rank-2 polymorphism or GATs) and write a brief explanation.
