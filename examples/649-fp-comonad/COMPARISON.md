# OCaml vs Rust: Comonad

## Core Operations

### OCaml
```ocaml
module type COMONAD = sig
  type 'a t
  val extract : 'a t -> 'a
  val extend : ('a t -> 'b) -> 'a t -> 'b t
end
```

### Rust
```rust
pub trait Comonad: Sized {
    type Item;
    fn extract(&self) -> Self::Item;
    fn extend<B, F>(self, f: F) -> Self where F: Fn(&Self) -> B;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Self-reference | Easy via HKT | Requires careful design |
| Cloning | Implicit | Explicit Clone bound |
| Duplicate | `extend id` | Needs Clone + Output type |
