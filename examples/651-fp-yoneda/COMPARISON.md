# OCaml vs Rust: Yoneda Lemma

## Yoneda Representation

### OCaml
```ocaml
type 'a t = { run : 'r. ('a -> 'r) -> 'r list }
let map f y = { run = fun g -> y.run (fun x -> g (f x)) }
```

### Rust
```rust
struct YonedaVec<A> {
    run: Box<dyn FnOnce() -> Vec<A>>,
}
// Simplified due to lack of rank-2 types
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Rank-2 types | Native with records | Not directly supported |
| Polymorphism | Implicit forall | Generic parameters |
| Implementation | True Yoneda | CPS-style approximation |
| GADT for Coyoneda | Existential types | Boxed trait objects |
