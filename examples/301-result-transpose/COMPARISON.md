# OCaml vs Rust: transpose

## Pattern: Result<Option<T>> to Option<Result<T>>

### Rust
```rust
let ok_some: Result<Option<i32>, &str> = Ok(Some(42));
ok_some.transpose()  // => Some(Ok(42))

let ok_none: Result<Option<i32>, &str> = Ok(None);
ok_none.transpose()  // => None
```

### OCaml
```ocaml
let transpose = function
  | Ok (Some v) -> Some (Ok v)
  | Ok None -> None
  | Error e -> Some (Error e)
```

## Key Differences

| Input | Result |
|-------|--------|
| `Ok(Some(v))` | `Some(Ok(v))` |
| `Ok(None)` | `None` |
| `Err(e)` | `Some(Err(e))` |

| Concept | OCaml | Rust |
|---------|-------|------|
| Method | Manual match | `.transpose()` |
| Bidirectional | Two functions | Same method on both types |
| Use case | Composing Option and Result | Same |
