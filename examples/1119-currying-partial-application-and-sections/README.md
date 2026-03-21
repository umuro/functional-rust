# Example 024: Currying, Partial Application, and Sections
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Show how OCaml's automatic currying, partial application, and operator sections can be emulated in Rust.

## Learning Outcomes

- How to emulate curried functions using closures in Rust
- How to create partially applied functions from general ones
- How to simulate labeled arguments in Rust using structs or factory closures
- The difference between Rust's uncurried functions and OCaml's curried-by-default functions

## OCaml Approach

OCaml functions are curried by default: every function takes exactly one argument and returns either a result or another function. This enables partial application for free, and operator sections let you treat infix operators as prefix functions.

## Rust Approach

Rust functions are not curried. Idiomatic Rust uses multi‑argument functions (tuples) and explicit closures for partial application. The example shows two styles: idiomatic Rust (closures that capture their environment) and a more functional style that mimics currying.

## Key Differences

1. **Currying:** OCaml does it automatically; Rust requires explicit closure chains.
2. **Partial Application:** OCaml uses `add 5`; Rust uses `|y| add(5, y)` or a factory function.
3. **Labeled Arguments:** OCaml has named parameters; Rust uses structs or separate closures.
4. **Operator Sections:** OCaml can write `( * ) 2`; Rust can define `|x| x * 2`.
## Exercises

1. Partially apply a curried `clamp` function `(i32, i32, i32) -> i32` to create a `clamp_0_100` specialization, and map it over a `Vec<i32>`.
2. Write a `compose` higher-order function and demonstrate that `(compose(f, g))(x) == f(g(x))` holds for string processing functions.
3. Build a query-filter DSL using partially applied predicates: define `eq`, `gt`, `lt` as curried comparators and use `filter` with partial application to select records from a `Vec<Row>`.
