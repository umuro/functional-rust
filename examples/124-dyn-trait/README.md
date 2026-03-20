📖 **[View on hightechmind.io →](https://hightechmind.io/rust/124-dyn-trait)**

---

# dyn Trait — Dynamic Dispatch
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Sometimes the concrete type implementing a trait is unknown at compile time — a plugin system, a heterogeneous collection of shapes, or a UI widget tree. Static dispatch (`impl Trait` / generics) requires knowing the type at compile time and produces one copy of the code per type. Dynamic dispatch (`dyn Trait`) uses a vtable to resolve method calls at runtime, enabling heterogeneous collections and open extension without recompilation. Understanding when to choose each is a core Rust design skill.

## Learning Outcomes

- Understand the three polymorphism strategies: `dyn Trait`, `impl Trait`/generics, and enum dispatch
- Learn what a vtable is and why `dyn Trait` costs one pointer indirection per method call
- See when each strategy is appropriate: open extension vs. closed set vs. performance-critical paths
- Recognize the `dyn`-compatibility rules that restrict which traits can be used with `dyn`

## Rust Application

The code implements `Shape` in three styles. The `dyn Trait` version stores `Vec<Box<dyn Shape>>` — each `Box` is a fat pointer (data pointer + vtable pointer). The generic version requires a uniform element type. The enum version uses `ShapeEnum` with explicit `match` — the compiler knows all variants, generates no vtable, and inlines every branch. For UI frameworks and plugin systems, `dyn Trait` is the right choice; for performance-critical geometry, enum dispatch wins.

## OCaml Approach

OCaml's object system provides true dynamic dispatch via virtual method tables. OCaml's idiomatic approach for sum types uses variants (`type shape = Circle of float | Rect of float * float`) with pattern matching — equivalent to Rust's enum dispatch, and the most common approach in functional OCaml code. First-class modules provide a form of existential dispatch similar to `Box<dyn Trait>`.

## Key Differences

1. **Cost**: Rust `dyn Trait` adds one indirection per call plus heap allocation; OCaml objects similarly add an indirection; OCaml variants use direct dispatch via `match`.
2. **Openness**: `dyn Trait` allows external crates to add new types; enum dispatch is closed; OCaml polymorphic variants allow open extension similarly to `dyn Trait`.
3. **Trait object restrictions**: Rust requires traits to be "dyn-compatible" (no generic methods, no `Self` in non-receiver position); OCaml objects have no such restriction.
4. **Fat pointers**: Rust's `dyn Trait` reference is two machine words (data + vtable); OCaml object references are one word (the object header points to the method table).

## Exercises

1. Add a `Triangle` variant to the enum-dispatch version and verify it compiles without touching the `dyn Trait` version.
2. Write a `largest_area(shapes: &[Box<dyn Shape>]) -> f64` function using the dynamic dispatch version.
3. Benchmark the three approaches (dyn, generic, enum) with 10,000 shape evaluations and compare throughput.
