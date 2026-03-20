# Example 024: Currying, Partial Application, and Operator Sections

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** Cornell CS3110 — Functional Programming in OCaml

## Problem Statement

Demonstrate how OCaml's automatic currying and partial application translate
to Rust's explicit closure-capture model. Show `curry`/`uncurry` converters,
`flip` for argument reordering, operator sections, and function pipelines.

## Learning Outcomes

- How to express partial application in Rust via closures that capture arguments
- Why `Box<dyn Fn>` is needed when returning closures from generic higher-order functions
- How `curry` and `uncurry` convert between tupled and sequential calling styles
- How `flip` (argument-order swap) enables operator sections like `halve = flip(div)(2)`
- How to fold a slice of function pointers to build a processing pipeline

## OCaml Approach

In OCaml, every function is curried by default: `let add x y = x + y` already
has type `int -> int -> int`, so `add 5` is a valid partial application with
type `int -> int`. Operator sections like `( * ) 2` work because operators are
ordinary curried functions. `Fun.flip` swaps argument order to enable sections
like `halve = Fun.flip ( / ) 2`.

## Rust Approach

Rust functions take all their arguments at once; partial application is
expressed by returning a closure that captures the fixed arguments. Generic
higher-order functions that return closures (like `curry` and `flip`) need
`Box<dyn Fn>` because Rust cannot yet express `impl Fn(A) -> impl Fn(B) -> C`
as a stable return type. Function pipelines use `.fold()` over a slice of
`fn` pointers, mirroring `List.fold_left (fun acc f -> f acc)`.

## Key Differences

1. **Currying:** OCaml functions are curried by default; Rust closures capture
   arguments explicitly with `move |y| x + y`.
2. **Returning closures:** OCaml returns curried functions transparently; Rust
   needs `Box<dyn Fn>` to heap-allocate closures whose concrete type is unknown
   at the call site.
3. **Labeled arguments:** OCaml's `~scale` and `~shift` allow partial
   application in any order; Rust uses positional parameters and wraps calls
   in closures to fix specific arguments.
4. **Operator sections:** OCaml's `( * ) 2` is natural currying; Rust writes
   `|x| x * 2` or defines a named function.

## Exercises

1. Write a curried `zip_with: (A -> B -> C) -> [A] -> [B] -> [C]` and use partial application to create a vector addition function from a curried add.
2. Implement `uncurry` that converts a curried function `A -> B -> C` back into a two-argument function `(A, B) -> C`.
3. Use currying, partial application, and composition together to build a small expression evaluator: a curried `eval_binop` parameterized by operator and operands, composed with a parser that splits an infix expression string.
