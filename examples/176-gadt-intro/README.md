📖 **[View on hightechmind.io →](https://hightechmind.io/rust/176-gadt-intro)**

---

# Introduction to GADTs
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Generalized Algebraic Data Types (GADTs) extend ordinary algebraic data types by allowing each constructor to specify a different return type for the type parameter. This enables type-safe expression trees where `eval` on `int expr` always returns `int`, not `int | bool`. GADTs were introduced in OCaml to bring the expressiveness of dependent types to mainstream functional programming. Rust simulates them via phantom types and sealed trait hierarchies.

## Learning Outcomes

- Understand what GADTs are and why they extend ordinary ADTs
- Learn Rust's GADT simulation: phantom type parameters + sealed marker types
- See how the `ExprType` sealed trait restricts valid type indices
- Understand why Rust's simulation requires `unreachable!()` branches that OCaml's GADTs eliminate

## Rust Application

The sealed module defines `ExprType` as a private supertrait, implemented only for `i64` and `bool`. `ExprInner` is an untyped enum; `Expr<T: ExprType>` wraps it with a phantom `T`. Smart constructors (`int_lit`, `bool_lit`, `add`, `equal`) set `T` correctly — the compiler rejects constructing an `Expr<bool>` with `AddF` because `add` returns `Expr<i64>`. The `eval` function on `Expr<T>` must handle all `ExprInner` variants, with `unreachable!()` for impossible cases guaranteed by the phantom type.

## OCaml Approach

OCaml's GADTs express this directly:
```ocaml
type _ expr =
  | IntLit : int -> int expr
  | BoolLit : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | Equal : int expr * int expr -> bool expr
let rec eval : type a. a expr -> a = function
  | IntLit n -> n
  | BoolLit b -> b
  | Add (l, r) -> eval l + eval r
  | Equal (l, r) -> eval l = eval r
```
The compiler knows that `eval` on `int expr` cannot match `BoolLit` — no `unreachable!()` needed.

## Key Differences

1. **Exhaustiveness**: OCaml's GADT pattern matching is exhaustive per index — impossible cases are excluded; Rust's phantom type simulation requires `unreachable!()` for dead branches.
2. **Constructor syntax**: OCaml's `: type expr` syntax is built into the language; Rust requires phantom parameters and smart constructors.
3. **Type variable scope**: OCaml's `type a. a expr -> a` scopes the type variable per clause; Rust uses trait bounds and associated types.
4. **Safety guarantee**: Both prevent constructing ill-typed expressions at compile time; OCaml's guarantee is stronger (no runtime dead-code branches).

## Exercises

1. Add a `Neg : int expr -> int expr` constructor and implement its evaluation.
2. Add `IfThenElse : bool expr -> 'a expr -> 'a expr -> 'a expr` — how does the type constraint on the two branches affect the Rust simulation?
3. Verify at compile time that `add(bool_lit(true), int_lit(1))` is rejected.
