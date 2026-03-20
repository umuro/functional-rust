📖 **[View on hightechmind.io →](https://hightechmind.io/rust/119-zero-cost-abs)**

---

# Zero-Cost Abstractions
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

High-level code often runs slower than hand-written low-level code in languages without zero-cost guarantees — intermediate allocations, dynamic dispatch, and interpreter overhead add up. Rust's zero-cost abstraction principle guarantees that iterator chains, closures, and newtype wrappers compile to identical machine code as their hand-written equivalents. This enables writing expressive, composable code without sacrificing performance — a core design goal of the language.

## Learning Outcomes

- Understand what "zero-cost abstraction" means concretely in terms of generated code
- See how iterator chains fuse into a single loop with no intermediate allocation
- Learn how closures are monomorphized and inlined, eliminating indirect calls
- Understand how newtypes provide compile-time safety with literally zero runtime overhead

## Rust Application

The code demonstrates three forms. First, `(0..n).filter(|x| x % 2 == 0).map(|x| x * x).sum()` compiles to a single loop — LLVM fuses the operations during optimization. Second, `make_polynomial` returns `impl Fn(f64) -> f64`; each call site gets a monomorphized, inlined copy. Third, `Meters` and `Seconds` newtypes are `repr(transparent)` — they vanish completely at runtime, leaving bare `f64` values, yet the type system prevents adding meters to seconds.

## OCaml Approach

OCaml does not guarantee zero-cost abstractions by default. `List.filter |> List.map` allocates two intermediate lists. OCaml closures carry a heap-allocated environment record. The `flambda` optimizer can eliminate some overhead in batch compilation, and `Seq` provides lazy iteration, but these are opt-in rather than guaranteed by the language design.

## Key Differences

1. **Iterator fusion**: Rust's iterator adaptors fuse at compile time (no intermediate collections); OCaml's eager list functions allocate at each step unless using `Seq`.
2. **Closure cost**: Rust closures are monomorphized with `impl Fn` — zero heap allocation, inlined call; OCaml closures are heap-allocated function records.
3. **Newtype overhead**: Rust newtypes are zero-overhead by construction (`#[repr(transparent)]`); OCaml has no equivalent — modules wrapping types may add indirection.
4. **Guarantee vs. best-effort**: Rust makes zero-cost a language-level guarantee for these patterns; OCaml relies on the optimizer (flambda) for similar effects.

## Exercises

1. Compare assembly output (via `cargo asm` or Godbolt) of `sum_even_squares` and `sum_even_squares_manual` — verify they produce identical loops.
2. Implement a `Celsius` newtype and a `to_fahrenheit` conversion; confirm the newtype has `size_of::<Celsius>() == size_of::<f64>()`.
3. Chain five iterator adaptors (filter, map, take, zip, fold) and verify in a benchmark that performance matches a manually unrolled loop.
