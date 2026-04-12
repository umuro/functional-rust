📖 **[View on hightechmind.io →](https://hightechmind.io/rust/438-macro-format-args)**

---

# 438: `format_args!` and Efficient Formatting
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

String formatting is ubiquitous but has performance implications. `format!("{}: {}", name, value)` always allocates a new `String`. For logging, writing to a buffer, or outputting to a `Write` implementor, you want to format directly without intermediate allocation. `format_args!` returns a `fmt::Arguments` value — a lightweight descriptor of the format string and its arguments — that can be passed to `write!`, `writeln!`, or any `fmt::Write` implementor. This avoids the intermediate `String` allocation when the target already implements `Write`.

`format_args!` is the foundation of all Rust's format macros: `format!`, `println!`, `writeln!`, `eprintln!`, `log::info!` — they all ultimately receive `fmt::Arguments`.

## Learning Outcomes

- Understand `format_args!` as the zero-allocation format descriptor used by `write!` and `writeln!`
- Learn how `write!(buf, ...)` is more efficient than `buf.push_str(&format!(...))` for buffer building
- See how format width (`{:>width$}`) and precision specifiers work programmatically
- Understand `std::fmt::Write` trait enabling custom write targets
- Learn the format performance hierarchy: `write!` > `format!` > string concatenation

## Rust Application

In `src/lib.rs`, `write_to_buffer` uses `write!(buf, ...)` with `String` implementing `fmt::Write` directly. `format_padded` uses `{:>width$}` with a named `width` parameter for dynamic width specification. `format_number` implements thousands-separator formatting by reversing the digit string, inserting commas every 3 digits, then re-reversing. This is an example of manual formatting where `format!` doesn't have the right built-in specifier.

## OCaml Approach

OCaml's `Printf.sprintf`, `Format.sprintf`, and `Buffer.add_string` correspond to Rust's formatting approaches. `Buffer.t` is the direct equivalent of `String` as a write target. `Format.fprintf fmt "..." args` writes to a formatter without intermediate allocation. OCaml's `format` type (for `Printf.printf`/`Format.printf`) is type-checked at compile time similarly to Rust's format strings.

## Key Differences

1. **Type-checked formats**: Both Rust and OCaml check format strings at compile time; Rust uses a macro-based approach, OCaml uses `format` type GADT inference.
2. **Zero-copy writing**: Rust's `write!` avoids intermediate allocation; OCaml's `Buffer.add_string` is similarly efficient.
3. **Dynamic width**: Rust uses `{:>width$}` with named width; OCaml uses `Printf.sprintf "%*s" width s` for dynamic width.
4. **Custom targets**: Rust's `fmt::Write` trait enables any type to be a format target; OCaml's `Format.formatter` is the universal output target.

## Exercises

1. **Log formatter**: Implement `struct LogLine` with `timestamp: u64`, `level: &str`, `message: &str`. Use `write!(buf, ...)` to format it as `"[{timestamp}] {LEVEL}: {message}"` into a pre-allocated `String` buffer, benchmarking against `format!("{}", line)`.
2. **Table formatter**: Build a `TableWriter` implementing `fmt::Write` that collects rows and emits aligned column output when `finish()` is called, computing column widths from all rows.
3. **Number formatting**: Extend `format_number` to support different separators (`,` for US, `.` for EU, `_` for code), and add `format_currency(n: f64, symbol: &str, decimal_places: u8)` for monetary formatting.
