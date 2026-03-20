📖 **[View on hightechmind.io →](https://hightechmind.io/rust/850-functors-intro)**

---

# Functors Introduction
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A functor is a type that supports mapping a function over its contents while preserving structure. `Option::map`, `Result::map`, `Vec::iter().map()`, `Iterator::map()` are all functors in Rust. The functor pattern abstracts "apply a transformation inside a container" independently of the container type. This enables writing generic algorithms that work over any functor: parse, transform, validate — all without knowing whether the value is present, absent, or in a list. Functors are the foundation of the map → filter → fold pipeline central to functional programming and the Rust iterator protocol.

## Learning Outcomes

- Define a `Functor` trait: `fn map<B, F: Fn(A) -> B>(self, f: F) -> Self<B>` (approximated in Rust)
- Implement `map` for a custom `Maybe<T>` type mirroring `Option<T>`
- Verify functor laws: identity (`map(id) == id`) and composition (`map(f∘g) == map(f)∘map(g)`)
- Recognize how Rust's `Option::map`, `Result::map`, and `Iterator::map` implement the functor concept
- Understand why Rust's type system cannot express the generic `Functor<F>` trait due to HKT limitations

## Rust Application

```rust
#[derive(Debug, PartialEq, Clone)]
enum Maybe<T> { Nothing, Just(T) }

impl<T> Maybe<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,
            Maybe::Just(x) => Maybe::Just(f(x)),
        }
    }
}
```

The `map` consumes `self` (taking ownership) and returns `Maybe<U>` — a different type. `FnOnce` is the weakest closure bound, accepting any callable. The pattern match handles both variants: `Nothing` maps to `Nothing`, `Just(x)` applies `f` to `x` and wraps the result. Rust cannot express the higher-kinded type abstraction `Functor<F<_>>` in stable code; each type implements `map` independently rather than through a shared trait.

## OCaml Approach

OCaml represents Maybe as `type 'a maybe = Nothing | Just of 'a`. The `map` function is `let map f = function Nothing -> Nothing | Just x -> Just (f x)`. OCaml's module system allows a `Functor` signature: `module type FUNCTOR = sig type 'a t; val map : ('a -> 'b) -> 'a t -> 'b t end`. This higher-kinded abstraction works in OCaml via parameterized modules. The `Option.map` in stdlib has the same signature. Law verification: `map Fun.id x = x` and `map (f |> g) x = map g (map f x)`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Generic functor trait | Not expressible (no HKT) | `FUNCTOR` module signature |
| Per-type map | `impl<T> Maybe<T> { fn map }` | `let map f = function ...` |
| Law verification | Property tests | Same; or module functors |
| Identity law | `x.map(|v| v) == x` | `map Fun.id x = x` |
| Composition law | `x.map(|v| g(f(v)))` | `map (f >> g) x = map g (map f x)` |
| stdlib | `Option::map`, `Result::map` | `Option.map`, `List.map` |

## Exercises

1. Implement `map` for a custom `Tree<T>` type that applies the function to every leaf value.
2. Verify the functor identity law by writing a test: `tree.map(|x| x)` should equal `tree` for any tree.
3. Verify the composition law: `tree.map(f).map(g)` should equal `tree.map(|x| g(f(x)))`.
4. Implement a generic function that works over any type with a `map` method using a trait bound.
5. Demonstrate that Rust's `Iterator::map` satisfies the functor laws for lazy sequences.
