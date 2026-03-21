📖 **[View on hightechmind.io →](https://hightechmind.io/rust/199-scott-encoding)**

---

# Scott Encoding
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Scott encoding is an alternative to Church encoding that represents data as pattern matches — a function that takes one continuation per constructor. Where Church encoding ties data to iteration, Scott encoding ties data to pattern matching. Scott-encoded natural numbers and lists support O(1) `head` and `tail` (Church lists need O(n) to access the tail), making Scott encoding more practical for functional data structures. It is the basis for how some functional compilers represent data types at runtime.

## Learning Outcomes

- Understand Scott encoding as "data as its own eliminator"
- Compare Scott encoding to Church encoding: pattern match vs. iteration
- See how `match` in functional languages maps to Scott's continuation-passing pattern
- Learn why Scott encoding is preferable to Church encoding for pattern-matching-heavy code

## Rust Application

Scott-encoded natural numbers: `type ScottNat = Box<dyn Fn(Box<dyn Fn() -> A>) -> Box<dyn Fn(Box<dyn Fn(ScottNat) -> A>) -> A>>`. Zero is `|zero_case| |_| zero_case()`. Successor `n` is `|_| |succ_case| succ_case(n)`. To match: `scott_match(n, on_zero, on_succ)` applies `n` to the two continuations. Unlike Church numerals, `predecessor(n)` is O(1): `scott_match(n, || zero, |prev| prev)`.

## OCaml Approach

OCaml's variant types are the standard encoding; Scott encoding is the lambda calculus explanation of how they work:
```ocaml
(* Scott natural number: one continuation per constructor *)
let zero zero_c _succ_c = zero_c
let succ n _zero_c succ_c = succ_c n
let predecessor n = n (fun () -> zero) (fun prev -> prev)
```
In OCaml, this is purely educational — actual code uses `type nat = Zero | Succ of nat` with pattern matching.

## Key Differences

1. **Pattern match access**: Scott encoding gives O(1) `head`/`tail` (pattern match access); Church encoding gives O(1) fold but O(n) pattern match equivalent.
2. **Practical use**: Scott encoding is how GHC's Core represents algebraic data types internally — each constructor is a "case expression" applicand.
3. **Type verbosity**: Scott encoding in Rust requires deeply nested `Box<dyn Fn>` types; OCaml infers all types automatically.
4. **Elimination principle**: Church encoding embeds the iteration scheme (fold); Scott encoding embeds the pattern match (case expression) — mathematically dual approaches.

## Exercises

1. Implement Scott-encoded lists with `nil`, `cons`, `head`, and `tail` operations.
2. Verify that `predecessor(succ(succ(zero)))` produces `succ(zero)` using `scott_to_int`.
3. Implement `add_scott` for Scott naturals — it will be more complex than Church's `add` because you need to recurse differently.
