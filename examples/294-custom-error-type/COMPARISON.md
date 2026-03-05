# OCaml vs Rust: Custom Error Types

## Pattern 1: Error Enum Definition

### OCaml
```ocaml
type parse_error =
  | InvalidNumber of string
  | OutOfRange of int * int * int
  | EmptyInput
```

### Rust
```rust
#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidNumber(String),
    OutOfRange { value: i64, min: i64, max: i64 },
    EmptyInput,
}
```

## Pattern 2: Error Display

### OCaml
```ocaml
let pp_parse_error = function
  | InvalidNumber s -> Printf.sprintf "invalid: '%s'" s
  | OutOfRange (n, lo, hi) -> 
    Printf.sprintf "%d out of range [%d, %d]" n lo hi
  | EmptyInput -> "empty input"
```

### Rust
```rust
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "invalid: '{}'", s),
            ParseError::OutOfRange { value, min, max } =>
                write!(f, "{} out of range [{}, {}]", value, min, max),
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error type | Variant type or exception | Enum with named variants |
| Display | Ad-hoc function | `impl Display` trait |
| Debug | Automatic | `#[derive(Debug)]` |
| Named fields | Tuples only | Struct-like variants available |
| Exhaustiveness | Compiler checks | Compiler checks `match` arms |
