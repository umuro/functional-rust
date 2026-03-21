📖 **[View on hightechmind.io →](https://hightechmind.io/rust/426-proc-macro-function-like)**

---

# 426: Function-like Proc Macros
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Function-like proc macros look like function calls (`my_macro!(...)`) but unlike `macro_rules!`, they receive their entire argument as a `TokenStream` and can generate arbitrary Rust code. This enables domain-specific languages embedded in Rust source: SQL queries (`sql!("SELECT * FROM users WHERE id = $1")` with type-checked parameters), HTML templates, CSS-in-Rust, regex patterns compiled at build time. The `sql!` macro can verify query syntax at compile time and generate typed query structs — impossible with `macro_rules!`.

Function-like proc macros power `quote!` itself, `pest`'s grammar macros, type-checked SQL in `sqlx`, and compile-time regex compilation in `regex_lite`.

## Learning Outcomes

- Understand function-like proc macros as the most flexible macro type (arbitrary input parsing)
- Learn the `#[proc_macro]` registration and the `TokenStream` in/out signature
- See how function-like macros differ from `macro_rules!` (full tokenstream access, external crate)
- Understand compile-time validation use cases: SQL, regex, URIs, JSON schemas
- Learn the ergonomic difference: function-like macros allow custom syntax without `$` sigils

## Rust Application

The `src/lib.rs` demonstrates what function-like proc macros produce. A `sql!` proc macro would parse its string argument, validate SQL syntax at compile time, and generate a typed query struct with parameter types inferred from the schema. A `regex!` proc macro compiles the regex at compile time, avoiding runtime compilation overhead. A `html!` macro parses HTML-like syntax and generates `String`-building code.

## OCaml Approach

OCaml's equivalent is the `[%ppx_name expr]` extension point syntax. `[%sql "SELECT * FROM users"]` with a custom PPX validates SQL at compile time. `[%re "regex"]` compiles regular expressions. OCaml's `ppxlib` extension points provide the same power as function-like proc macros, with the same compile-time validation benefits. The `ocaml-re` library uses this for compile-time regex compilation.

## Key Differences

1. **Syntax freedom**: Rust function-like proc macros accept any token sequence; OCaml extension points must contain valid OCaml expressions inside `[%name ...]`.
2. **Error location**: Rust can attach errors to specific spans within the input; OCaml PPX errors appear at the extension point location.
3. **Schema access**: Both can read external files (SQL schemas, OpenAPI specs) during compilation; Rust uses build scripts to pass paths via env vars.
4. **Caching**: Rust proc macros run on every compilation (unless incremental cache hits); OCaml PPX runs on every file containing the extension.

## Exercises

1. **Compile-time JSON**: Implement `json!({ "key": "value", "num": 42 })` as a function-like macro (or simulate with `macro_rules!`) that validates the JSON structure at compile time and generates a `serde_json::Value` constructor.
2. **Unit literal**: Create `meters!(5.3)` that expands to `Meters(5.3f64)` and `kilometers!(1.2)` to `Kilometers(1.2f64)`. Use function-like proc macros (or `macro_rules!`) to make unit conversion values ergonomic.
3. **Compile-time UUID**: Implement `uuid!("550e8400-e29b-41d4-a716-446655440000")` that validates the UUID format at compile time and generates a `[u8; 16]` constant. Compile-error on malformed input.
