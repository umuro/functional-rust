# Comparison: Example 218 — Anamorphism

## ana Definition

### OCaml
```ocaml
let rec ana coalg seed =
  FixL (map_lf (ana coalg) (coalg seed))
```

### Rust
```rust
fn ana<S>(coalg: &dyn Fn(S) -> ListF<S>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|s| ana(coalg, s))))
}
```

## Coalgebra: Range

### OCaml
```ocaml
let range_coalg (lo, hi) =
  if lo > hi then NilF
  else ConsF (lo, (lo + 1, hi))

let range lo hi = ana range_coalg (lo, hi)
```

### Rust
```rust
fn range(lo: i64, hi: i64) -> FixList {
    ana(&|s: (i64, i64)| {
        if s.0 > s.1 { ListF::NilF }
        else { ListF::ConsF(s.0, (s.0 + 1, s.1)) }
    }, (lo, hi))
}
```

## Coalgebra: Collatz

### OCaml
```ocaml
let collatz_coalg n =
  if n <= 1 then ConsF (1, 0)
  else if n mod 2 = 0 then ConsF (n, n / 2)
  else ConsF (n, 3 * n + 1)
```

### Rust
```rust
ana(&|s| {
    if s <= 0 { ListF::NilF }
    else if s == 1 { ListF::ConsF(1, 0) }
    else if s % 2 == 0 { ListF::ConsF(s, s / 2) }
    else { ListF::ConsF(s, 3 * s + 1) }
}, n)
```
