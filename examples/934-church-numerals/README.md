📖 **[View on hightechmind.io →](https://hightechmind.io/rust/934-church-numerals)**

---

# 934-church-numerals — Church Numerals

## Problem Statement

Church numerals are a demonstration that numbers and arithmetic can be encoded purely as functions — no integers required. Introduced by Alonzo Church as part of lambda calculus (1930s), a Church numeral N is a function that applies another function N times: `zero = λf.λx. x`, `one = λf.λx. f x`, `two = λf.λx. f(f x)`. Successor, addition, and multiplication can all be defined in terms of function composition. This example is primarily educational, illustrating lambda calculus encoding in a typed language. It reveals why Rust requires `Rc` for shared closures and `Box<dyn Fn>` for type erasure.

## Learning Outcomes

- Understand Church numerals as function-encoded natural numbers
- Use `Box<dyn Fn>` for heap-allocated first-class functions
- Use `Rc` to share a closure across multiple calls without `Clone`
- Understand why OCaml represents Church numerals more naturally than Rust
- Recognize the connection between Church numerals and higher-order functions

## Rust Application

`type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>`. `zero()` ignores the function and returns the identity. `one()` applies the function once. `succ(n)` applies the function one more time than `n`. `add(m, n)` applies `f` through both `m` and `n`. The `Rc` sharing is necessary because the function `f` must be passed to both the `n` application and the extra application in `succ`. Converting to integers: `to_int(n) = n(|x| x + 1)(0)`.

## OCaml Approach

OCaml's uniform function representation makes Church numerals straightforward: `type church = { apply: 'a. ('a -> 'a) -> 'a -> 'a }`. `let zero = { apply = fun _f x -> x }`. `let succ n = { apply = fun f x -> f (n.apply f x) }`. `let to_int n = n.apply (fun x -> x + 1) 0`. OCaml's polymorphic `'a. ('a -> 'a) -> 'a -> 'a` is a rank-2 polymorphic type — it works for any type `'a`. Rust cannot express this without `Box<dyn Fn>` because closures have unique unnameable types.

## Key Differences

1. **Type erasure need**: Rust requires `Box<dyn Fn>` because each closure has a unique type; OCaml's uniform heap representation handles any closure as `('a -> 'b)`.
2. **Sharing with Rc**: Rust needs `Rc` to share `f` between multiple applications in `succ`; OCaml closures are implicitly shared.
3. **Rank-2 polymorphism**: OCaml can express `church = { apply: 'a. ('a -> 'a) -> 'a -> 'a }` directly; Rust cannot express rank-2 types without workarounds.
4. **Practical utility**: Church numerals are academic in both languages — the takeaway is understanding higher-order functions and type system limits.

## Exercises

1. Implement `church_mult(m: Church, n: Church) -> Church` that applies `m` times the application of `f` n times (composition).
2. Implement `church_to_bool` using Church booleans: `true = λt.λf. t`, `false = λt.λf. f`.
3. Write `church_pred` (predecessor function) — note this is significantly harder than successor due to the encoding constraints.
