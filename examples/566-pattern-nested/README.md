📖 **[View on hightechmind.io →](https://hightechmind.io/rust/566-pattern-nested)**

---

# Nested Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Real data structures are rarely flat — they are trees: enums containing structs containing options containing results. Nested patterns allow matching deeply nested structures in a single expression, extracting values from multiple levels simultaneously. Without nested patterns, extracting a value from `Some(Some(Point { x, y }))` would require multiple nested `match` statements. Nested patterns are essential in AST traversal, JSON processing, configuration parsing, and any domain with layered data.

## Learning Outcomes

- How `Outer { inner: Inner { value } }` matches and extracts from nested structs
- How `Some(Some(v))` matches and extracts from nested `Option`
- How `Ok(Ok(v))` and `Ok(Err(msg))` handle nested `Result`
- How to combine struct destructuring, enum variants, and tuple patterns in one expression
- Where nested patterns appear: AST traversal, JSON manipulation, nested configuration

## Rust Application

`get_value(o: &Outer)` uses `Outer { inner: Inner { value } } => *value` — two levels of struct destructuring in one pattern. `unwrap_nested(opt: Option<Option<i32>>)` matches `Some(Some(v))`, `Some(None)`, and `None` — two levels of `Option`. `process_nested(res: Result<Result<i32, &str>, &str>)` handles all four combinations of nested `Result`. These patterns avoid deep nesting of `match` and `if let` chains.

Key patterns:
- `Outer { inner: Inner { value } }` — nested struct
- `Some(Some(v))` — nested Option unwrap
- `Ok(Err(msg))` — nested Result pattern
- Combining: `Some(Struct { field: Some(v) })`

## OCaml Approach

OCaml nested patterns are identical in expressive power:

```ocaml
let get_value { inner = { value } } = value
let unwrap_nested = function
  | Some (Some v) -> v
  | Some None -> -1
  | None -> -2
```

The syntax differs slightly but the capability is the same.

## Key Differences

1. **Struct syntax**: Rust `Outer { inner: Inner { value } }` uses braces; OCaml `{ inner = { value } }` uses `=` for record fields.
2. **Pattern depth**: Both languages support arbitrarily deep nested patterns with the same compile-time exhaustiveness checking.
3. **Readability**: Very deep nesting can become hard to read in both — the `?` operator and `and_then` chains are often cleaner for nested `Option`/`Result`.
4. **Performance**: Nested patterns compile to nested conditional jumps — the same as explicit nested `match` but without the code duplication.

## Exercises

1. **Three-level nest**: Create `struct A { b: Option<B> }; struct B { c: Vec<C> }; struct C { value: i32 }` and write a function that extracts the first `C.value` if all levels are present.
2. **Nested config**: Implement a `Config { database: Option<DbConfig { host: String, port: u16 }> }` and write `fn db_host(c: &Config) -> Option<&str>` using a single nested pattern.
3. **AST node**: Define a simple arithmetic AST `Expr { Add(Box<Expr>, Box<Expr>), Lit(i32) }` and write an `eval` function using nested match patterns.
