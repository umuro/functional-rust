# Example 266: Recursive Descent Parser

**Difficulty:** ⭐⭐⭐
**Category:** Pattern Matching | Algebraic Data Types | Mutual Recursion
**OCaml Source:** Classic recursive descent parsing technique

## Problem Statement

Parse a stream of arithmetic tokens (numbers, `+`, `*`) into an abstract syntax tree (AST), then evaluate the tree — correctly handling operator precedence (multiplication binds tighter than addition).

## Learning Outcomes

- How recursive `enum` types in Rust require `Box` for heap allocation (OCaml boxes automatically)
- How mutually recursive free functions in Rust mirror OCaml's `and` keyword exactly
- How slice patterns (`["+", tail @ ..]`) replace OCaml's list head/tail pattern matching
- How a cursor-based `Parser` struct is the idiomatic Rust alternative to threading remainder slices

## OCaml Approach

OCaml encodes the AST as a sum type (`type expr = Num of int | Add of expr * expr | Mul of expr * expr`) and defines three mutually recursive functions with `let rec ... and ...`. Each function takes a token list and returns a `(expr, remaining_list)` pair, threading the unconsumed tokens through the call chain automatically via tuple returns.

## Rust Approach

Rust encodes the AST as an `enum Expr` with `Box<Expr>` children (since Rust requires sized recursive types). The functional solution uses three mutually recursive free functions that return `(Expr, &[&str])` pairs — a borrowed sub-slice replaces OCaml's immutable list tail with zero-copy efficiency. The struct-based solution uses a `Parser` with a `pos: usize` cursor, which is the pattern found in production Rust parsers.

## Key Differences

1. **Recursive types:** OCaml allocates ADT values implicitly; Rust requires `Box<Expr>` to make the recursive enum `Sized`.
2. **Token threading:** OCaml passes list tails by value (persistent lists share structure); Rust passes `&[&str]` sub-slices — a pointer/length pair with no allocation.
3. **Mutual recursion:** OCaml uses `let rec f … and g …`; Rust uses ordinary `fn` declarations since forward references are always visible within a module.
4. **Pattern matching on sequences:** OCaml matches `"+" :: rest'`; Rust matches `["+", tail @ ..]` — both destructure the head and bind the tail.
5. **Error handling:** OCaml uses `failwith`; Rust uses `panic!` in this example (production code would return `Result`).
