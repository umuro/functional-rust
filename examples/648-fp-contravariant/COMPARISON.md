# OCaml vs Rust: Contravariant Functor

## Type Definition

### OCaml
```ocaml
module type CONTRAVARIANT = sig
  type 'a t
  val contramap : ('b -> 'a) -> 'a t -> 'b t
end
```

### Rust
```rust
pub trait Contravariant<A> {
    type Output<B>;
    fn contramap<B, F>(self, f: F) -> Self::Output<B>
    where F: FnOnce(B) -> A;
}
```

## Predicate Implementation

### OCaml
```ocaml
let contramap f p = fun b -> p (f b)
```

### Rust
```rust
fn contramap_predicate<A, B, F>(pred: impl Fn(&A) -> bool, f: F) -> impl Fn(&B) -> bool
where F: Fn(&B) -> A {
    move |b| pred(&f(b))
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Closures | Lightweight | Heap allocation often needed |
| References | Less explicit | Explicit with `&` |
| Composition | Natural with `|>` | Method chains |
