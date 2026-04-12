📖 **[View on hightechmind.io →](https://hightechmind.io/rust/422-derive-macro-concept)**

---

# 422: Derive Macro Concepts
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Many trait implementations are entirely mechanical: `Debug` for a struct just prints each field name and value, `Clone` copies each field, `PartialEq` compares each field. Writing these by hand for every type is tedious, error-prone (especially when fields are added later), and distracts from the actual logic. `#[derive(Debug, Clone, PartialEq)]` instructs the compiler to generate these mechanical implementations automatically based on the type's structure. Understanding what derive macros generate is essential for debugging unexpected behavior.

Derive macros are the most common form of code generation in Rust: `serde::Deserialize`, `Debug`, `Clone`, `PartialEq`, `Hash`, `Default` — virtually every struct uses them.

## Learning Outcomes

- Understand what code `#[derive(Debug)]`, `#[derive(Clone)]`, and `#[derive(PartialEq)]` generate
- Learn how derive macros inspect the struct/enum structure to generate field-by-field code
- See the equivalence between `ManualDebug`'s hand-written impl and the derived version
- Understand when derived implementations are insufficient (custom comparison, non-standard display)
- Learn the requirements: all fields must implement the derived trait

## Rust Application

In `src/lib.rs`, `Point` derives `Debug, Clone, Copy, PartialEq, Eq, Hash, Default` — all seven traits from a single line. `ManualDebug` shows what `#[derive(Debug)]` actually generates: `f.debug_struct("ManualDebug").field("value", &self.value).finish()`. The manual `Clone` for `ManualDebug` shows `ManualDebug { value: self.value }` — identical to what the derive would produce. Tests verify both produce the same output.

## OCaml Approach

OCaml uses `ppx_deriving` or `ppx_compare`/`ppx_hash` from Jane Street for equivalent code generation. `[@@deriving show, eq, ord]` after a type definition generates `show`, `equal`, and `compare` functions. The `show` ppx generates `pp` functions for `Format.formatter`. Unlike Rust's integrated derive system, OCaml's derivers are separate ppx plugins that must be listed as build dependencies.

## Key Differences

1. **Integrated vs. plugins**: Rust's `Debug`, `Clone`, `PartialEq`, `Hash` are built into `rustc`; OCaml requires external ppx plugins in `dune` configuration.
2. **Trait vs. function**: Rust derives implement traits (uniform interface); OCaml ppx generates standalone functions (`show`, `equal`, `compare`).
3. **Field traversal**: Both generate field-by-field code; Rust's version uses the trait interface (`Debug::fmt` per field), OCaml's uses pattern matching.
4. **Error when field lacks trait**: Rust compile error says "field `x` of type `T` doesn't implement `Debug`"; OCaml ppx gives similar errors.

## Exercises

1. **Expand and study**: Add `#[allow(unused)]` and a new field `pub label: Option<String>` to `Point`. Predict what the derived `Debug` output will look like, then verify with `format!("{:?}", p)`. Add a field of a type that doesn't implement `Debug` and study the compile error.
2. **Custom debug**: Implement a `struct Password(String)` where the derived `Debug` would expose the secret. Write a custom `Debug` that outputs `Password("***")` regardless of the actual value.
3. **Partial derive**: Implement a struct where `PartialEq` should compare only some fields (e.g., a `User` where equality is by `id` only). Write the manual implementation and add a comment explaining why the derived version would be wrong.
