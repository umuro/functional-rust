[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 082 — Type Aliases
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Use `type` aliases to give shorter, more descriptive names to complex type expressions. Define `Point`, `Name`, `AppResult<T>`, `Predicate<T>`, and `Transform<T>` — reducing repetition in signatures and improving readability — and compare with OCaml's equivalent `type` declarations.

## Learning Outcomes

- Write type aliases with `type Name = ...` in Rust
- Create generic aliases like `type AppResult<T> = Result<T, AppError>`
- Understand that aliases are transparent: the compiler sees through them
- Distinguish type aliases (transparent) from newtypes (opaque wrapper structs)
- Use `type Predicate<T> = Box<dyn Fn(&T) -> bool>` for complex closure types
- Map Rust aliases to OCaml `type 'a result_t = ('a, error) result`

## Rust Application

`type Point = (f64, f64)` renames a tuple type for use in `distance`. `type AppResult<T> = Result<T, AppError>` shortens every return type that uses `AppError` — a pattern used throughout the standard library (`std::io::Result<T>`). `type Predicate<T> = Box<dyn Fn(&T) -> bool>` gives a readable name to the filter closure type. Aliases are fully transparent: `AppResult<i32>` and `Result<i32, AppError>` are the same type and can be used interchangeably. They exist only for human readability.

## OCaml Approach

OCaml's `type point = float * float` and `type 'a predicate = 'a -> bool` serve the same purpose. Parameterised aliases use the `'a` syntax: `type 'a result_t = ('a, error) result`. OCaml's type system treats aliases as identical to their expansion — no subtyping, no coercion needed. The notation is slightly different (`'a result_t` vs `AppResult<T>`) but the semantics are identical.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Syntax | `type Name = Type` | `type name = type` |
| Generic | `type Foo<T> = ...` | `type 'a foo = ...` |
| Transparency | Fully transparent | Fully transparent |
| vs newtype | `struct Meters(f64)` is opaque | `type meters = Meters of float` is opaque |
| Common use | `io::Result<T>`, `Predicate<T>` | `'a option`, custom result aliases |
| Closure alias | `Box<dyn Fn(...)>` needed | First-class function type `'a -> 'b` |

The key distinction: type aliases are purely cosmetic and transparent; newtypes (tuple structs in Rust, single-constructor variants in OCaml) create genuinely new types with type-checking separation. Use an alias when you want shorter notation; use a newtype when you want compile-time separation.

## Exercises

1. Define `type Matrix = Vec<Vec<f64>>` and write a `transpose(m: &Matrix) -> Matrix` function using it.
2. Create `type Parser<T> = Box<dyn Fn(&str) -> Option<(T, &str)>>` and implement a digit parser and a letter parser.
3. Write `type ResultVec<T, E> = Vec<Result<T, E>>` and a function `partition_results` that splits it into `(Vec<T>, Vec<E>)`.
4. Demonstrate the transparency: write a function that accepts `AppResult<i32>` and call it with `Result<i32, AppError>` directly.
5. In OCaml, define `type ('a, 'b) either = Left of 'a | Right of 'b` and write a `partition_eithers` function. Compare this design with Rust's `Result`.
