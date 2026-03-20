📖 **[View on hightechmind.io →](https://hightechmind.io/rust/432-macro-enum-dispatch)**

---

# 432: Macro Enum Dispatch
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Enums with many variants often need repetitive match arms for dispatch methods. A `Command` enum with 20 variants requires a 20-arm `match` for each dispatch method. When variants map uniformly to trait implementations, macros can generate all the match arms automatically. This is the `enum_dispatch` crate's core idea: eliminate the boilerplate of matching each variant and delegating to the inner type. The `dispatch_enum!` macro generates both the enum and its dispatch methods from a compact declaration.

Enum dispatch macros appear in event handling systems, command patterns, codec implementations, and any state machine with many uniform variant behaviors.

## Learning Outcomes

- Understand how `dispatch_enum!` generates both enum variants and dispatch methods
- Learn how `stringify!($variant)` in match arms provides zero-cost variant names
- See how macros reduce the O(n) boilerplate of adding dispatch methods to enums
- Understand when enum dispatch (closed set, fast) is better than `dyn Trait` (open set, flexible)
- Learn how the `matches!` macro simplifies variant-checking predicates

## Rust Application

In `src/lib.rs`, `dispatch_enum!(Action { Start, Stop, Pause, Resume })` generates the `Action` enum and an `impl Action` with a `name()` method. The `stringify!($variant)` in the match arm converts each variant name to a `&'static str` at compile time. The `Command` enum demonstrates manual dispatch in `execute()`. The `matches!(self, DoorState::Closed)` idiom for state predicates shows the `matches!` standard library macro for single-variant checking.

## OCaml Approach

OCaml handles enum dispatch through algebraic types and pattern matching. A `type command = Print of string | Add of int * int | Exit` with `let execute = function Print s -> ... | Add (a,b) -> ... | Exit -> ...` is idiomatic. OCaml's exhaustiveness checking ensures all variants are handled. Adding a new variant breaks existing match expressions — a feature (forces updates) or a bug (breaks compatibility).

## Key Differences

1. **Exhaustiveness**: Both Rust and OCaml require exhaustive match arms; adding a variant to a `dispatch_enum!`-generated enum requires updating all dispatch methods.
2. **Name generation**: Rust's `stringify!($variant)` gives the variant name as a string automatically; OCaml needs explicit string mappings or deriving.
3. **Macro vs. pattern**: OCaml's pattern matching is a language feature; Rust's dispatch is a macro-generated impl.
4. **Closed vs. open**: Both enum dispatch approaches are closed sets; `dyn Trait` in Rust and first-class modules in OCaml handle open sets.

## Exercises

1. **Codec dispatch**: Use `dispatch_enum!` to generate a `Format { Json, Csv, Toml, Binary }` enum with a `content_type() -> &'static str` method returning the MIME type for each format.
2. **Error category**: Create `ErrorCategory { Network, Io, Parse, Permission, Timeout }` with dispatch methods `is_retryable() -> bool` and `log_level() -> &'static str`. Implement these with different behavior per variant.
3. **Dispatch with data**: Extend the pattern to handle variants with payloads: `dispatch_data!( Message { Text(String) => fn len -> String::len(data), Binary(Vec<u8>) => fn len -> data.len() } )`.
