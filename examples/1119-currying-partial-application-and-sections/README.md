# Example 024: Currying, Partial Application, and Sections

**Difficulty:** ⭐  
**Category:** Higher-Order Functions  
**OCaml Source:** [https://cs3110.github.io/textbook/chapters/hop/higher_order.html](https://cs3110.github.io/textbook/chapters/hop/higher_order.html)

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