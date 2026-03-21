📖 **[View on hightechmind.io →](https://hightechmind.io/rust/858-kleisli-composition)**

---

# Kleisli Composition
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Function composition `g ∘ f` works for plain functions, but fails for monadic functions `f: A -> Option<B>` and `g: B -> Option<C>` — you can't compose them directly because `g` expects `B`, not `Option<B>`. Kleisli composition solves this: `f >=> g` is a function `A -> Option<C>` that applies `f`, then if `Some(b)`, applies `g(b)`. This enables building pipelines of fallible functions as composable building blocks. Kleisli arrows are the category-theoretic way to think about monadic computation: instead of the Kleisli category being hard to understand, think of it as "composition that handles failure automatically." Used in parser combinators, middleware chains, and validation pipelines.

## Learning Outcomes

- Understand the Kleisli arrow: `A -> M<B>` where M is a monad (Option, Result, etc.)
- Implement `kleisli_compose(f, g)` returning a new function `A -> Option<C>`
- Recognize `f >=> g = |a| f(a).and_then(g)` — the definition
- Verify that Kleisli composition satisfies the monad laws (it forms a category)
- Apply to: pipeline building, middleware composition, parser combinator sequences

## Rust Application

```rust
pub fn kleisli<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}
// Usage: compose two fallible lookup functions
let find_user = |id: u32| -> Option<String> { ... };
let find_email = |name: &str| -> Option<String> { ... };
let find_email_by_id = kleisli(find_user, |name| find_email(&name));
```

The `kleisli` function takes two Kleisli arrows and returns their composition. `move |a| f(a).and_then(|b| g(b))` captures both `f` and `g` in the closure. The `impl Fn(A) -> Option<C>` return type avoids boxing at the cost of opaque types (can't name the return type). For multiple compositions, a combinator like `compose_all` builds a chain. The associativity monad law guarantees that `kleisli(kleisli(f, g), h) == kleisli(f, kleisli(g, h))`.

## OCaml Approach

OCaml's Kleisli composition: `let ( >=> ) f g = fun a -> f a >>= g`. The operator `>=>` is the "fish operator" in Haskell. `let find_email_by_id = find_user >=> find_email`. OCaml's partial application makes this natural: `let composed = f >=> g >=> h` chains three fallible functions. The associativity of `>=>` follows from the monad associativity law. OCaml's `Fun.compose` is for plain functions; `>=>` is for Kleisli arrows.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Composition | `kleisli(f, g)` function | `f >=> g` operator |
| Capture | `move` closure | Implicit capture |
| Return type | `impl Fn(A) -> Option<C>` | `'a -> 'c option` |
| Multiple compose | Manual nesting | `f >=> g >=> h` |
| Associativity | Follows from monad laws | Same |
| Result variant | Same pattern with `Result` | Same |

## Exercises

1. Implement `kleisli` for `Result<T, E>` and chain three validation functions.
2. Verify that Kleisli composition is associative: `kleisli(kleisli(f, g), h)(a) == kleisli(f, kleisli(g, h))(a)`.
3. Implement `kleisli_id` — the identity Kleisli arrow — and verify `kleisli(kleisli_id, f) == f`.
4. Build a middleware pipeline using Kleisli composition: each middleware transforms a request `Option<Request>` → `Option<Response>`.
5. Implement a parser combinator sequence using Kleisli composition: `parse_header >=> parse_body >=> parse_footer`.
