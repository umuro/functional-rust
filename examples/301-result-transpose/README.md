📖 **[View on hightechmind.io →](https://hightechmind.io/rust/301-result-transpose)**

---

# 301: Result::transpose() — Flipping Nested Types

## Problem Statement

When mapping over an `Option<&str>` to parse it, the result is `Option<Result<T, E>>` — an option containing a result. But many APIs expect `Result<Option<T>, E>` — a result containing an optional value. The `transpose()` method converts between these two nested forms, enabling clean composition when optionality and fallibility interact. This is a common need when parsing optional configuration values or handling nullable database fields.

## Learning Outcomes

- Understand `Result<Option<T>, E>::transpose()` → `Option<Result<T, E>>`
- Understand `Option<Result<T, E>>::transpose()` → `Result<Option<T>, E>`
- Use `transpose()` to convert between `Option<Result<_,_>>` and `Result<Option<_>,_>`
- Apply `transpose()` after mapping over an `Option` to parse a value

## Rust Application

`transpose()` is available on both `Result<Option<T>, E>` and `Option<Result<T, E>>`:

```rust
// Parse an optional string into an optional number
pub fn maybe_parse(s: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    s.map(|s| s.parse::<i32>()).transpose()
    // s.map gives Option<Result<i32, _>>
    // .transpose() gives Result<Option<i32>, _>
}
// maybe_parse(Some("42")) -> Ok(Some(42))
// maybe_parse(Some("x"))  -> Err(parse error)
// maybe_parse(None)       -> Ok(None)
```

## OCaml Approach

OCaml does not have a standard `transpose` function. It is implemented manually as a pattern match:

```ocaml
let transpose_opt_result = function
  | None -> Ok None
  | Some (Ok v) -> Ok (Some v)
  | Some (Error e) -> Error e

let transpose_result_opt = function
  | Ok None -> None
  | Ok (Some v) -> Some (Ok v)
  | Error e -> Some (Error e)
```

## Key Differences

1. **Standard library**: Rust provides `transpose()` as a standard method on both `Option` and `Result`; OCaml requires manual pattern matching.
2. **Nullability interaction**: The pattern arises naturally when optional values are parsed — extremely common in config parsing and database nullable field handling.
3. **Composition**: `transpose()` makes it possible to use `collect::<Result<Vec<Option<_>>, _>>()` patterns cleanly.
4. **Symmetry**: The two directions are inverses: `Option::transpose` and `Result::transpose` compose to identity.

## Exercises

1. Parse a `Vec<Option<&str>>` of optional number strings into `Result<Vec<Option<i32>>, E>` using `map`, `transpose`, and `collect`.
2. Write a function that reads optional database fields (represented as `Option<&str>`) and parses them into a struct, using `transpose()` for each field.
3. Demonstrate that `opt.transpose()` and `result.transpose()` are inverses by applying both in sequence and verifying the result.
