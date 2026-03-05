# Comparison: Traverse with Option

## Traverse

**OCaml:**
```ocaml
let rec traverse_option f = function
  | [] -> Some []
  | x :: xs ->
    match f x with
    | None -> None
    | Some y ->
      match traverse_option f xs with
      | None -> None
      | Some ys -> Some (y :: ys)
```

**Rust (built-in via collect!):**
```rust
fn traverse_option<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().map(f).collect()  // That's it!
}
```

## Sequence

**OCaml:**
```ocaml
let sequence_option xs = traverse_option Fun.id xs
(* [Some 1; Some 2; Some 3] → Some [1; 2; 3] *)
```

**Rust:**
```rust
fn sequence_option<T: Clone>(xs: &[Option<T>]) -> Option<Vec<T>> {
    xs.iter().cloned().collect()
}
```

## Fold-Based Traverse

**OCaml:**
```ocaml
let traverse_option_fold f xs =
  List.fold_right (fun x acc ->
    match f x, acc with
    | Some y, Some ys -> Some (y :: ys)
    | _ -> None
  ) xs (Some [])
```

**Rust:**
```rust
fn traverse_option_fold<T, U, F: Fn(&T) -> Option<U>>(xs: &[T], f: F) -> Option<Vec<U>> {
    xs.iter().try_fold(Vec::new(), |mut acc, x| {
        acc.push(f(x)?);
        Some(acc)
    })
}
```
