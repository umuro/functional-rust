# OCaml vs Rust: Kleisli Category

## Fish Operator (>=>)

### OCaml
```ocaml
let ( >=> ) f g = fun x ->
  match f x with
  | None -> None
  | Some y -> g y
```

### Rust
```rust
fn fish<A, B, C>(
    f: impl FnOnce(A) -> Option<B>,
    g: impl FnOnce(B) -> Option<C>,
) -> impl FnOnce(A) -> Option<C> {
    move |a| f(a).and_then(g)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Operator syntax | Native `>=>` | Function call |
| Closure costs | Lightweight | Potential boxing |
| Composition | Infix chaining | Nested calls |
