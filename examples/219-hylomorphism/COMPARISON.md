# Comparison: Example 219 — Hylomorphism

## hylo Definition

### OCaml
```ocaml
let rec hylo alg coalg seed =
  alg (map_f (hylo alg coalg) (coalg seed))
```

### Rust
```rust
fn hylo<S, A>(alg: &dyn Fn(ListF<A>) -> A, coalg: &dyn Fn(S) -> ListF<S>, seed: S) -> A {
    alg(coalg(seed).map(|s| hylo(alg, coalg, s)))
}
```

## Factorial

### OCaml
```ocaml
let factorial n = hylo
  (function NilF -> 1 | ConsF (n, acc) -> n * acc)
  (fun n -> if n <= 0 then NilF else ConsF (n, n - 1))
  n
```

### Rust
```rust
fn factorial(n: i64) -> i64 {
    hylo(
        &|l| match l { ListF::NilF => 1, ListF::ConsF(n, acc) => n * acc },
        &|n| if n <= 0 { ListF::NilF } else { ListF::ConsF(n, n - 1) },
        n,
    )
}
```

## Merge Sort (Tree Hylo)

### OCaml
```ocaml
let merge_sort xs = hylo_tree merge_alg split_coalg xs
```

### Rust
```rust
fn merge_sort(xs: Vec<i64>) -> Vec<i64> {
    if xs.is_empty() { return vec![]; }
    hylo_tree(&merge_alg, &split_coalg, xs)
}
```
