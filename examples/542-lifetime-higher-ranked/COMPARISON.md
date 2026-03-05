# OCaml vs Rust: Higher-Ranked Types

## OCaml
```ocaml
(* Rank-2 polymorphism via records *)
type 'a processor = { process: 'b. 'b -> 'a }

(* Or via first-class modules *)
let apply (type a) (f : a -> a) (x : a) = f x
```

## Rust
```rust
// HRTB: for<'a> quantifies over all lifetimes
pub fn apply_hrtb<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).to_string()
}

// Common use: callbacks that work on any borrow
fn transform<F>(f: F) where F: for<'a> Fn(&'a T) -> &'a T
```

## Key Differences

1. **OCaml**: Rank-2 via records or modules
2. **Rust**: for<'a> syntax for lifetime universality
3. **Rust**: HRTB common for Fn traits with references
4. Both: Enable maximally flexible callbacks
5. **Rust**: Compiler infers HRTB in many cases
