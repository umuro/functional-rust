# Example 006: Function Composition

## Concept

Function composition combines simple functions into complex pipelines. OCaml uses operators (`<<`, `>>`). Rust uses iterator chaining.

## Key Differences

**OCaml:**
```ocaml
let (>>) f g x = g (f x)  (* Left-to-right composition *)
let complex = square >> double >> add3
```

**Rust:**
```rust
// Manual composition
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(B) -> C, G: Fn(A) -> B
{
    move |x| f(g(x))
}

// Idiomatic: iterator chaining
vec![1, 2, 3]
    .into_iter()
    .map(square)
    .map(double)
    .map(add3)
```

## Rust's Approach

Rust favors **iterator chaining** over composition operators:
- More readable
- Zero-cost abstraction
- Lazy evaluation
- Better type inference

## Next Steps

Examples 007+ will explore type inference, immutability, and tail-call optimization.
