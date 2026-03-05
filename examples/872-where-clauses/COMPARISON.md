# Comparison: Where Clauses

## Inline vs Where Clause

**Rust — Inline bounds (simple):**
```rust
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a >= b { a } else { b })
}
```

**Rust — Where clause (complex):**
```rust
fn transform_and_combine<T, U, A, F, G>(items: &[T], transform: F, combine: G, init: A) -> A
where
    F: Fn(&T) -> U,
    G: Fn(A, U) -> A,
{
    items.iter().fold(init, |acc, x| combine(acc, transform(x)))
}
```

## OCaml Equivalent — No Explicit Constraints

**OCaml:**
```ocaml
let transform_and_combine ~transform ~combine ~init items =
  List.fold_left (fun acc x -> combine acc (transform x)) init items
(* Types are fully inferred, no constraints written *)
```

**Rust:**
```rust
fn transform_and_combine<T, U, A, F, G>(items: &[T], transform: F, combine: G, init: A) -> A
where F: Fn(&T) -> U, G: Fn(A, U) -> A,
{ /* ... */ }
```

## Multiple Related Bounds

**OCaml:**
```ocaml
let sorted_summary items to_str =
  let sorted = List.sort compare items in
  String.concat ", " (List.map to_str sorted)
```

**Rust:**
```rust
fn sorted_summary<T>(items: &mut [T]) -> String
where
    T: Ord + Display,
{
    items.sort();
    items.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
}
```
