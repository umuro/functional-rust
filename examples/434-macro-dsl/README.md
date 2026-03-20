📖 **[View on hightechmind.io →](https://hightechmind.io/rust/434-macro-dsl)**

---

# 434: Domain-Specific Languages with Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Some problem domains have natural syntaxes that are more readable and less error-prone than general-purpose code. SQL queries, HTML templates, configuration languages, and test specifications benefit from DSL syntax. `macro_rules!` can implement simple DSLs within Rust source: `query!(SELECT name, age FROM users WHERE "active = true")` is safer and more readable than string concatenation. The macro validates structure at compile time (correct SQL keywords, table identifiers) and generates the underlying representation.

DSL macros appear in `sqlx`'s typed queries, `html!` in Yew, `css!` in Stylist, and testing DSLs like `rstest`'s table-driven tests.

## Learning Outcomes

- Understand how `macro_rules!` can parse domain-specific syntax using identifier and expression fragments
- Learn the `SELECT $cols FROM $table WHERE $cond` pattern matching technique
- See how `stringify!($ident)` converts macro identifiers to strings for query building
- Understand the limitations: macros parse tokens, not semantics (no SQL schema validation)
- Learn when DSL macros are appropriate (stable syntax, compile-time structure checking)

## Rust Application

In `src/lib.rs`, `query!(SELECT name, age FROM users)` matches the two-arm macro: `SELECT $($col:ident),+ FROM $table:ident`. The identifiers are converted to strings with `stringify!`, joined, and formatted into a SQL string. The `WHERE $cond:expr` arm adds a condition expression. `html!($tag:ident { $content:expr })` generates HTML tags using identifier names for tag names.

## OCaml Approach

OCaml implements DSLs through its parsing infrastructure. The `menhir` parser generator creates parsers for arbitrary grammars. PPX extensions enable `[%sql "SELECT ..."]` syntax validated by the PPX. `angstrom` provides parser combinators for runtime DSL parsing. OCaml's quotations (in camlp5) support `<:expr< ... >>` syntax for embedding OCaml-like expressions — similar to Rust's `quote!` but for OCaml ASTs.

## Key Differences

1. **Token matching**: Rust DSL macros parse token streams with pattern matching; OCaml uses formal grammars (Menhir) or combinator parsers.
2. **Compile-time validation**: Both can validate DSL syntax at compile time; Rust through macro arm matching, OCaml through PPX.
3. **Runtime DSLs**: OCaml's parser ecosystem (`angstrom`, `sedlex`) handles runtime DSL parsing elegantly; Rust uses `nom`, `pest`, or `chumsky`.
4. **Expressiveness**: Complex DSLs with context-sensitive rules exceed `macro_rules!` capability; both fall back to proc macros / PPX for complex cases.

## Exercises

1. **Route DSL**: Implement `route!(GET "/users/{id}" => handler_fn)` that generates a `Route { method: Method::Get, path: "/users/{id}", handler: handler_fn }` struct. Parse `GET`/`POST`/`PUT`/`DELETE` as ident fragments.
2. **Test table DSL**: Create `test_cases!( add | a | b | result | 1 | 2 | 3, 4 | 5 | 9 )` that generates individual test functions for each row, asserting `add(a, b) == result`.
3. **Config DSL**: Implement `config!{ timeout: 30, host: "localhost", port: 8080 }` producing a `Config { timeout: 30, host: "localhost".to_string(), port: 8080 }` value, handling integer and string literals with appropriate type coercion.
