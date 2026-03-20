# Example 1107: Currying, Partial Application, and Sections

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** https://cs3110.github.io/textbook/chapters/hop/higher_order.html

## Problem Statement

Demonstrate how OCaml's automatic currying and partial application translate to
Rust, where functions take all arguments at once but closures enable the same
specialisation patterns.

## Learning Outcomes

- How `move` closures in Rust replicate OCaml's automatic partial application
- Why Rust needs `Box<dyn Fn>` to implement generic `curry`/`uncurry` converters
- How named functions serve as operator sections when placed in `&[fn]` slices
- How two-parameter closures replace OCaml's labeled-argument partial application

## OCaml Approach

Every OCaml function is curried by default: `let add x y = x + y` has type
`int -> int -> int`, meaning applying it to one argument returns a new function.
`let add5 = add 5` literally creates a specialised adder at zero cost. Operator
sections like `( * ) 2` and `Fun.flip ( / ) 2` use the same mechanism. Labeled
arguments (`~scale ~shift`) allow partial application in any order.

## Rust Approach

Rust functions take all arguments simultaneously; partial application is written
explicitly as a `move` closure. `fn partial_add(x: i32) -> impl Fn(i32) -> i32`
captures `x` in a closure and returns it. Generic `curry`/`uncurry` require
`Box<dyn Fn>` for the inner closure because unnameable closure types cannot be
expressed as `impl Fn` inside another closure return. Operator sections become
named `fn` items that match `fn(i32) -> i32` and can be collected into a slice
for pipeline processing with `fold`.

## Key Differences

1. **Default currying:** OCaml functions are automatically curried; Rust requires an explicit closure wrapper returning another closure.
2. **Partial application syntax:** OCaml: `add 5`; Rust: `move |y| add(5, y)` or `partial_add(5)`.
3. **Generic higher-order converters:** OCaml's `curry`/`uncurry` are naturally polymorphic; Rust requires `Box<dyn Fn>` for the inner returned closure due to unnameable closure types.
4. **Operator sections:** OCaml uses `( * ) 2` or `Fun.flip ( / ) 2`; Rust uses named `fn` items or inline closures like `|x| x * 2`.

## Exercises

1. Use partial application to create a family of multiplier functions (`double`, `triple`, `times_n`) from a single curried `multiply: i32 -> i32 -> i32`.
2. Implement `section_left` and `section_right` combinators that fix the left or right argument of a binary function, and use them to adapt `str::contains` into prefix/suffix checkers.
3. Write a pipeline that reads a list of raw log strings, uses partially applied predicates to filter by severity level, partially applied formatters to normalize each line, and outputs the result — with each step expressed as a point-free composition.
