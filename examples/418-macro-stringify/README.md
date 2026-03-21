📖 **[View on hightechmind.io →](https://hightechmind.io/rust/418-macro-stringify)**

---

# 418: `stringify!` and `concat!` Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Some code patterns require converting source code tokens to string literals at compile time: debug output showing the expression that was evaluated, test assertions printing the failing condition's source text, enum variants converting to their names. The built-in `stringify!($e)` macro captures an expression as a `&'static str` without evaluating it. `concat!` joins string literals at compile time. Together, these enable zero-runtime-cost reflective string generation from source code.

`stringify!` is used in `assert_eq!`'s error messages (printing the two expression texts), `dbg!` macro, `std::env!`-based build scripts, and any diagnostic macro that should show "what the programmer wrote."

## Learning Outcomes

- Understand how `stringify!($expr)` captures the token representation of an expression
- Learn how `concat!` joins multiple string literals and `stringify!` outputs into a single `&'static str`
- See how `string_enum!` uses `stringify!` to generate `as_str()` methods automatically
- Understand the difference: `stringify!(1 + 2)` produces `"1 + 2"`, not `"3"`
- Learn how these macros enable zero-cost reflective debugging

## Rust Application

In `src/lib.rs`, `dbg_named!($val:expr)` uses `stringify!($val)` to get the textual name of the expression, then evaluates `$val` and prints both. `assert_dbg!($cond:expr)` includes `stringify!($cond)` in the panic message so failing assertions show the condition text. `string_enum!` uses `stringify!($variant)` in the generated `as_str()` match arm to produce the variant name string — automatically keeping enum variant names and their string forms in sync.

## OCaml Approach

OCaml's `[%string "..."]` syntax and `Printf.sprintf` format strings handle string construction. For reflective output (showing the source expression), OCaml PPX is required — the `ppx_expect` and `ppx_sexp_conv` extensions generate test output showing expression text. Without PPX, OCaml has no built-in way to turn an expression into its source text. The `__LOC__` and `__FUNCTION__` special values provide location information.

## Key Differences

1. **Source capture**: Rust's `stringify!` captures token text without evaluation; OCaml has no equivalent without PPX.
2. **Compile-time string joining**: Rust's `concat!` joins at compile time; OCaml uses `^` (runtime) or `Printf.sprintf` (runtime).
3. **Location info**: Both have `file!()` / `__FILE__` and `line!()` / `__LINE__`; Rust's are macros, OCaml's are special values.
4. **Zero runtime cost**: `stringify!` and `concat!` produce `&'static str` with no runtime allocation; OCaml's equivalent string operations allocate on the GC heap.

## Exercises

1. **Trace macro**: Implement `trace!(expr)` that prints `"TRACE: {file}:{line}: {expr_text} = {val:?}"` and returns the value. Use it to instrument a sorting algorithm and observe which comparisons are made.
2. **Contract macro**: Implement `requires!(cond, "precondition description")` that in debug builds asserts the condition and in release builds is a no-op. Use `stringify!(cond)` in the assertion message.
3. **Enum display**: Use `string_enum!` to define `Direction { North, South, East, West }` and verify that `Direction::North.as_str() == "North"`. Then implement `Display` for the enum using the `as_str()` method.
