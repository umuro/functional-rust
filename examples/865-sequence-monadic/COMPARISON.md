# Comparison: Sequence Monadic

## Option Sequence

**OCaml:**
```ocaml
let sequence_option xs =
  List.fold_right (fun x acc ->
    match x, acc with
    | Some y, Some ys -> Some (y :: ys)
    | _ -> None
  ) xs (Some [])
```

**Rust:**
```rust
fn sequence_option<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
    xs.into_iter().collect()  // That's it!
}
```

## Generic Sequence

**OCaml:**
```ocaml
let sequence_generic ~bind ~return_ xs =
  List.fold_right (fun mx acc ->
    bind mx (fun x ->
    bind acc (fun xs ->
    return_ (x :: xs)))
  ) xs (return_ [])
```

**Rust (no direct equivalent — collect handles it via FromIterator):**
```rust
// For Option: xs.into_iter().collect::<Option<Vec<_>>>()
// For Result: xs.into_iter().collect::<Result<Vec<_>, _>>()
// The trait system handles the dispatch
```
