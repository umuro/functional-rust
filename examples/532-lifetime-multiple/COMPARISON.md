# OCaml vs Rust: Multiple Lifetimes

## OCaml
```ocaml
(* No concept of multiple lifetimes — GC handles all *)
type pair = { first: string; second: string }

let first_of x _y = x
let make_pair first second = { first; second }
```

## Rust
```rust
// Independent lifetimes for different fields
pub struct Pair<'a, 'b> {
    pub first: &'a str,
    pub second: &'b str,
}

// Output tied to first input only
pub fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x  // 'b can be shorter than 'a
}
```

## Key Differences

1. **OCaml**: Single memory model, GC tracks all references
2. **Rust**: Multiple lifetimes express independent validity
3. **Rust**: 'a and 'b can have different scopes
4. **Rust**: Return type specifies which lifetime applies
5. **Rust**: Enables borrowing from multiple sources safely
