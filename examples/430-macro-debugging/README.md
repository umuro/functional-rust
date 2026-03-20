📖 **[View on hightechmind.io →](https://hightechmind.io/rust/430-macro-debugging)**

---

# 430: Macro Debugging Techniques

## Problem Statement

Macro expansion errors can be opaque — the compiler shows the expanded output but not always why a specific expansion fails. A macro that works for some inputs but fails for others requires tools to inspect what tokens are being matched and what code is being generated. `cargo-expand` shows the full expanded source, `trace_macros!` shows each expansion step, `stringify!` converts tokens to strings for inspection, and strategic `compile_error!` can reveal what the macro is seeing at a specific expansion point.

Debugging macros is an essential skill: complex `macro_rules!` patterns and proc macros are hard to reason about without tooling.

## Learning Outcomes

- Learn how `cargo-expand` (`cargo expand`) shows the fully expanded macro output
- Understand how `trace_macros!(true)` enables step-by-step expansion tracing (nightly only)
- See how `compile_error!(stringify!($tokens))` reveals what tokens a macro is receiving
- Learn how `dbg!` and `eprintln!` in macro bodies help debug runtime behavior
- Understand how to write failing expansion tests with `trybuild`

## Rust Application

In `src/lib.rs`, `debug_sum!` uses `eprintln!` to print the values being summed — a runtime debugging technique for macro-generated code. `show_expansion!` uses `compile_error!` with `stringify!` to halt compilation with a message showing the exact tokens the macro received. `stringify_args!` converts macro arguments to a `Vec<&str>` for inspection. The combination of `stringify!` + `compile_error!` is a "printf debugging" technique for macro development.

## OCaml Approach

OCaml PPX debugging uses `-ppx` flag with manual invocation to see transformed output. `ocamlfind ppx_deriving/show.ppx file.ml -impl` shows the PPX output. The `Ppx_tools` library provides `Ppx_tools.Genlex` for parsing and `Ppx_tools.Ppx_coptions` for debugging. OCaml doesn't have a direct equivalent of `cargo expand` but `dune describe pp file.ml` shows the preprocessed output.

## Key Differences

1. **Tooling**: `cargo expand` is a widely-used Rust tool with IDE integration; OCaml's equivalent requires more manual invocation.
2. **Compile-time inspection**: Rust's `compile_error!(stringify!(...))` provides in-source debugging; OCaml requires external tool invocation.
3. **Step-by-step**: Rust's `trace_macros!` (nightly) shows each expansion step; OCaml has no equivalent interactive tracing.
4. **Editor integration**: `rust-analyzer` shows macro expansions inline; OCaml editors have limited PPX expansion visualization.

## Exercises

1. **Trace a complex macro**: Take the `min_of!` macro from example 414 and instrument it with `eprintln!` calls to print each recursive step. Verify that `min_of!(5, 3, 8, 1, 4)` correctly traces through the recursion.
2. **show_expansion debugging**: Use the `show_expansion!` technique to understand what tokens a complex macro is receiving. Create a macro that sometimes fails and use `compile_error!(stringify!(...))` to reveal the exact input at the failing arm.
3. **trybuild test**: Set up a `tests/ui/` directory with a failing macro invocation. Write a `tests/macro_tests.rs` using `trybuild::TestCases` that verifies the expected compile error message appears when the macro is misused.
