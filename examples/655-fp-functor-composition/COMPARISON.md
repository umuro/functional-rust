# OCaml vs Rust: Functor Composition

## Mapping Nested Functors

### OCaml
```ocaml
let map_option_list f ol = Option.map (List.map f) ol
```

### Rust
```rust
fn map_option_vec<A, B>(ov: Option<Vec<A>>, f: impl Fn(A) -> B) -> Option<Vec<B>> {
    ov.map(|v| v.into_iter().map(f).collect())
}
```

## Key Insight

Both approaches map the inner functor (G) first, then lift through outer (F).
The composition F ∘ G maps via: `F.map(G.map(f))`

## Type Aliases

### OCaml
```ocaml
type 'a option_list = 'a list option
```

### Rust
```rust
type OptionVec<A> = Option<Vec<A>>;
```
