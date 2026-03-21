📖 **[View on hightechmind.io →](https://hightechmind.io/rust/232-sum-types)**

---

# Sum Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A sum type (also called a tagged union, discriminated union, or variant type) holds a value of exactly one of several possible types. `Result<T, E>` holds either a `T` or an `E`. `Option<T>` holds either a `T` or nothing. A `Shape` holds either a `Circle` or a `Rectangle`. Sum types are the fundamental mechanism for modeling data with alternatives, replacing nullable pointers, error codes, and untagged unions with type-safe, exhaustively checkable alternatives.

## Learning Outcomes

- Understand sum types as the dual of product types: "this OR that" vs. "this AND that"
- Learn how Rust enums implement sum types with associated data
- See exhaustive pattern matching as the consumer of sum types
- Understand the type algebra: `|A| + |B|` inhabitants for `Either<A, B>`

## Rust Application

Rust's `enum` is a sum type: `enum Shape { Circle(f64), Rect(f64, f64) }` has two variants, each with its own data. Pattern matching with `match` is exhaustive — the compiler rejects unhandled cases. `Option<T>` is `enum Option { None, Some(T) }`; `Result<T, E>` is `enum Result { Ok(T), Err(E) }`. Recursive sum types like `List<T> { Nil, Cons(T, Box<List<T>>) }` build data structures. The `#[non_exhaustive]` attribute allows libraries to add variants without breaking downstream code.

## OCaml Approach

OCaml's variant types are the original inspiration for Rust's enums:
```ocaml
type shape = Circle of float | Rect of float * float
let area = function
  | Circle r -> Float.pi *. r *. r
  | Rect (w, h) -> w *. h
```
OCaml's variants are exhaustively matched with `match` — unhandled cases produce a warning (or error with `-w +8`). OCaml also has polymorphic variants (`` `Circle ``...) for open-world sum types that can be extended without modifying the original type.

## Key Differences

1. **Syntax**: OCaml's `type t = A | B of int` vs. Rust's `enum T { A, B(i32) }` — structurally identical, syntactically different.
2. **Polymorphic variants**: OCaml's `` [> `A | `B] `` provides open sum types; Rust's enums are always closed (unless `#[non_exhaustive]`).
3. **Exhaustiveness**: Both require exhaustive matching; OCaml warns by default, errors with flags; Rust errors by default.
4. **Data attachment**: Both allow per-variant associated data with arbitrary types; OCaml's tuple syntax `A of int * string` vs. Rust's `A(i32, String)` or named fields.

## Exercises

1. Define a `Token` sum type for a simple expression language: `Int(i64)`, `Plus`, `Minus`, `LParen`, `RParen`.
2. Implement `eval: Expr -> i64` for a recursive `Expr` type using exhaustive pattern matching.
3. Add `#[non_exhaustive]` to a public enum and verify that downstream matches require a wildcard arm.
