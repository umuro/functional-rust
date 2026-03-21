📖 **[View on hightechmind.io →](https://hightechmind.io/rust/851-functor-laws)**

---

# Functor Laws
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

A type with a `map` method is not a valid functor unless it satisfies two algebraic laws: identity (`map(id) == id`) and composition (`map(f∘g) == map(f)∘map(g)`). These laws are not enforced by the Rust type system — they're mathematical contracts that ensure `map` truly "preserves structure" and doesn't do hidden processing. Code that depends on functor behavior (optimizations, refactoring, generic algorithms) relies on these laws holding. Understanding laws lets you reason about code algebraically: `Option::map` satisfying the laws means you can freely refactor `x.map(f).map(g)` to `x.map(|v| g(f(v)))` with confidence.

## Learning Outcomes

- State the identity law: `fmap id == id` — mapping the identity function doesn't change the functor
- State the composition law: `fmap (f ∘ g) == fmap f ∘ fmap g` — map distributes over composition
- Write tests that verify both laws for `Maybe`, `Vec`, and a custom `Tree` type
- Understand why law violations indicate incorrect `map` implementations
- Recognize how the laws enable algebraic reasoning and safe refactoring

## Rust Application

```rust
// Identity law: x.map(|v| v) == x
fn identity_law<T: Clone + PartialEq, F: Fn(T) -> T>(x: Maybe<T>) -> bool {
    x.clone().map(|v| v) == x
}
// Composition law: x.map(f).map(g) == x.map(|v| g(f(v)))
fn composition_law<T, U, V>(x: Maybe<T>, f: impl Fn(T)->U, g: impl Fn(U)->V) 
where T: Clone, Maybe<U>: PartialEq, Maybe<V>: PartialEq {
    // x.clone().map(&f).map(&g) == x.map(|v| g(f(v)))
}
```

The identity law test uses `|v| v` as the identity function. The composition law tests that fusing `map(f)` and `map(g)` into a single `map(g∘f)` gives the same result. Both laws hold trivially for `Nothing` (the empty case is unaffected by mapping). For `Just(x)`, the identity law means `f(x)` where f is identity returns x; the composition law means `g(f(x)) == g(f(x))` — both paths apply f then g.

## OCaml Approach

OCaml verifies functor laws with inline tests: `assert (map Fun.id x = x)` for identity and `assert (map (Fun.compose g f) x = (Fun.compose (map g) (map f)) x)` for composition. `Fun.compose` in stdlib composes functions right-to-left. For property-based testing, `QCheck` generates random values and verifies laws automatically. OCaml's module functors provide a formal way to state and check laws: `module CheckFunctor(F: FUNCTOR)(T: ...) = struct ... end`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Identity test | `.map(\|v\| v) == original` | `map Fun.id x = x` |
| Composition test | `.map(f).map(g) == .map(g∘f)` | `map (f >> g) x = (map g) (map f x)` |
| Property testing | `proptest` crate | `QCheck` library |
| Law enforcement | Tests only (not type system) | `module type LAWS` (convention) |
| `id` function | `\|v\| v` closure | `Fun.id` stdlib |
| Composition fn | Manual `\|v\| g(f(v))` | `Fun.compose` |

## Exercises

1. Write property tests using `proptest` to verify both functor laws for `Option<i32>` with random inputs.
2. Construct a "broken" functor that fails the identity law — one where `map(id)` changes the value — to see what violations look like.
3. Verify that `Vec::iter().map(f).map(g)` collected equals `Vec::iter().map(|v| g(f(v)))` collected.
4. Implement a lawful `Functor` trait using associated types and verify implementors satisfy the laws via trait tests.
5. Show that the composition law enables a "map fusion" optimization and benchmark the fused vs. unfused versions.
