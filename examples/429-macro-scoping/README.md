📖 **[View on hightechmind.io →](https://hightechmind.io/rust/429-macro-scoping)**

---

# 429: Macro Scoping and #[macro_export]

**Difficulty:** 3  **Level:** Advanced

`macro_rules!` macros follow textual scoping within a file, but `#[macro_export]` hoists them to the crate root — understanding this distinction prevents confusing "macro not found" errors.

## The Problem This Solves

Macros in Rust have a scoping model that surprises everyone the first time. Unlike functions and types, which are scoped to modules, `macro_rules!` macros are visible only from the point of definition downward in the source file. Define a macro at the bottom of a file and try to use it at the top — you get a "cannot find macro" error. Move it up and it works. This ordering dependency feels arbitrary and trips up developers used to module-based scoping.

The second surprise is cross-module use. A macro defined in `mod inner` is not automatically visible in the parent module. And when you want to export a macro for use by other crates, `pub use` doesn't work — you need `#[macro_export]`, which bypasses the module tree entirely and puts the macro at the crate root.

Understanding these rules is essential for library authors. A library that exports macros must either use `#[macro_export]` (the modern way) or the old `#[macro_use]` extern crate pattern.

## The Intuition

Think of `macro_rules!` as a textual find-replace rule that the compiler registers as it reads the file top to bottom. Once registered, it's available for the rest of the file. If a macro is in a submodule, it's registered in that module's namespace only — unreachable from outside unless exported.

`#[macro_export]` teleports the macro to the crate's top-level namespace, as if you had defined it in `lib.rs`. Other crates can then use it with `use your_crate::your_macro!` (Rust 2018+) or the older `#[macro_use] extern crate your_crate`.

`$crate::` inside a macro is the counterpart: it refers to the defining crate regardless of where the macro is used, ensuring that helper functions the macro calls are correctly resolved.

## How It Works in Rust

```rust
// Local macro — visible only from here downward in this file/module
macro_rules! add {
    ($a:expr, $b:expr) => { $a + $b };
}

// Exported macro — hoisted to crate root, importable from other crates
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tolerance:expr) => {{
        let diff = ($a - $b).abs();
        assert!(diff < $tolerance);
    }};
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-9f64);  // calls itself — same crate
    };
}

// $crate:: resolves to THIS crate even when macro is used from another crate
#[macro_export]
macro_rules! my_debug {
    ($val:expr) => {
        // Without $crate::, 'log_impl' would be looked up in the CALLER's crate
        $crate::log_impl($val)
    };
}

mod inner {
    // Visible only inside 'inner' module:
    macro_rules! inner_only { ($x:expr) => { $x + 1 }; }

    // #[macro_export] hoists even from a nested module to crate root:
    #[macro_export]
    macro_rules! inner_exported { ($x:expr) => { $x * 2 }; }

    pub fn compute(x: i32) -> i32 {
        inner_only!(x) // fine — same module
    }
}

// inner_only!(5); // ERROR: not visible outside 'inner'
inner_exported!(5); // fine — hoisted to crate root
```

## What This Unlocks

- **Library macros** — export utility macros (`assert_approx_eq!`, `bail!`, `ensure!`) for users of your crate with `#[macro_export]`.
- **Cross-crate safety** — use `$crate::` to reference your crate's items inside exported macros, preventing broken lookups when users import the macro.
- **Structured macro organisation** — keep macros in dedicated files, use `#[macro_use] mod macros;` (with `#[macro_export]` inside) to make them available crate-wide.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Macro visibility | `ppx` transformations apply globally during compilation | `macro_rules!` textually scoped; must define before use |
| Export to other modules | Functions/types follow module system naturally | `#[macro_export]` required — bypasses module tree |
| Cross-crate references inside macros | N/A | Use `$crate::` to reference the defining crate |
| Importing macros | Module opens (`open`) | Rust 2018+: `use crate::my_macro!`; older: `#[macro_use]` |
