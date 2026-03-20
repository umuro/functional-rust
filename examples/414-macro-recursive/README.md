📖 **[View on hightechmind.io →](https://hightechmind.io/rust/414-macro-recursive)**

---

# 414: Recursive Macro Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Some computations and code generation tasks have naturally recursive structure: counting elements, reversing lists, checking if a value equals any of N options. In functions, recursion is straightforward. In macros, recursion works by having one arm handle the base case (empty input) and another arm peel off one element and recursively invoke the macro on the remainder. This compile-time recursion is bounded by Rust's `macro_recursion_limit` (default 128) and computes entirely at compile time, producing efficient code with no runtime recursion.

Recursive macros are used in `matches!`, compile-time string concatenation, recursive DSL parsers, and any macro needing to process a variable-length input sequentially.

## Learning Outcomes

- Understand how recursive macros use base case and inductive step arms
- Learn the `@acc` (accumulator) pattern for building up results recursively
- See how `count!` uses `1 + count!(rest)` to count at compile time
- Understand the `reverse_list!(@acc [...] ...)` pattern for tail-recursive macros
- Learn about the `recursion_limit` attribute and when to increase it

## Rust Application

In `src/lib.rs`, `count!()` returns 0 for empty input (base case) and `1 + count!(tail)` for non-empty (inductive). `reverse_list!` uses the accumulator pattern with `@acc` internal arms — the public entry point initializes an empty accumulator, each step moves the head to the front of the accumulator, and the base case emits the array. `one_of!` recursively expands to `val == first || one_of!(val, rest)` until the single-element base case.

## OCaml Approach

OCaml PPX generates recursive code via recursive OCaml functions operating on AST nodes. A compile-time list-reversal PPX processes the actual list data during compilation. OCaml's `let[@unrolled] rec` attribute can unroll recursive functions. For true compile-time computation, OCaml uses its `module` system with recursive functors, though these are rarer than Rust's macro recursion.

## Key Differences

1. **Compile vs. runtime**: Rust macro recursion is compile-time token transformation; OCaml's equivalent would use module system recursion or PPX, both more heavyweight.
2. **Recursion limit**: Rust imposes a `recursion_limit` (default 128, adjustable with `#![recursion_limit = "512"]`); OCaml has no equivalent constraint.
3. **Accumulator pattern**: The `@acc` arm naming convention is a Rust macro idiom for internal state; OCaml uses standard accumulating function parameters.
4. **Error locality**: Rust macro recursion errors can produce deep expansion traces; OCaml recursive function errors appear at the function definition or call site.

## Exercises

1. **Compile-time sum**: Implement `const_sum!(1, 2, 3, 4, 5)` that expands to the literal sum `1 + 2 + 3 + 4 + 5` and verify with `const SUM: i32 = const_sum!(1, 2, 3)`.
2. **Unique macro**: Write `unique_types!(i32, f64, i32, String, f64)` using recursive macro expansion to deduplicate types and generate a tuple type containing only unique types from the list.
3. **Zip lists**: Implement `zip!(($a1, $a2, ...), ($b1, $b2, ...))` using recursive macro arms that peel one element from each list per step and accumulate `($a1, $b1)` pairs.
