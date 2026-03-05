# OCaml vs Rust: Monoid Laws

## Type Definition

### OCaml
```ocaml
module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end
```

### Rust
```rust
pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}
```

## mconcat Implementation

### OCaml
```ocaml
let mconcat (type a) (module M : MONOID with type t = a) items =
  List.fold_left M.combine M.empty items
```

### Rust
```rust
pub fn mconcat<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Identity value | Module value | Associated function |
| First-class modules | Yes | No |
| Iterator protocol | List-based | IntoIterator trait |
