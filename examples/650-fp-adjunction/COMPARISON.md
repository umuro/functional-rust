# OCaml vs Rust: Adjunction

## Curry-Uncurry

### OCaml
```ocaml
let curry f a b = f (a, b)
let uncurry f (a, b) = f a b
```

### Rust
```rust
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where F: Fn(A, B) -> C + Clone + 'static { ... }
```

## State Monad from Adjunction

### OCaml
```ocaml
type ('s, 'a) state = 's -> 'a * 's
let pure a = fun s -> (a, s)
```

### Rust
```rust
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Partial application | Native | Closures + Clone |
| Function types | Lightweight | Box<dyn> for storage |
| Currying | Built-in | Manual implementation |
