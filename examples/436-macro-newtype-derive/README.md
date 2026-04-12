📖 **[View on hightechmind.io →](https://hightechmind.io/rust/436-macro-newtype-derive)**

---

# 436: Newtype Derive Patterns
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Newtypes provide type safety but impose an implementation burden: you must re-derive or re-implement every trait the inner type has. `Email(String)` should support `Display`, `FromStr`, `Deref<Target=str>`, `AsRef<str>`, and more — all of which `String` already provides. The `derive_more` crate and custom macros generate these delegating implementations automatically. Without them, every newtype requires dozens of boilerplate impl blocks that simply forward to the inner type.

Newtype derive patterns appear in domain modeling (`UserId`, `OrderId`, `Email`), unit systems (`Meters`, `Kilograms`), and any codebase that uses newtypes extensively for type safety.

## Learning Outcomes

- Understand the boilerplate burden of newtypes and how derive macros reduce it
- Learn which traits are derivable and which require custom `impl` for newtypes
- See how `derive_more::Display`, `derive_more::From`, `derive_more::Deref` work
- Understand validated newtypes (constructor returns `Result`/`Option`) vs. transparent newtypes
- Learn how `#[derive(PartialOrd, Ord)]` on newtypes provides comparison via the inner type

## Rust Application

In `src/lib.rs`, `Email(String)` uses standard derives for `Debug, Clone, PartialEq, Eq, Hash`. The constructor `Email::new` validates the string, returning `Result<Self, &'static str>`. `as_str()` provides access to the inner string. `PositiveInt(u32)` derives `PartialOrd, Ord` — comparison delegates to `u32` automatically. The `derive_more` crate (not used here) would add `Display`, `From<String>`, `Deref`, and `Into<String>` with single attributes.

## OCaml Approach

OCaml's newtype equivalent is an abstract type in a module: `module Email : sig type t; val of_string : string -> t option; val to_string : t -> string end`. The module hides the `string` inside, requiring explicit conversion. OCaml's `ppx_deriving` can generate comparison and hash functions. The module system enforces the type boundary more strictly than Rust's newtypes, which are just single-field structs.

## Key Differences

1. **Derive vs. module**: Rust newtypes derive traits selectively; OCaml modules hide the type and provide only explicitly exposed functions.
2. **Deref transparency**: Rust newtypes can implement `Deref<Target=Inner>` for transparency; OCaml provides only the functions defined in the module signature.
3. **Field access**: Rust newtypes with `pub` fields expose the inner value directly; OCaml's abstract types require accessor functions.
4. **Derive more**: The `derive_more` crate reduces newtype boilerplate; OCaml's `ppx_deriving` serves the same role.

## Exercises

1. **derive_more usage**: Add `derive_more` as a dependency and rewrite `Email` using `#[derive(Display, From, Deref, Into)]`. Remove the manual `as_str` method and verify the same functionality works through `Deref`.
2. **Newtype stack**: Create a stack of newtypes: `RawId(u64)`, `UserId(RawId)`, `AdminId(UserId)`. Implement `From<u64> for UserId` via `RawId`. Show that `AdminId` can't be accidentally used where `UserId` is expected.
3. **Comprehensive newtype**: Implement `Percentage(f64)` with validation (0.0..=100.0), `Display` showing `42.5%`, `Add`/`Sub` operations that clamp results to valid range, and `From<f64>` with saturation.
