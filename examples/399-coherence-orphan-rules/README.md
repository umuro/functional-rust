📖 **[View on hightechmind.io →](https://hightechmind.io/rust/399-coherence-orphan-rules)**

---

# 399: Coherence and Orphan Rules

## Problem Statement

In a large ecosystem with thousands of crates, two independent crates could both implement `Display` for `Vec<i32>`, creating an ambiguous implementation conflict. Rust's orphan rule prevents this: you can only implement a trait for a type if either the trait or the type is defined in your current crate. This coherence guarantee ensures that every `(trait, type)` pair has exactly one implementation, making code predictable regardless of which crates are combined. The newtype pattern is the standard workaround when you need to implement a foreign trait for a foreign type.

Coherence is fundamental to Rust's trait system reliability and is why `serde`, `std`, and other foundational crates can co-exist without ambiguity.

## Learning Outcomes

- Understand why the orphan rule exists (coherence guarantee, preventing ecosystem conflicts)
- Learn which combinations are allowed: (local trait, foreign type), (foreign trait, local type), but not (foreign trait, foreign type)
- See the newtype pattern as the standard workaround for the orphan rule
- Understand how `impl Describable for i32` works because `Describable` is local
- Learn how blanket impls interact with coherence

## Rust Application

In `src/lib.rs`, `Wrapper<T>` is a local newtype enabling `impl Display for Wrapper<Vec<i32>>` — the trait is foreign (`Display`) but the type `Wrapper<...>` is local. `Describable` is a local trait, so it can be implemented for any type including foreign `i32`, `String`, and generic `Vec<T>`. Attempting `impl Display for Vec<i32>` (foreign trait + foreign type) would fail. The blanket `impl<T: Describable> Describable for Vec<T>` is allowed because `Describable` is local.

## OCaml Approach

OCaml has no orphan rule. Any module can provide any function for any type. This flexibility enables convenience but can create conflicts — if two modules both `open`'d provide `to_string : int -> string`, the last one shadows the other. OCaml resolves conflicts through module shadowing and explicit qualification (`Module.function`), accepting ambiguity at the cost of explicit resolution.

## Key Differences

1. **Coherence guarantee**: Rust has global coherence — one impl per `(trait, type)` pair; OCaml relies on lexical scoping and `open` ordering.
2. **Conflict resolution**: Rust makes impl conflicts a compile error; OCaml silently shadows with the most recently opened module.
3. **Orphan rule**: Rust enforces it; OCaml has no equivalent — any function can be defined for any type anywhere.
4. **Newtype necessity**: Rust requires newtypes to work around orphan restrictions; OCaml can directly add functions to any type's module.

## Exercises

1. **Orphan rule exploration**: Attempt to implement `std::fmt::Display` for `std::collections::HashMap<String, i32>`. Observe the compiler error. Then create `DisplayMap(HashMap<String, i32>)` newtype and implement `Display` on it instead.
2. **Blanket impl limits**: Implement `trait Summary` with a blanket `impl<T: Display> Summary for T`. Then try to add a second blanket `impl<T: Debug> Summary for T`. Observe the coherence error and explain why it occurs.
3. **Crate simulation**: Create a module `lib_a` with `trait Printable {}` and a module `lib_b` with `struct Point { x: f32, y: f32 }`. In your main module, implement `Printable for Point` and explain why this satisfies the orphan rule.
