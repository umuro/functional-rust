# OCaml vs Rust: Foldable

## Type Definition

### OCaml
```ocaml
module type FOLDABLE = sig
  type 'a t
  val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b
  val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b
end
```

### Rust
```rust
pub trait Foldable {
    type Item;
    fn fold_left<B, F>(self, init: B, f: F) -> B
    where F: FnMut(B, Self::Item) -> B;
    fn fold_right<B, F>(self, init: B, f: F) -> B
    where F: FnMut(Self::Item, B) -> B;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Higher-kinded types | `'a t` in module | Associated type `Item` |
| Parametric polymorphism | Functors | Generics |
| Argument order | `f init container` | `container.fold(init, f)` |
