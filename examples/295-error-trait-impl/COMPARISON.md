# OCaml vs Rust: std::error::Error Trait

## Pattern 1: Basic Error Implementation

### Rust
```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ParseError { input: String, reason: String }

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse '{}': {}", self.input, self.reason)
    }
}

impl Error for ParseError {}  // source() defaults to None
```

## Pattern 2: Error with Source Chain

### OCaml
```ocaml
exception ChainedError of string * exn

let with_context msg result =
  match result with
  | Ok _ as r -> r
  | Error e -> Error (ChainedError (msg, e))
```

### Rust
```rust
#[derive(Debug)]
struct ValidationError {
    field: String,
    source: Box<dyn Error + Send + Sync>,
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
```

## Pattern 3: Dynamic Error Collection

### Rust
```rust
let errors: Vec<Box<dyn Error>> = vec![
    Box::new(ParseError { ... }),
    Box::new(IoError { ... }),
];
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error trait | No standard | `std::error::Error` |
| Requirements | None | `Display + Debug` |
| Error chaining | Manual cause field | `source()` method |
| Dynamic dispatch | Exceptions are polymorphic | `Box<dyn Error>` |
| Walk chain | Manual traversal | Loop over `.source()` |
