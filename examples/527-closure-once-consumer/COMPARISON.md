# OCaml vs Rust: FnOnce / Consuming Closures

## OCaml
```ocaml
(* No explicit distinction — GC manages resources *)
let consume_token token = token.value

(* Callbacks typically work the same way *)
let with_resource resource f = f resource
```

## Rust
```rust
// FnOnce: closure that consumes captured values
pub fn make_consumer(token: Token) -> impl FnOnce() -> String {
    move || token.consume()  // token moved, callable only once
}

// with_resource consumes the resource
pub fn with_resource<R, T, F: FnOnce(R) -> T>(resource: R, f: F) -> T {
    f(resource)
}
```

## Key Differences

1. **OCaml**: GC handles resources, no ownership tracking
2. **Rust**: FnOnce ensures closure called at most once
3. **Rust**: Ownership system enforces single-use at compile time
4. **Rust**: FnOnce > FnMut > Fn (hierarchy of capabilities)
5. **Rust**: Move semantics guarantee resource cleanup
