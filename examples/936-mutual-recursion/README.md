📖 **[View on hightechmind.io →](https://hightechmind.io/rust/936-mutual-recursion)**

---

# 936-mutual-recursion — Mutual Recursion
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Mutually recursive functions call each other: `is_even(n) = (n == 0) || is_odd(n-1)` and `is_odd(n) = (n != 0) && is_even(n-1)`. Neither function can be defined in terms of only itself; they require simultaneous definition. In OCaml, `let rec is_even n = ... and is_odd n = ...` uses `and` to co-define them. Rust does not need special syntax — any two functions in the same module can call each other freely since function names are resolved at link time, not at definition time. This difference reveals a fundamental design choice between definition-order-sensitive and definition-order-free languages.

## Learning Outcomes

- Implement mutually recursive functions in Rust without special syntax
- Understand why OCaml requires `let rec ... and ...` while Rust does not
- Apply mutual recursion to expression tree evaluation with multiple constructors
- Recognize when mutual recursion clarifies problem structure vs when a single recursive function suffices
- Compare Rust's implicit mutual recursion support with OCaml's explicit `and` keyword

## Rust Application

`is_even` and `is_odd` are defined as two separate `pub fn` — each calls the other by name. The compiler accepts this because function definitions are globally visible within the module. The `Expr` enum shows a more complex mutual recursion: `eval_expr` dispatches to the `Lit`, `Add`, and `Mul` cases, with `Add` and `Mul` recursively calling `eval_expr` on their sub-expressions. A formal grammar for arithmetic expressions also uses mutual recursion: `expr -> term ('+' term)*`, `term -> factor ('*' factor)*`.

## OCaml Approach

OCaml requires `let rec is_even n = ... and is_odd n = ...` because OCaml processes definitions sequentially. Without `and`, `is_odd` is not in scope when `is_even` is defined. `let rec ... and ...` co-defines both simultaneously. This makes the mutual dependency explicit and machine-verifiable. For type definitions: `type even_tree = Even | ENode of odd_tree and odd_tree = Odd | ONode of even_tree` works similarly. The `and` keyword appears for both `let rec` and `type` co-definitions.

## Key Differences

1. **Syntax**: OCaml requires explicit `let rec ... and ...` for mutual recursion; Rust uses ordinary `fn` with no special syntax.
2. **Definition order**: OCaml is definition-order-sensitive — forward references require `and`; Rust resolves names globally within the module regardless of definition order.
3. **Type co-definition**: OCaml `type a = ... and b = ...` for mutually recursive types; Rust allows `struct A { b: Box<B> }` followed by `struct B { a: Box<A> }` in any order.
4. **Clarity**: OCaml's explicit `and` documents mutual dependencies; Rust's approach requires reading both functions to discover the mutual dependency.

## Exercises

1. Implement a mutual recursive descent parser for arithmetic with `parse_expr`, `parse_term`, and `parse_factor` functions.
2. Write mutually recursive `is_even_count(list: &[i32]) -> bool` and `is_odd_count(list: &[i32]) -> bool` that check parity of the list length.
3. Define a mutually recursive type `JsonLike = Null | Bool(bool) | Num(f64) | Arr(Vec<JsonLike>) | Obj(Vec<(String, JsonLike)>)` and implement a depth-counting function.
