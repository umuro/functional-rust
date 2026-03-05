# OCaml vs Rust: Topos Concepts

## Subobject Classifier

### OCaml
```ocaml
type omega = True | False
let char_fn subset x = if List.mem x subset then True else False
```

### Rust
```rust
enum Omega { True, False }
fn char_fn<T: PartialEq>(subset: &[T]) -> impl Fn(&T) -> Omega + '_ {
    move |x| if subset.contains(x) { Omega::True } else { Omega::False }
}
```

## Key Insight

Types in both languages form a (weak) topos:
- Subobject classifier = `bool` / `Omega`
- Power object = predicates / sets
- Internal logic = type-level reasoning
