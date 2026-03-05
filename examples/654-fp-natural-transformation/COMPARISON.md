# OCaml vs Rust: Natural Transformations

## Option to List/Vec

### OCaml
```ocaml
let option_to_list = function
  | None -> []
  | Some x -> [x]
```

### Rust
```rust
fn option_to_vec<A>(opt: Option<A>) -> Vec<A> {
    opt.into_iter().collect()
}
```

## Naturality Verification

Both languages can verify naturality through tests:
- Apply f then transform
- Transform then apply f
- Compare results

## Key Insight

Natural transformations are just polymorphic functions that respect structure.
In Rust: generic functions with proper constraints.
In OCaml: parametric polymorphic functions.
