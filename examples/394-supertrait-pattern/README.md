📖 **[View on hightechmind.io →](https://hightechmind.io/rust/394-supertrait-pattern)**

---

# 394: Supertrait Pattern

## Problem Statement

Some traits only make sense when combined with others. A `Printable` type that can print itself both in debug and display forms requires both `Debug` and `Display`. Instead of requiring callers to write `T: Debug + Display + Printable` everywhere, supertraits express this: `trait Printable: Debug + Display` declares that any type implementing `Printable` must also implement `Debug` and `Display`. This reduces boilerplate at every call site and makes semantic groupings explicit in the trait system.

Supertraits appear throughout `std`: `Copy: Clone`, `Eq: PartialEq`, `Ord: Eq + PartialOrd`, and `Error: Debug + Display`. They are the mechanism for trait inheritance in Rust.

## Learning Outcomes

- Understand how supertraits encode trait inheritance in Rust's type system
- Learn that supertrait bounds apply everywhere the child trait is used
- See how default methods in supertraits can leverage the required supertrait bounds
- Understand how `User: Printable` implies `User: Debug + Display` transitively
- Learn to use supertraits to group commonly-combined trait requirements

## Rust Application

In `src/lib.rs`, `trait Printable: Debug + Display` declares both supertraits. The default `print` method uses both `{:?}` (Debug) and `{}` (Display) on `self` — possible because the supertrait bounds guarantee both are implemented. `trait Entity: Clone + Default` groups the persistence-related requirements. `User` derives `Debug, Clone, Default` and implements `Display` and `Printable`. The `Printable for User` impl is empty — the default method is inherited.

## OCaml Approach

OCaml achieves supertrait-like behavior through module signature inclusion: `module type PRINTABLE = sig include DEBUG; include DISPLAY; val print : t -> unit end`. An implementing module must provide all fields from all included signatures. First-class modules can be chained this way. OCaml's object system achieves inheritance differently via class inheritance with `inherit`, but this is less common in functional OCaml code.

## Key Differences

1. **Declaration**: Rust uses `: Supertrait` syntax in the trait definition; OCaml uses `include Sig` in module types or class `inherit` in objects.
2. **Default methods**: Rust supertrait default methods can call supertrait methods directly; OCaml functor default implementations use similar delegation.
3. **Transitivity**: Rust's bounds are transitive — a bound `T: Printable` implies `T: Debug + Display`; OCaml requires explicit inclusion in each signature.
4. **Derive interaction**: Rust's `derive` attributes automatically satisfy supertrait requirements (`#[derive(Debug, Clone)]`); OCaml uses `deriving` ppx extensions.

## Exercises

1. **Animal hierarchy**: Define `trait Alive` with `fn breathe() -> String`, then `trait Animal: Alive` with `fn speak() -> String`, then `trait Pet: Animal` with a default `fn cuddle() -> String`. Implement for `Dog` and `Cat`.
2. **Sortable group**: Define `trait Sortable: Ord + Clone` with a default method `fn sort_copy(items: &[Self]) -> Vec<Self>` that returns a sorted copy. Implement it for `i32` and a custom `Score` newtype.
3. **Display-requiring trait**: Create `trait Report: Display` with a default `fn print_report(&self)` and `fn to_file(&self, path: &str) -> std::io::Result<()>` that writes `self.to_string()` to disk. Implement for a `SalesReport` struct.
