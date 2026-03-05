# Comparison: Example 220 — Paramorphism

## para Definition

### OCaml
```ocaml
let rec para alg (FixL f as original) =
  let paired = map_f (fun child -> (para alg child, child)) f in
  alg paired
```

### Rust
```rust
fn para<A: Clone>(alg: &dyn Fn(ListF<(A, FixList)>) -> A, fl: &FixList) -> A {
    let paired = fl.0.map_ref(|child| (para(alg, child), child.clone()));
    alg(paired)
}
```

## tails Algebra

### OCaml
```ocaml
let tails_alg = function
  | NilF -> [[]]
  | ConsF (_, (rest_tails, original_tail)) ->
    to_list original_tail :: rest_tails
```

### Rust
```rust
ListF::NilF => vec![vec![]],
ListF::ConsF(_, (rest_tails, original_tail)) => {
    let mut v = vec![to_vec(&original_tail)];
    v.extend(rest_tails);
    v
}
```
