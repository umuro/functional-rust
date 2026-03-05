# Comparison: Example 224 — Mutumorphism

## mutu Definition

### OCaml
```ocaml
let rec mutu alg_a alg_b (FixN f) =
  let paired = map_nat (mutu alg_a alg_b) f in
  (alg_a paired, alg_b paired)
```

### Rust
```rust
fn mutu<A: Clone, B: Clone>(
    alg_a: &dyn Fn(NatF<(A, B)>) -> A,
    alg_b: &dyn Fn(NatF<(A, B)>) -> B,
    fix: &FixNat,
) -> (A, B) {
    let paired = fix.0.map_ref(|child| mutu(alg_a, alg_b, child));
    (alg_a(paired.clone()), alg_b(paired))
}
```

## isEven / isOdd

### OCaml
```ocaml
let is_even_alg = function
  | ZeroF -> true
  | SuccF (_even, odd) -> odd   (* isEven depends on isOdd *)

let is_odd_alg = function
  | ZeroF -> false
  | SuccF (even, _odd) -> even  (* isOdd depends on isEven *)
```

### Rust
```rust
fn is_even_alg(n: NatF<(bool, bool)>) -> bool {
    match n { NatF::ZeroF => true, NatF::SuccF((_even, odd)) => odd }
}
fn is_odd_alg(n: NatF<(bool, bool)>) -> bool {
    match n { NatF::ZeroF => false, NatF::SuccF((even, _odd)) => even }
}
```

## Typed Expression

### OCaml
```ocaml
let val_alg = function
  | AddF ((VInt a, _), (VInt b, _)) -> VInt (a + b)
  | IfF ((VBool true, _), (v, _), _) -> v
  | _ -> VError
```

### Rust
```rust
fn val_alg(e: ExprF<(Value, Typ)>) -> Value {
    match e {
        ExprF::Add((Value::VInt(a), _), (Value::VInt(b), _)) => Value::VInt(a + b),
        ExprF::If((Value::VBool(true), _), (v, _), _) => v,
        _ => Value::VError,
    }
}
```
