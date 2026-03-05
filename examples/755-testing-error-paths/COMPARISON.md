# OCaml vs Rust: Testing Error Paths

## Error Type Definition

### Rust
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Empty,
    TooLong { len: usize, max: usize },
    InvalidChar { ch: char, pos: usize },
}
```

### OCaml
```ocaml
type parse_error =
  | Empty
  | Too_long of { len: int; max: int }
  | Invalid_char of { ch: char; pos: int }
```

## Testing Specific Error Variants

### Rust
```rust
#[test]
fn test_empty_error() {
    assert_eq!(parse_positive(""), Err(ParseError::Empty));
}

#[test]
fn test_invalid_char_error() {
    assert_eq!(
        parse_positive("12x4"),
        Err(ParseError::InvalidChar { ch: 'x', pos: 2 })
    );
}
```

### OCaml
```ocaml
let%test "empty error" =
  parse_positive "" = Error Empty

let%test "invalid char error" =
  parse_positive "12x4" = Error (Invalid_char { ch = 'x'; pos = 2 })
```

## Pattern Matching on Errors

### Rust
```rust
assert!(matches!(result, Err(ParseError::OutOfRange { .. })));
```

### OCaml
```ocaml
match result with
| Error (Out_of_range _) -> true
| _ -> false
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error comparison | Structural equality | `PartialEq` derive |
| Wildcard match | `_` | `..` for struct fields |
| Error display | Manual `to_string` | `Display` trait |
| Test assertion | `=` | `assert_eq!` |
