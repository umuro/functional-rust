# OCaml vs Rust: Error Chaining

## Pattern: Source Chain

### Rust
```rust
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

### OCaml
```ocaml
type config_error = { cause: file_error option }
(* No standard trait - manual chaining *)
```

## Pattern: Walking the Chain

### Rust
```rust
let mut cause = e.source();
while let Some(c) = cause {
    println!("  Caused by: {}", c);
    cause = c.source();
}
```

### OCaml
```ocaml
let rec walk_chain = function
  | None -> ()
  | Some e ->
    Printf.printf "  Caused by: %s\n" (string_of_error e);
    walk_chain e.cause
```

## Pattern: Finding Root Cause

### Rust
```rust
fn root_cause(e: &dyn Error) -> &dyn Error {
    let mut current = e;
    while let Some(source) = current.source() {
        current = source;
    }
    current
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Chain mechanism | Manual cause field | `Error::source()` |
| Standard interface | None | `std::error::Error` trait |
| Root cause | Manual traversal | Same pattern works |
| Pretty printing | Custom function | Loop over `source()` |
| Polymorphism | Exception hierarchy | `dyn Error` trait object |
