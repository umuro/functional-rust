📖 **[View on hightechmind.io →](https://hightechmind.io/rust/302-option-transpose)**

---

# 302: Option::transpose() — Collecting Optional Results

## Problem Statement

HashMap lookups return `Option<&V>`. Parsing the value returns `Result<T, E>`. The combination is `Option<Result<T, E>>` — but most downstream code wants `Result<Option<T>, E>`. The `Option::transpose()` method handles this conversion. A closely related use case is collecting a `Vec<Option<Result<T, E>>>` where `None` means "absent" and `Err` means "failed to parse", and both need to be handled cleanly.

## Learning Outcomes

- Use `Option<Result<T, E>>::transpose()` to convert to `Result<Option<T>, E>`
- Apply this to map lookups followed by value parsing
- Filter out `None` values while propagating `Err` from a mixed `Vec`
- Understand the semantics: `None` becomes `Ok(None)`, `Some(Ok(v))` becomes `Ok(Some(v))`, `Some(Err(e))` becomes `Err(e)`

## Rust Application

The canonical use case: look up a config key (optional) and parse its value (fallible):

```rust
pub fn lookup_and_parse(
    map: &HashMap<&str, &str>,
    key: &str,
) -> Result<Option<i32>, ParseIntError> {
    map.get(key)           // Option<&&str>
       .map(|s| s.parse()) // Option<Result<i32, _>>
       .transpose()        // Result<Option<i32>, _>
}
// If key absent: Ok(None)
// If key present and valid: Ok(Some(42))
// If key present but invalid: Err(parse error)
```

## OCaml Approach

OCaml requires explicit pattern matching for this transformation:

```ocaml
let lookup_and_parse map key =
  match Hashtbl.find_opt map key with
  | None -> Ok None
  | Some s -> match int_of_string_opt s with
    | None -> Error ("invalid: " ^ s)
    | Some n -> Ok (Some n)
```

## Key Differences

1. **Ergonomics**: Rust's `transpose()` condenses the three-way match into a single method call; OCaml requires explicit nested pattern matching.
2. **Type system**: Rust encodes the transformation in the type system — the compiler rejects incorrect applications.
3. **filter_map interaction**: `filter_map(|opt| opt.map(|s| s.parse::<i32>()).transpose())` elegantly handles None-skip and Err-propagate in one expression.
4. **collect integration**: `iter.filter_map(opt_result).collect::<Result<Vec<_>, _>>()` combines option filtering with result collection cleanly.

## Exercises

1. Parse a `Vec<Option<&str>>` where `None` means "use default 0" and `Some("x")` should propagate as an error, using `transpose()` and `unwrap_or`.
2. Implement a function that reads an optional HTTP header value and parses it as a number, returning `Ok(None)` if absent.
3. Collect a `Vec<Option<Result<i32, E>>>` into a `Result<Vec<i32>, E>`, skipping `None` values and short-circuiting on the first `Err`.
