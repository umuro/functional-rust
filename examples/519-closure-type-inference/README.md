📖 **[View on hightechmind.io →](https://hightechmind.io/rust/519-closure-type-inference)**

---

# Closure Type Inference
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Type inference for closures is a quality-of-life feature that distinguishes modern functional-leaning languages from older systems languages. In Rust, the compiler infers closure parameter and return types from the context in which the closure is first used — similar to how Hindley-Milner inference works in ML-family languages. However, unlike full HM inference, Rust locks in a closure's type at its first use site and rejects subsequent calls with different types. Understanding these rules helps avoid cryptic type errors when composing iterators and higher-order functions.

## Learning Outcomes

- How Rust infers closure parameter types from their first use
- Why closures have unique, anonymous types that must be captured via generics or boxing
- When explicit type annotations are required and when they are redundant
- Why the same closure cannot be called with two different argument types
- How `apply<F, T, U>(f: F, x: T) -> U` generalizes over closure types

## Rust Application

`apply<F, T, U>(f: F, x: T) -> U where F: Fn(T) -> U` is a fully generic HOF — the compiler infers `F`, `T`, and `U` at each call site. `inference_demo` shows `let double = |x| x * 2` — once called with `5i32`, the compiler locks `x: i32`. `consistent_types` shows the error that would arise if `process` were called with both `i32` and `f64`. `needs_annotation<T: Add>` requires explicit `|a: T, b: T|` because `T` is generic and cannot be inferred from expression shape alone.

Key patterns:
- Single-type-per-closure rule — once inferred, the type is fixed
- `let f = |x| x + 1; let _: i32 = f(5);` — inference from usage
- Generic bounds `T: std::ops::Add<Output = T> + Copy` enabling polymorphic closures

## OCaml Approach

OCaml uses the Hindley-Milner algorithm with full let-polymorphism. Closures infer types independently at each use, and a value-restriction applies to prevent unsound generalization of mutable values. Unlike Rust, OCaml can generalize `let f = fun x -> x` to `'a -> 'a` — a genuinely polymorphic identity closure.

```ocaml
let apply f x = f x       (* 'a -> 'b inferred *)
let double = fun x -> x * 2  (* int -> int inferred from * *)
```

## Key Differences

1. **Polymorphic closures**: OCaml can produce a polymorphic closure `'a -> 'a`; Rust closures have a single unique type — true polymorphism requires a trait bound on a generic parameter.
2. **Lock-in rule**: Rust fixes the closure type at first use, making later calls with different types a hard error; OCaml unifies types across uses in the same scope.
3. **Type annotation frequency**: Rust closures rarely need annotations when used immediately with concrete types; OCaml type annotations are often omitted entirely due to HM inference.
4. **Error messages**: Rust type errors for closures point to the conflicting use site; OCaml errors often point to the unification failure between two constraints, which can be further away.

## Exercises

1. **Double then square**: Write `let f = |x| x * 2` and compose it with `let g = |x| x * x` using `apply`, verifying that all types are inferred with no annotations.
2. **Generic apply2**: Implement `apply2<F, A, B, C>(f: F, a: A, b: B) -> C where F: Fn(A, B) -> C` and use it with both a named function and a closure.
3. **Annotation exploration**: Try giving `let f = |x| x + 1` an explicit return type `-> i64` and verify that calling it with an `i32` literal causes a type error, explaining why.
