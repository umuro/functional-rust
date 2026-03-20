[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 078 — Where Clauses

## Problem Statement

Express complex trait bounds on generic functions and types using Rust's `where` clause syntax. Implement `print_if_equal`, `zip_with`, `sum_items`, `dot_product`, and `display_collection` — each requiring multiple or compound constraints — and compare with OCaml's module functor approach to constraining polymorphic code.

## Learning Outcomes

- Write `where` clauses to separate complex bounds from the function signature
- Combine multiple trait bounds on a single type parameter (`T: Display + PartialEq`)
- Apply bounds to associated types of iterator (`I::Item: Add + Default`)
- Use `IntoIterator` with `I::Item: Display` for generic collection printing
- Understand when `where` improves readability over inline bound syntax
- Map OCaml functor module signatures to Rust `where` constraints

## Rust Application

`where` clauses move trait bounds after the function body's opening `{`, improving readability when bounds grow complex. `sum_items` demonstrates constraining an associated type — `I::Item: Add<Output = I::Item> + Default` — which is only expressible in a `where` clause. `dot_product` requires `T: Add + Mul + Default + Copy`, combining arithmetic and memory semantics. `display_collection` accepts any `IntoIterator` whose items implement `Display`, producing a formatted string. Each function is fully generic yet statically dispatched — no runtime overhead.

## OCaml Approach

OCaml expresses constraints via module signatures. A functor `MathOps(S : SUMMABLE)` requires the input module to provide `zero`, `add`, and `to_string`. More complex constraints combine signatures: `module type RING = sig include SUMMABLE include MULTIPLIABLE end`. Concrete modules like `IntSum` and `FloatSum` are produced by applying the functor to struct-style anonymous modules. The type system ensures constraints are satisfied at functor application, not at call sites — equivalent to Rust's monomorphization.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Syntax | `where T: Trait1 + Trait2` | `module type SIG = sig … end` |
| Scope | Per function/impl block | Module-level functor |
| Associated type bounds | `I::Item: Trait` in `where` | `with type item = t` refinement |
| Constraint composition | `+` on trait bounds | `include` in module types |
| Monomorphization | Yes, at compile time | Functors instantiated at application |
| Runtime cost | None | None |

`where` syntax is purely cosmetic in most cases — it does not change semantics versus inline bounds. The exception is associated type constraints, which *require* `where`. OCaml functors produce named modules, making them first-class values; Rust generic functions are erased into monomorphized copies.

## Exercises

1. Add a `min_max` function with signature `fn min_max<T>(slice: &[T]) -> Option<(&T, &T)>` using a `where` clause requiring `T: PartialOrd`.
2. Write `map_collect<I, F, B>(iter: I, f: F) -> Vec<B>` using `where` to express the iterator and closure bounds separately.
3. Implement a `Printable` trait with a `print` method, then use `where T: Printable + Clone` in a function that clones and prints each element of a slice.
4. Create a functor in OCaml called `Sorted(C : COMPARABLE)` and implement a sorted insertion function. Compare the constraint surface area with the Rust `where` equivalent.
5. Explore the difference between `fn f<T: A + B>()` and `fn f<T>() where T: A + B`. Verify both compile identically with `cargo expand` or by checking the generated MIR.
