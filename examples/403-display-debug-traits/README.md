📖 **[View on hightechmind.io →](https://hightechmind.io/rust/403-display-debug-traits)**

---

# 403: Display and Debug Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Formatting output for different audiences requires different representations. Debug output should be complete and unambiguous for developers (`Color::Rgb(255, 0, 0)`); display output should be readable for end users (`rgb(255,0,0)`). Mixing these leads to either confusing user-facing output or opaque developer output. Rust separates these concerns with two traits: `Debug` (derivable, for `{:?}`) and `Display` (manual, for `{}`). Additional formatter traits (`LowerHex`, `Binary`, `Octet`, `Pointer`) handle domain-specific representations.

`Display` and `Debug` are the entry points to all of Rust's format machinery — `format!`, `println!`, `write!`, `eprintln!`, and `assert_eq!` error messages all rely on them.

## Learning Outcomes

- Understand the semantic difference between `Debug` (developer) and `Display` (user) formatting
- Learn how to implement `fmt::Display` with `fmt::Formatter` and `write!` macro
- See how `#[derive(Debug)]` generates a complete structural representation automatically
- Understand how to implement additional format traits like `LowerHex` for `{:x}` support
- Learn how `fmt::Formatter` provides alignment, width, fill, and precision parameters

## Rust Application

In `src/lib.rs`, `Color` derives `Debug` automatically and gets `Color::Rgb(r, g, b)` format for free. `Display` is implemented manually to produce `rgb(r,g,b)` and color names. `LowerHex` enables `format!("{:x}", color)` to produce CSS hex colors like `#ff0000`. The `fmt` method receives `&mut fmt::Formatter` and returns `fmt::Result` — all format traits share this signature. The `write!` macro inside `fmt` is the primary building block.

## OCaml Approach

OCaml has `Printf.printf "%s"` for string formatting and `Format.pp_print_*` functions for the `Format` module's structured pretty-printing. Custom types implement `pp : Format.formatter -> t -> unit` functions used with `%a` in format strings. There is no `Display`/`Debug` split — the same `pp` function serves both roles, though libraries conventionally have `pp` (compact) and `show` (verbose) variants.

## Key Differences

1. **Automatic derivation**: Rust's `#[derive(Debug)]` generates complete implementations; OCaml's `deriving show` ppx extension provides similar capability.
2. **Two-trait split**: Rust maintains `Debug` and `Display` as separate traits; OCaml uses one `pp` convention, relying on programmer discipline.
3. **Format string integration**: Rust uses `format!("{}", x)` for Display and `format!("{:?}", x)` for Debug; OCaml uses `%a` with explicit `pp` functions.
4. **Fmt trait family**: Rust has `Debug`, `Display`, `Binary`, `Octal`, `LowerHex`, `UpperHex`, `LowerExp`, `UpperExp`, `Pointer` — a full family; OCaml relies on `Printf` format strings.

## Exercises

1. **Pretty matrix**: Implement `Display` for a `Matrix<f64>` that formats as aligned columns (use `write!(f, "{:8.3}", val)` width specifier) and `Debug` showing the raw flat array.
2. **Recursive tree**: Implement `Display` for a `Tree<i32>` (leaf or node with two children) using indentation — leaves on their own line, nodes with children indented 2 spaces. Use `fmt::Formatter::write_str` for each line.
3. **Custom format trait**: Implement `fmt::Binary` for a `Bitset(u64)` type, formatting it as `0b{bits}` with the set bits indicated. Implement `fmt::Display` to show just the set bit positions as `{1, 3, 7}`.
