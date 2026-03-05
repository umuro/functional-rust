# OCaml vs Rust: Monad Composition

## Monad Transformer

### OCaml
```ocaml
module OptionT (M : MONAD) = struct
  type 'a t = 'a option M.t
  let bind m f = M.bind m (function
    | None -> M.return None
    | Some a -> f a)
end
```

### Rust
```rust
// Manual stacking (no HKT for true transformers)
type OptionResult<A, E> = Result<Option<A>, E>;

fn bind_option_result<A, B, E>(ma: OptionResult<A, E>, f: impl FnOnce(A) -> OptionResult<B, E>) -> OptionResult<B, E> {
    match ma { ... }
}
```

## Key Difference

OCaml can abstract over any monad with modules.
Rust requires concrete type combinations or trait objects.
