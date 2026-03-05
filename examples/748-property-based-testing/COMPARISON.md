# OCaml vs Rust: Property-Based Testing

## QuickCheck-style Testing

### OCaml (QCheck)
```ocaml
let test_sort_idempotent =
  QCheck.Test.make ~count:1000
    QCheck.(list small_int)
    (fun l -> List.sort compare (List.sort compare l) = List.sort compare l)

let test_sort_preserves_length =
  QCheck.Test.make ~count:1000
    QCheck.(list small_int)
    (fun l -> List.length (List.sort compare l) = List.length l)
```

### Rust (std-only)
```rust
pub trait Arbitrary: Sized + Clone + Debug {
    fn arbitrary(rng: &mut Lcg) -> Self;
    fn shrink(&self) -> Vec<Self> { vec![] }
}

fn forall<T: Arbitrary, F: FnMut(&T) -> bool>(
    name: &str, tests: usize, mut prop: F
) -> bool {
    let mut rng = Lcg::new(42);
    for _ in 0..tests {
        let input = T::arbitrary(&mut rng);
        if !prop(&input) {
            // Shrink to find minimal counterexample
            return false;
        }
    }
    true
}
```

## Shrinking Counterexamples

### OCaml
```ocaml
(* QCheck provides automatic shrinking *)
QCheck.Shrink.list
```

### Rust
```rust
impl Arbitrary for i32 {
    fn shrink(&self) -> Vec<i32> {
        if *self == 0 { return vec![]; }
        vec![0, self / 2, self.abs() - 1]
            .into_iter()
            .filter(|&x| x.abs() < self.abs())
            .collect()
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Library | QCheck (external) | Can be std-only |
| Shrinking | Built-in combinators | Manual implementation |
| Generator | `QCheck.Gen` | `Arbitrary` trait |
| Integration | Works with OUnit | Works with `#[test]` |
| Determinism | Seeded PRNG | Seeded PRNG |
