# OCaml vs Rust: Bifunctor

## Type Definition

### OCaml
```ocaml
module type BIFUNCTOR = sig
  type ('a, 'b) t
  val bimap : ('a -> 'c) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end
```

### Rust
```rust
pub trait Bifunctor<A, B> {
    type Output<C, D>;
    fn bimap<C, D, F, G>(self, f: F, g: G) -> Self::Output<C, D>
    where F: FnOnce(A) -> C, G: FnOnce(B) -> D;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type constructor arity | Native 2-param | Associated type |
| Currying | Built-in | Manual closures |
| Either type | Custom ADT | Custom enum |
