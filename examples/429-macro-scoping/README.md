📖 **[View on hightechmind.io →](https://hightechmind.io/rust/429-macro-scoping)**

---

# 429: Macro Scoping and Visibility
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Macros have complex scoping rules in Rust that differ from both functions and types. A `macro_rules!` without `#[macro_export]` is only visible within the file it's defined in and below in the same module tree. `#[macro_export]` exports to the crate root, making it accessible as `crate::my_macro!`. The 2018 edition introduced module-path importing (`use crate::my_macro`). Understanding these rules is essential for structuring crates with macros and for importing macros from dependencies.

Macro scoping rules explain why `use std::collections::HashMap` doesn't export macros, why `#[macro_use] extern crate` was the old way to import macros, and how `pub use crate_name::macro_name` re-exports work.

## Learning Outcomes

- Understand `#[macro_export]` and how it places macros at crate root
- Learn that macro visibility is lexical — macros defined later in a file are visible earlier (unlike functions)
- See how module-local macros (without `#[macro_export]`) are restricted to their module
- Understand the `use crate::macro_name` path for importing macros in Rust 2018+
- Learn how `pub use dependency::macro_name` re-exports macros from dependencies

## Rust Application

In `src/lib.rs`, `public_macro!` has `#[macro_export]` making it available crate-wide as `crate::public_macro!`. `private_macro!` has no export, restricting it to this file. The `inner` module defines `local_macro!` visible only within the module. `use_private()` calls the private macro, demonstrating it works within the crate. `use_public()` demonstrates the exported macro.

## OCaml Approach

OCaml modules naturally scope functions. A `module Private = struct let helper = ... end` creates a private scope. `module type S = sig val public_fn : unit -> string end` restricts what's exported. OCaml doesn't distinguish macros from functions in scoping — PPX extensions are build-time only and don't have runtime scope. Library users access PPX features through dune configuration, not `use` statements.

## Key Differences

1. **Path-based import**: Rust 2018 macros use `use crate::macro_name`; older Rust required `#[macro_use] extern crate name`.
2. **Crate root export**: `#[macro_export]` always places macros at the crate root regardless of module nesting; OCaml exports are always at the module they're defined in.
3. **No runtime scope**: Macros don't exist at runtime; OCaml module functions do. The scoping rules serve compile-time resolution only.
4. **Re-export**: Rust re-exports macros with `pub use crate_dep::macro_name`; OCaml re-exports with `include Module` or explicit `let fn = Mod.fn`.

## Exercises

1. **Module-gated macro**: Define a macro inside a `mod internal { macro_rules! my_macro { ... } }` and verify it's not accessible outside the module. Then add `#[macro_export]` and verify it becomes accessible via `crate::my_macro`.
2. **Re-export chain**: Create a crate A with `#[macro_export] macro_rules! helper!`. In crate B, `pub use crate_a::helper`. In crate C using crate B, verify `use crate_b::helper` makes the macro available.
3. **Conditional export**: Use `#[cfg(feature = "macros")] #[macro_export]` to make a macro only available when a feature is enabled. Write a doc comment explaining the feature flag to users.
