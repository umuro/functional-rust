# Comparison: Example 221 — Apomorphism

## apo Definition

### OCaml
```ocaml
let rec apo coalg seed =
  FixL (map_f (function
    | Left fix -> fix
    | Right s -> apo coalg s
  ) (coalg seed))
```

### Rust
```rust
fn apo<S>(coalg: &dyn Fn(S) -> ListF<Either<FixList, S>>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|either| match either {
        Either::Left(fix) => fix,
        Either::Right(s) => apo(coalg, s),
    })))
}
```

## Insert into Sorted List

### OCaml
```ocaml
let insert_coalg x = function
  | FixL NilF -> ConsF (x, Left (FixL NilF))
  | FixL (ConsF (y, rest)) as original ->
    if x <= y then ConsF (x, Left original)   (* short-circuit *)
    else ConsF (y, Right rest)                  (* continue *)
```

### Rust
```rust
apo(&|fl: FixList| match fl.0.as_ref() {
    ListF::NilF => ListF::ConsF(x, Either::Left(nil())),
    ListF::ConsF(y, rest) =>
        if x <= *y { ListF::ConsF(x, Either::Left(fl.clone())) }
        else { ListF::ConsF(*y, Either::Right(rest.clone())) },
}, lst)
```
