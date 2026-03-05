# Comparison: Traverse with Result

## Traverse

**OCaml:**
```ocaml
let rec traverse_result f = function
  | [] -> Ok []
  | x :: xs ->
    match f x with
    | Error e -> Error e
    | Ok y -> match traverse_result f xs with
      | Error e -> Error e
      | Ok ys -> Ok (y :: ys)
```

**Rust:**
```rust
fn traverse_result<T, U, E, F: Fn(&T) -> Result<U, E>>(xs: &[T], f: F) -> Result<Vec<U>, E> {
    xs.iter().map(f).collect()  // Built-in!
}
```

## Sequence

**OCaml:**
```ocaml
let sequence_result xs = traverse_result Fun.id xs
```

**Rust:**
```rust
fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()
}
```
