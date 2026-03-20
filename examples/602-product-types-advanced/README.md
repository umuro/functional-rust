📖 **[View on hightechmind.io →](https://hightechmind.io/rust/602-product-types-advanced)**

---

# Advanced Product Types

## Problem Statement

Product types (tuples, structs, records) combine multiple values into one. In category theory, a product `A × B` has projections `fst: A×B → A` and `snd: A×B → B`. Beyond tuples, advanced product types include heterogeneous lists (type-level lists of different types), named product types with field accessors, and type-level products enabling generic programming over record shapes. Understanding product types as mathematical objects clarifies why certain operations are natural (projection, bimap) and others require additional constraints.

## Learning Outcomes

- How `Pair<A, B>` models the categorical product with projections `fst` and `snd`
- How `bimap(f, g)` transforms both components independently
- How `curry` and `uncurry` relate functions on pairs to two-argument functions
- How heterogeneous tuples encode type-level lists in Rust with `HNil` and `HCons`
- Where product types appear: configuration structs, multiple return values, type-level programming

## Rust Application

`Pair<A, B>` implements `fst`, `snd`, `bimap`, `map_fst`, `map_snd`. `curry(f: Fn(A, B) -> C) -> impl Fn(A) -> impl Fn(B) -> C` and `uncurry` convert between pair functions and curried functions. `HNil` and `HCons<H, T>` implement heterogeneous lists at the type level — `HCons<i32, HCons<String, HNil>>` is a type-level record.

Key patterns:
- `bimap(f, g)` — transform both components of a product
- `curry`/`uncurry` — isomorphism between `Fn(A, B)` and `Fn(A) -> Fn(B)`
- `HCons<Head, Tail>` — type-level heterogeneous list

## OCaml Approach

```ocaml
type ('a, 'b) pair = { fst: 'a; snd: 'b }
let bimap f g { fst; snd } = { fst = f fst; snd = g snd }
let curry f a b = f { fst = a; snd = b }
let uncurry f { fst; snd } = f fst snd
```

## Key Differences

1. **Tuple syntax**: Rust `(A, B)` built-in tuples; OCaml `'a * 'b` product types — both are built-in.
2. **HList**: Rust's type-level HList via `HCons`/`HNil`; OCaml uses first-class modules or `GADT`-based approaches for type-level heterogeneous lists.
3. **Currying**: OCaml functions are curried by default — `f a b` is natural; Rust functions are uncurried — `curry` is a conversion utility.
4. **Category theory**: The product type satisfies the universal property: any type `C` with functions to `A` and `B` factors through `A × B` — this motivates the API design.

## Exercises

1. **Zip pairs**: Implement `fn zip<A, B>(a: Vec<A>, b: Vec<B>) -> Vec<Pair<A, B>>` and `fn unzip<A, B>(pairs: Vec<Pair<A, B>>) -> (Vec<A>, Vec<B>)`.
2. **Currying**: Write `fn add_curried(a: i32) -> impl Fn(i32) -> i32` as a manually curried addition, then verify `uncurry(add_curried)(Pair(3, 4)) == 7`.
3. **HList length**: Implement a trait `HLen` with `fn len() -> usize` on `HNil` (returns 0) and `HCons<H, T: HLen>` (returns `1 + T::len()`).
