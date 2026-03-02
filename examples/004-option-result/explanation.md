# Example 004: Algebraic Data Types (Option and Result)

## Concept

ADTs represent data with multiple variants. `Option<T>` handles nullable values, `Result<T, E>` handles errors. Both eliminate null pointer exceptions through exhaustive pattern matching.

## Key Mappings

| OCaml | Rust | Purpose |
|-------|------|---------|
| `option 'a` | `Option<T>` | Nullable values |
| `Some x` | `Some(x)` | Present value |
| `None` | `None` | Absent value |
| `('a, 'e) result` | `Result<T, E>` | Error handling |
| `Ok x` | `Ok(x)` | Success |
| `Error e` | `Err(e)` | Failure |

## Monadic Operations

Both languages support monadic chaining:

**OCaml:**
```ocaml
let (>>=) opt f = match opt with
  | None -> None
  | Some x -> f x
```

**Rust:**
```rust
// Built-in as .and_then()
opt.and_then(|x| some_fn(x))
```

## Rust Advantages

1. **Built-in methods** - `.map()`, `.and_then()`, `.unwrap_or()`, etc.
2. **? operator** - Early return on error
3. **Standard library** - Consistent error handling patterns

## Next Steps

Example 005 explores currying and partial application.
