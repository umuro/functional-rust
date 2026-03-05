# OCaml vs Rust: Kan Extensions

## Codensity Monad

### OCaml
```ocaml
type 'a t = { run : 'r. ('a -> 'r) -> 'r }
let bind m f = { run = fun k -> m.run (fun a -> (f a).run k) }
```

### Rust
```rust
struct Codensity<A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(A) -> A>) -> A>,
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Rank-2 polymorphism | Native | Approximated |
| Continuation type | Polymorphic | Fixed to `A -> A` |
| Existentials (Lan) | GADTs | Trait objects |
