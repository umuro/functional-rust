# OCaml vs Rust: Traversable

## Traverse Implementation

### OCaml
```ocaml
let traverse_option f xs =
  let rec go acc = function
    | [] -> Some (List.rev acc)
    | x :: rest ->
      match f x with
      | None -> None
      | Some y -> go (y :: acc) rest
  in go [] xs
```

### Rust
```rust
fn traverse_option<A, B, F>(xs: Vec<A>, mut f: F) -> Option<Vec<B>>
where F: FnMut(A) -> Option<B> {
    let mut result = Vec::new();
    for x in xs {
        result.push(f(x)?);
    }
    Some(result)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error propagation | Pattern matching | `?` operator |
| Iteration style | Recursive | Imperative loop |
| Memory | Immutable cons | Mutable push |
