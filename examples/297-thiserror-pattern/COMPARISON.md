# OCaml vs Rust: thiserror Pattern

## Pattern: Derive-Based Error Definition

### Rust (with thiserror)
```rust
#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("connection to '{host}' failed")]
    ConnectionFailed { host: String },
    #[error("query failed: {0}")]
    QueryFailed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### Rust (manual equivalent)
```rust
#[derive(Debug)]
pub enum DbError {
    ConnectionFailed { host: String },
    QueryFailed(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed { host } =>
                write!(f, "connection to '{}' failed", host),
            DbError::QueryFailed(sql) =>
                write!(f, "query failed: {}", sql),
        }
    }
}

impl Error for DbError {}

impl From<std::io::Error> for DbError { ... }
```

### OCaml
```ocaml
type db_error =
  | ConnectionFailed of string
  | QueryFailed of string

let string_of_db_error = function
  | ConnectionFailed host -> Printf.sprintf "connection to '%s' failed" host
  | QueryFailed sql -> Printf.sprintf "query failed: %s" sql
```

## Key Differences

| Concept | OCaml | Rust (manual) | Rust (thiserror) |
|---------|-------|---------------|------------------|
| Error messages | Ad-hoc function | `impl Display` | `#[error("...")]` |
| From conversion | Manual | `impl From` | `#[from]` |
| Source chain | Manual field | `fn source()` | Auto from `#[from]` |
| Boilerplate | Minimal | ~30 lines | ~5 lines |
