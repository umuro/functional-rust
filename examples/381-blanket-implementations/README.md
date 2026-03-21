📖 **[View on hightechmind.io →](https://hightechmind.io/rust/381-blanket-implementations)**

---

# 381: Blanket Implementations
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When a trait's functionality can be derived entirely from another trait, duplicating the implementation for every concrete type is tedious and error-prone. Blanket implementations solve this by implementing a trait for all types satisfying a bound: `impl<T: Bound> MyTrait for T`. This is how the standard library implements `ToString` for everything that implements `Display`, and how `From` blanket-implies `Into`. The technique enables powerful composable abstractions without requiring each type to opt in explicitly.

Blanket impls are foundational to Rust's trait system and appear throughout `std`: `From`/`Into` conversions, `Iterator` adapters, `AsRef`/`AsMut`, and the `serde` serialization framework's generic implementations.

## Learning Outcomes

- Understand how blanket implementations reduce boilerplate across entire type families
- Learn the coherence rules that prevent conflicting blanket implementations
- See how `impl<T: Display> Summary for T` applies to all `Display` types simultaneously
- Understand the orphan rule constraint: either the trait or the type must be local to your crate
- Learn when blanket impls are appropriate vs. when they create conflicts

## Rust Application

In `src/lib.rs`, `impl<T: fmt::Display> Summary for T` implements `Summary` for every type that already implements `Display` — integers, floats, strings, and any user type with `Display`. Similarly `impl<T: fmt::Display> DoubleString for T` and `impl<T: fmt::Debug> IntoJson for T` demonstrate that different bounds enable different blanket impls. The compiler resolves these at the call site: `42i32.summarize()` triggers the blanket impl, not a specialized impl.

## OCaml Approach

OCaml achieves similar effects through module functors: `module MakeSummary (M : Display) = struct let summarize x = "Summary: " ^ M.to_string x end`. This requires explicit functor application (`module IntSummary = MakeSummary(Int)`), unlike Rust's automatic blanket resolution. OCaml's type classes (via first-class modules or modular implicits) provide similar power but require more explicit wiring.

## Key Differences

1. **Automatic vs. explicit**: Rust's blanket impls apply automatically when bounds are satisfied; OCaml functors must be applied explicitly per type.
2. **Coherence**: Rust enforces that no two blanket impls can conflict (overlapping implementations are rejected); OCaml has no global coherence requirement.
3. **Orphan rule**: Rust prevents implementing a foreign trait for a foreign type; OCaml modules have no orphan restrictions since implementations are local to modules.
4. **Discoverability**: Rust's blanket impls are visible in documentation (rustdoc shows "implementors"); OCaml functor applications are invisible unless explicitly named.

## Exercises

1. **Printable blanket**: Define a `Printable` trait with a `print(&self)` method and implement it as a blanket impl for all `Display` types. Verify it works for `i32`, `f64`, `String`, and a custom struct.
2. **Validate blanket**: Define a `Validate` trait with `fn validate(&self) -> Result<(), String>`. Create a `NonEmpty` marker trait, then write a blanket impl of `Validate` for all types implementing `NonEmpty + Display`.
3. **Conflict demonstration**: Write two blanket impls that would conflict (e.g., one for `T: Display` and one for `T: Debug`) and document the compiler error message that results, explaining why coherence requires one impl to win.
