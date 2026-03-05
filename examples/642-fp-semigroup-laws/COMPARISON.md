# OCaml vs Rust: Semigroup Laws

## Type Class Definition

### OCaml
```ocaml
module type SEMIGROUP = sig
  type t
  val combine : t -> t -> t
end
```

### Rust
```rust
pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}
```

## Implementation

### OCaml
```ocaml
module SumSemigroup : SEMIGROUP with type t = int = struct
  type t = int
  let combine a b = a + b
end
```

### Rust
```rust
impl Semigroup for Sum<i32> {
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type class mechanism | Module functors | Traits |
| Orphan rules | None (modules explicit) | Coherence rules |
| Newtypes | Less common | Common (Sum, Product) |
| Verification | First-class modules | Generic functions |
