📖 **[View on hightechmind.io →](https://hightechmind.io/rust/877-display-trait)**

---

# 877-display-trait — Display Trait

## Problem Statement

Every user-facing type eventually needs a string representation. In Java, `toString()` is defined on `Object` and returns `String`. In Python, `__str__` serves the same role. Rust splits this into two distinct traits: `Display` for human-readable output (`{}` format) and `Debug` for developer-diagnostic output (`{:?}` format). This separation prevents the common bug of accidentally showing internal debug details to end users. OCaml uses `to_string` functions by convention (no enforced interface), or `Format.fprintf` for more complex formatting. Implementing `Display` unlocks `format!`, `println!`, `to_string()`, and any generic function bounded on `Display`.

## Learning Outcomes

- Implement `fmt::Display` for custom types to enable `{}` formatting
- Distinguish between `Display` (user-facing) and `Debug` (developer-facing)
- Write multi-line and nested `Display` implementations for structured types
- Understand how implementing `Display` automatically provides `to_string()`
- Compare with OCaml's `to_string` convention and `Format` module

## Rust Application

The code implements `Display` for `Color` (enum formatting), `Point` (coordinate formatting), and `Matrix` (multi-line grid formatting). For `Point`, both `Display` (`(1.00, 2.00)`) and `Debug` (`Point{ x: 1, y: 2 }`) are implemented, showing different representations. The `Matrix` implementation uses a loop inside `fmt` to produce multi-line output, using `writeln!` for all but the last row. Each `impl fmt::Display for T` provides `T::to_string()` for free via the blanket implementation in `std`.

## OCaml Approach

OCaml has no enforced `Display` interface. Convention is to define a `to_string: t -> string` function per module. For complex formatting, the `Format` module provides `fprintf`, `printf`, and `sprintf` with `%a` for custom formatters. `Format.pp_print_string`, `Format.pp_print_int` etc. compose into pretty-printers. OCaml's `[@@deriving show]` (via ppx_deriving) can auto-generate `show` functions similar to Rust's `#[derive(Debug)]`.

## Key Differences

1. **Trait vs convention**: Rust `Display` is a formal trait enforced by the compiler; OCaml `to_string` is a naming convention with no type-system enforcement.
2. **Automatic to_string**: Rust derives `to_string()` for free from `Display`; OCaml requires explicit `to_string` implementation.
3. **Format string integration**: Rust's `format!("{}", val)` invokes `Display`; OCaml uses `Printf.sprintf "%s" (to_string val)` or `Format.asprintf`.
4. **Debug vs Display**: Rust enforces the distinction via two separate traits; OCaml has no built-in equivalent separation.

## Exercises

1. Implement `Display` for a `FractionDisplay(i64, i64)` type that shows fractions like `3/4` in simplified form.
2. Write a `Table` struct with a `Vec<Vec<String>>` grid and implement `Display` to render it with column alignment.
3. Implement a `Duration` type wrapping seconds and implement `Display` to show `1h 23m 45s` formatted output.
