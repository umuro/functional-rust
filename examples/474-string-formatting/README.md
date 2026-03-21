📖 **[View on hightechmind.io →](https://hightechmind.io/rust/474-string-formatting)**

---

# String Formatting
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust's `format!`, `write!`, and `writeln!` macros provide compile-time checked format strings with alignment, precision, padding, and radix specifiers built into the language.

## Problem Statement

Dynamic string construction is one of the most frequent programming operations: generating output, building error messages, constructing SQL queries, serialising data. C's `printf` has no compile-time safety. Python's f-strings and Java's `String.format` check formats at runtime. Rust's format macros check format specifiers against argument types **at compile time**, making mismatches a build error rather than a runtime panic or silent truncation.

## Learning Outcomes

- Use alignment specifiers `{:>width}`, `{:<width}`, `{:^width}` for right/left/centre padding
- Format numbers: hexadecimal `{:x}`, binary `{:b}`, octal `{:o}`, floating-point precision `{:.2}`
- Write directly into a `String` with `write!(buf, ...)` using `fmt::Write`
- Format collections with `{:?}` (Debug) vs. `{}` (Display)
- Understand `format_args!` as the zero-allocation building block underlying all format macros

## Rust Application

Format specifiers follow the pattern `{[argument][':'[fill][align][sign][width][.precision][type]]}`:

```rust
format!("{:>5}", "hi")     // "   hi"  — right-align in 5-char field
format!("{:.2}", 3.14159)  // "3.14"   — 2 decimal places
format!("{:x}", 255u8)     // "ff"     — lowercase hex
```

`write!(s, ...)` appends to any `impl fmt::Write` — including `String` — without allocating a temporary `String` and concatenating:

```rust
let mut s = String::new();
write!(s, "{}", 42).unwrap();
```

`{:?}` invokes the `Debug` trait; `{:#?}` pretty-prints with indentation for nested structures.

## OCaml Approach

OCaml uses `Printf.printf`/`Printf.sprintf` for C-style formatting and `Format.printf` for structured pretty-printing:

```ocaml
Printf.sprintf "%5s" "hi"         (* "   hi" *)
Printf.sprintf "%.2f" 3.14159     (* "3.14"  *)
Printf.sprintf "%x" 255           (* "ff"    *)
```

OCaml 4.08+ added `Format.sprintf` with combinators for complex structures. The `Fmt` library provides composable formatters similar to Rust's `Display`/`Debug` split. OCaml's `Printf` formats are checked at compile time via GADT magic on format strings.

## Key Differences

1. **Compile-time checking**: Both Rust and OCaml check format strings at compile time; Rust uses macro expansion, OCaml uses GADT-typed format strings.
2. **Trait-based extensibility**: Rust's format system is extensible — any type implementing `Display` or `Debug` works; OCaml requires explicit format converters.
3. **`write!` to buffer**: Rust's `fmt::Write` trait allows writing into any buffer (including `Vec<u8>` with `io::Write`); OCaml uses `Buffer.add_string` + `Printf.bprintf`.
4. **Named arguments**: Rust supports `{name}` named format arguments; OCaml's `Printf` uses positional only.

## Exercises

1. **Table formatter**: Write `format_table(headers: &[&str], rows: &[Vec<String>]) -> String` that right-pads each column to its maximum width.
2. **Custom Display**: Implement `fmt::Display` for a `Matrix(Vec<Vec<f64>>)` type that formats it as space-separated rows, each on its own line with 2-decimal-place precision.
3. **No-alloc formatting**: Use `format_args!` with a custom `fmt::Write` implementor that writes directly to a fixed-size `[u8; 128]` buffer without heap allocation.
