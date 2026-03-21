📖 **[View on hightechmind.io →](https://hightechmind.io/rust/053-pipeline-operator)**

---

# Example 053: Pipeline Operator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

OCaml's `|>` operator threads a value through a sequence of functions left-to-right:
`5 |> double |> add1` reads as "take 5, double it, then add 1" — matching the order you think about the transformations.

## Learning Outcomes

- Understand why left-to-right pipelines improve readability over nested calls
- Implement a generic `Pipe` trait that adds `.pipe(f)` to any value
- Write a `pipe!` macro that expands a chain into sequential application
- Compare Rust's three styles: nested calls, trait method chaining, macro syntax

## OCaml Approach

Defines `|>` as `let ( |> ) x f = f x`. This is just reverse function application — `x |> f` is `f x`. Modern OCaml includes `|>` in the standard library. The operator composes naturally because it's left-associative: `x |> f |> g` parses as `(x |> f) |> g` which is `g(f(x))`.

## Rust Approaches

1. **Nested calls**: `add1(double(5))` — identical semantics, reads right-to-left
2. **Trait pipe**: `5.pipe(double).pipe(add1)` — left-to-right, ergonomic, generic over all types
3. **Macro**: `pipe!(5 => double, add1)` — explicit visual syntax for pipelines

## Key Differences

1. **Built-in vs explicit**: Rust has no `|>` operator; the trait and macro replicate the pattern
2. **Method vs operator**: `.pipe(f)` uses method syntax — familiar to Rust users, no operator overloading needed
3. **Macro flexibility**: `pipe!` makes the pipeline structure visually explicit
4. **Zero cost**: Both the trait default method and macro expand at compile time with no runtime overhead
5. **Ownership**: Each `pipe` step consumes and produces — ownership flows through the chain naturally

## Exercises

1. Implement a `pipe` function that takes a value and a single-argument closure, applies the closure, and returns the result — then chain three `pipe` calls to transform a string.
2. Write a `pipe_debug` wrapper that logs the intermediate value between two pipeline stages to stderr, keeping the same type signature as `pipe`.
3. Model a data-transformation pipeline using `pipe`: parse a CSV row string → split into fields → trim each field → parse the third field as `f64` → multiply by a tax rate, returning a `Result<f64, String>`.
