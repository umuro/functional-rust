# Comparison: Example 222 — Histomorphism

## Cofree Type

### OCaml
```ocaml
type 'a cofree_nat = CF of 'a * 'a cofree_nat nat_f
let head (CF (a, _)) = a
let tail (CF (_, t)) = t
```

### Rust
```rust
struct Cofree<A> {
    head: A,
    tail: Box<NatF<Cofree<A>>>,
}
```

## Fibonacci Algebra (Look Back 2 Steps)

### OCaml
```ocaml
let fib_alg = function
  | ZeroF -> 0
  | SuccF (CF (n1, ZeroF)) -> max 1 n1
  | SuccF (CF (n1, SuccF (CF (n2, _)))) -> n1 + n2
```

### Rust
```rust
fn fib_alg(n: NatF<Cofree<u64>>) -> u64 {
    match n {
        NatF::ZeroF => 0,
        NatF::SuccF(cf) => match cf.tail.as_ref() {
            NatF::ZeroF => 1,
            NatF::SuccF(cf2) => cf.head + cf2.head,
        }
    }
}
```

## histo Implementation

### OCaml
```ocaml
let rec histo_simple alg (FixN f) =
  alg (map_nat (histo_build alg) f)
and histo_build alg node =
  let result = histo_simple alg node in
  CF (result, map_nat (histo_build alg) (match node with FixN g -> g))
```

### Rust
```rust
fn histo<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> A {
    histo_build(alg, fix).head
}
fn histo_build<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> Cofree<A> {
    let layer = fix.0.map_ref(|child| histo_build(alg, child));
    let result = alg(layer.clone());
    Cofree::new(result, layer)
}
```
