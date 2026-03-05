# Comparison: Result Monad

## Bind Chain

**OCaml:**
```ocaml
let validate_input s =
  parse_int s >>= check_positive >>= check_even
```

**Rust:**
```rust
fn validate_input(s: &str) -> Result<i32, String> {
    parse_int(s)
        .and_then(check_positive)
        .and_then(check_even)
}
```

## Rust's ? Operator

**Rust:**
```rust
fn validate_input(s: &str) -> Result<i32, String> {
    let n = parse_int(s)?;       // early return on Err
    let n = check_positive(n)?;  // early return on Err
    check_even(n)                // final result
}
```

## Custom Error Types

**OCaml:**
```ocaml
type validation_error =
  | ParseError of string
  | NotPositive of int
  | NotEven of int
```

**Rust:**
```rust
#[derive(Debug)]
enum ValidationError {
    ParseError(String),
    NotPositive(i32),
    NotEven(i32),
}

// ? operator works with From trait for auto-conversion
```
