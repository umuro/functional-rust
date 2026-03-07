# Example 002: Function Composition

**Difficulty:** ⭐
**Category:** Higher-Order Functions | Closures | Composition
**OCaml Source:** Functional Programming fundamentals

## Problem Statement

Write a function `compose` that takes two functions `f` and `g` and returns their composition—a new function that applies `g` first, then `f` to the result.

## Learning Outcomes

- Understanding function composition as a higher-order function pattern
- How Rust closures capture their environment and enable function composition
- Type inference for generic function composition with Rust traits
- The difference between trait objects, function pointers, and closures

## OCaml Approach

In OCaml, functions are first-class values. `compose f g` creates a new function by simply wrapping `f (g x)` in a closure. OCaml's type system infers the composition automatically.

```ocaml
let compose f g x = f (g x)
```

## Rust Approach

Rust uses higher-rank trait bounds to express composition. The `compose` function is generic over the function types `F` and `G`, and returns an `impl Fn` that captures both functions. This provides zero-cost abstraction—no runtime overhead.

```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}
```

## Key Differences

1. **Type Parameters:** OCaml infers everything; Rust requires explicit type parameters `A, B, C` for the domain and codomain.
2. **Trait Bounds:** Rust requires `F: Fn(B) -> C` and `G: Fn(A) -> B` to express that `f` and `g` are callable with specific signatures.
3. **Return Type:** OCaml returns a value; Rust returns `impl Fn`, which can be:
   - A closure capturing `f` and `g`
   - A function pointer (less flexible but more concrete)
4. **Lifetime Handling:** Rust's `move` captures are explicit; OCaml's closures capture implicitly.
