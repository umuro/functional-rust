# OCaml vs Rust: Cartesian Closed Categories

## Curry/Uncurry

### OCaml
```ocaml
let curry f a b = f (a, b)
let uncurry f (a, b) = f a b
```

### Rust
```rust
fn curry<A, B, C>(f: impl FnOnce((A, B)) -> C) -> impl FnOnce(A) -> Box<dyn FnOnce(B) -> C>
```

## Key Insight

Both languages model a CCC:
- `()` = Terminal
- `(A, B)` = Product
- `fn(A) -> B` / `'a -> 'b` = Exponential

OCaml has native currying; Rust requires explicit boxing.
