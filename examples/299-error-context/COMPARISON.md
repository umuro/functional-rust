# OCaml vs Rust: Error Context

## Pattern: Context Wrapper

### Rust
```rust
#[derive(Debug)]
struct Context<E> {
    message: String,
    source: E,
}

impl<E: Error + 'static> Error for Context<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

// Usage
read_file(path).context("loading config")
```

### OCaml
```ocaml
type 'e context = { message: string; source: 'e }

let with_context msg result =
  match result with
  | Ok v -> Ok v
  | Error e -> Error { message = msg; source = e }
```

## Pattern: Error Chain Traversal

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
let rec print_chain = function
  | { message; source = None } -> Printf.printf "%s\n" message
  | { message; source = Some e } ->
    Printf.printf "%s\n  Caused by: " message;
    print_chain e
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Wrapping | Manual tuple/record | Struct with `Error::source()` |
| Chain traversal | Manual recursion | Standard `source()` linked list |
| Extension method | N/A | `.context()` trait method |
| Display vs cause | Combined | Separate concerns |
