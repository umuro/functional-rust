📖 **[View on hightechmind.io →](https://hightechmind.io/rust/198-church-encoding)**

---

# Church Encoding
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Church encoding represents data and operations as pure functions (lambdas). Church numerals encode natural numbers: zero is `λf.λx.x`, one is `λf.λx.f x`, two is `λf.λx.f (f x)`. Booleans, pairs, and lists can all be encoded as functions. This demonstrates that the lambda calculus is computationally complete — data and control flow are the same thing. Understanding Church encoding illuminates the foundation of functional programming and type theory.

## Learning Outcomes

- Understand Church encoding: representing data as higher-order functions
- Learn Church numerals, booleans, and pairs as function-based encodings
- See how arithmetic (successor, add, multiply) works on Church numerals
- Appreciate the theoretical foundation: types and proofs correspond (Curry-Howard, example 233)

## Rust Application

In Rust, Church numerals are encoded as `type ChurchNum = Box<dyn Fn(Box<dyn Fn(u32) -> u32>) -> Box<dyn Fn(u32) -> u32>>`. The encoding is verbose due to Rust's explicit function types. `zero = |f| |x| x`, `succ = |n| |f| |x| f(n(f)(x))`. To convert to `u32`: `church_to_int(n) = n(|x| x + 1)(0)`. Rust's `impl Fn` and trait objects are required because functions must be heap-allocated for higher-order composition.

## OCaml Approach

OCaml's Church encoding is more concise:
```ocaml
let zero f x = x
let succ n f x = f (n f x)
let add m n f x = m f (n f x)
let to_int n = n (fun x -> x + 1) 0
```
OCaml's type inference handles the complex function types automatically. The encoding is idiomatic OCaml and is used in courses on the lambda calculus and type theory.

## Key Differences

1. **Type verbosity**: OCaml infers Church numeral types; Rust requires explicit `Box<dyn Fn(...)>` types that are hard to write.
2. **Performance**: Church encoding is purely theoretical — computing 10 + 10 requires O(n) function applications; no practical use in performance-sensitive code.
3. **Rank-2 types**: Full Church encoding requires rank-2 types for polymorphic church numerals; both Rust and OCaml have limited rank-2 support (OCaml via record polymorphism, Rust via trait bounds).
4. **Educational value**: Church encoding demonstrates lambda calculus completeness; it is taught in CS theory courses regardless of practical utility.

## Exercises

1. Implement Church booleans: `true_ = |t| |f| t`, `false_ = |t| |f| f`, and `if_ = |cond| |then_| |else_| cond(then_)(else_)`.
2. Implement Church pairs: `pair = |x| |y| |f| f(x)(y)`, `fst = |p| p(|x| |_| x)`, `snd = |p| p(|_| |y| y)`.
3. Implement predecessor for Church numerals (this is famously tricky) using the pair encoding.
