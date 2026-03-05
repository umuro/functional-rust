# Result Type — Railway-Oriented Error Handling: OCaml vs Rust

## The Core Insight
Both languages use Result types for composable error handling, but Rust elevates this pattern to a first-class language feature with the `?` operator. OCaml requires defining custom bind operators; Rust builds them into the syntax. This makes "railway-oriented programming" — where errors automatically short-circuit a pipeline — natural in both languages.

## OCaml Approach
OCaml defines custom operators: `>>=` (bind, aka `and_then`) and `>>|` (map). These compose fallible functions: `parse_int s >>= positive >>= sqrt_safe`. Each step either passes the `Ok` value forward or short-circuits on `Error`. This pattern must be manually implemented (though libraries like `Base` provide it). The operator definitions make the pipeline read left-to-right, mimicking monadic composition from Haskell.

## Rust Approach
Rust's `Result<T, E>` has `.and_then()` and `.map()` as built-in methods, eliminating the need for custom operators. Even better, the `?` operator desugars to an early return on `Err`, making imperative-style code as composable as the functional pipeline. Both styles (`and_then` chains and `?` sequences) are idiomatic and produce identical results.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Bind | Custom `>>=` operator | `.and_then()` method |
| Map | Custom `>>|` operator | `.map()` method |
| Early return | Not available | `?` operator |
| Error propagation | Manual via `>>=` | Automatic via `?` |
| Error type | `string` (polymorphic) | `String` (or custom enum) |
| Parse int | `int_of_string_opt` | `str::parse::<i32>()` |

## What Rust Learners Should Notice
- The `?` operator is syntactic sugar for `match result { Ok(v) => v, Err(e) => return Err(e.into()) }` — it's railway-oriented programming built into the language
- `.and_then()` is the functional style; `?` is the imperative style — both are equally idiomatic in Rust
- Rust's `?` also handles error type conversion via the `From` trait, enabling different error types in a single function
- `.map_err()` transforms the error type without touching the success value — useful for unifying error types
- The monadic pattern (bind/map) is the same in both languages; Rust just provides more syntax sugar

## Further Reading
- [The Rust Book — The ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- [Cornell CS3110 — Error Handling](https://cs3110.github.io/textbook/chapters/data/options.html)
