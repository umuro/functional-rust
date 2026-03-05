# OCaml vs Rust: Variance

## OCaml
```ocaml
(* Variance annotations in type definitions *)
type +'a covariant = 'a list
type -'a contravariant = 'a -> unit
type 'a invariant = { mutable value: 'a }
```

## Rust
```rust
// Covariant: &'a T, Box<T>, Vec<T>
// Contravariant: fn(T) -> ()
// Invariant: &'a mut T, Cell<T>

// 'static can be used as shorter lifetime
fn demo<'short>(s: &'static str) -> &'short str { s }
```

## Key Differences

1. **OCaml**: Variance via +/- annotations on type params
2. **Rust**: Variance determined by type structure
3. Both: Covariant = same direction as subtyping
4. Both: Contravariant = opposite direction
5. Both: Invariant = no subtyping allowed
