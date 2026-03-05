# Comparison: Example 225 — Prepromorphism

## prepro Definition

### OCaml
```ocaml
let rec prepro nat alg (Fix f) =
  alg (map_f (fun child ->
    prepro nat alg (Fix (nat (unfix child)))
  ) f)
```

### Rust
```rust
fn prepro<A>(
    nat: &dyn Fn(ExprF<Fix>) -> ExprF<Fix>,
    alg: &dyn Fn(ExprF<A>) -> A,
    Fix(f): &Fix,
) -> A {
    alg(f.map_ref(|child| {
        let transformed = Fix(Box::new(nat(child.0.as_ref().clone())));
        prepro(nat, alg, &transformed)
    }))
}
```

## Natural Transformation: Mul → Add

### OCaml
```ocaml
let mul_to_add = function
  | MulF (a, b) -> AddF (a, b)
  | other -> other
```

### Rust
```rust
fn mul_to_add(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::MulF(a, b) => ExprF::AddF(a, b),
        other => other,
    }
}
```

## Remove Negation

### OCaml
```ocaml
let remove_neg = function
  | NegF a -> unfix a
  | other -> other
```

### Rust
```rust
fn remove_neg(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::NegF(a) => a.0.as_ref().clone(),
        other => other,
    }
}
```
