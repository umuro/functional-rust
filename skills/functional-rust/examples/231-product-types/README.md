# 231: Product Types

**Difficulty:** ⭐⭐  **Level:** Type System / Functional Patterns

A product type bundles ALL its fields together simultaneously — it is the categorical product of types, and every struct or tuple in Rust is one.

## The Problem This Solves

Data modeling in programming often comes down to two questions: does a value contain ALL of these things at once, or EXACTLY ONE of these things? Product types answer the first question. They're the data structure for "I need both A and B" — a `Point2d` needs both `x` and `y`, a `Config` needs both `host` and `port`, a `KeyValue` needs both key and value.

Understanding product types categorically unlocks the `curry`/`uncurry` isomorphism: a function `(A, B) -> C` is provably equivalent to `A -> (B -> C)`. This is why currying is a lossless transformation — not just a convention, but a theorem. It also explains why `struct` and tuples feel interchangeable for computation, even though one is named and one is positional.

## The Intuition

A **product type** contains ALL of its components simultaneously. A tuple `(i32, String)` always has both the `i32` and the `String`. A struct `Point2d { x: f64, y: f64 }` always has both `x` and `y`. You can't have a `Point2d` without a `y`.

The categorical view: the *product* of types `A` and `B` is the type `(A, B)` equipped with two *projections* `fst: (A, B) -> A` and `snd: (A, B) -> B`. It satisfies a *universal property*: for any type `X` with functions `f: X -> A` and `g: X -> B`, there's a unique `h: X -> (A, B)` such that `fst(h(x)) = f(x)` and `snd(h(x)) = g(x)`. In code: `h(x) = (f(x), g(x))`.

The **curry/uncurry isomorphism** says: `(A × B → C) ≅ (A → B → C)`. These are two ways to write the same function. `uncurry` collapses the two-argument form to tuple form; `curry` splits the tuple form into the nested function form. They are inverses of each other.

Why does this matter? Curried functions compose and partial-apply cleanly. Uncurried functions work naturally with pairs. Both representations are equally expressive — choose whichever fits the context.

## How It Works in Rust

```rust
// Product type: both fields always present
#[derive(Debug, Clone, Copy)]
pub struct Point2d { pub x: f64, pub y: f64 }

// Projections: extract components from the product
pub fn fst<A, B>(pair: (A, B)) -> A { pair.0 }
pub fn snd<A, B>(pair: (A, B)) -> B { pair.1 }

// Universal property: construct a product from two functions
pub fn pair_of<X: Clone, A, B>(
    f: impl Fn(X) -> A,
    g: impl Fn(X) -> B,
) -> impl Fn(X) -> (A, B) {
    move |x| (f(x.clone()), g(x))  // applies both f and g, bundles results
}

// Uncurry: (A, B) -> C  from  A -> B -> C
pub fn uncurry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn((A, B)) -> C {
    move |(a, b)| f(a, b)
}

// Curry: A -> (B -> C)  from  (A, B) -> C
// Requires Rc because the inner closure needs to share `f`
pub fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn((A, B)) -> C + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> C> {
    let f = Rc::new(f);
    move |a: A| {
        let f = Rc::clone(&f);
        Box::new(move |b: B| f((a.clone(), b)))
    }
}
```

Example: `pair_of` constructs a product from two projections
```rust
// Given any X, compute both f(x) and g(x) and bundle them
let fanout = pair_of(|x: i32| x * 2, |x: i32| x + 1);
assert_eq!(fanout(5), (10, 6));  // both computed from same input

// Curry/uncurry round-trip
let add = |a: i32, b: i32| a + b;
let uncurried = uncurry(add);
assert_eq!(uncurried((3, 4)), 7);
```

## What This Unlocks

- **Curry/uncurry for free** — any two-argument function can switch between curried and uncurried form. Partial application follows directly from the curried form.
- **Data modeling clarity** — recognising that `struct` fields are a categorical product tells you immediately that all fields must co-exist, ruling out nonsensical "partially constructed" states.
- **Dual to sum types** — every struct is a product; every enum is a sum (coproduct). Together they cover all data. Understanding one helps you understand the other.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Named product | `type t = { x: float; y: float }` | `struct Point2d { x: f64, y: f64 }` |
| Tuple | `(a, b)` — persistent pair | `(A, B)` — moved on use (unless `Copy`) |
| `fst`/`snd` | Free projection | Consumes tuple unless `Copy` |
| Curry | Automatic (all OCaml fns are curried) | Explicit closure + `Rc` for sharing |
| Method syntax | Free functions in module | `impl Point2d { fn method(&self) }` |
