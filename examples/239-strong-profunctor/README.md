📖 **[View on hightechmind.io →](https://hightechmind.io/rust/239-strong-profunctor)**

---

# Strong Profunctor
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A `Strong` profunctor extends a profunctor with `first: P<A, B> -> P<(A, C), (B, C)>` — the ability to "pass along" extra context `C` while the profunctor operates on `A -> B`. Functions are `Strong`: `first(f) = |(a, c)| (f(a), c)`. The significance: in the profunctor optics encoding, **lenses are exactly Strong profunctors**. A Van Laarhoven lens `type Lens s a = ∀p. Strong p => p a b -> p s t` works for any Strong profunctor.

## Learning Outcomes

- Understand `Strong` as the profunctor class that captures lens-like behavior
- Learn `first` and `second` as the two `Strong` operations
- See how functions implement `Strong` by threading context through
- Connect `Strong` to lens encoding: lenses are polymorphic over all Strong profunctors

## Rust Application

`trait Strong: Profunctor { fn first<A, B, C>(self: P<A, B>) -> P<(A, C), (B, C)>; fn second<A, B, C>(self: P<A, B>) -> P<(C, A), (C, B)>; }`. Implementing for functions: `first(f) = |(a, c)| (f(a), c)` — run the function on the first component, pass the second through. A lens `lens_s_a: forall p: Strong, p a b -> p s t` is a strong profunctor morphism — selecting the focus field corresponds to threading context through the profunctor.

## OCaml Approach

OCaml's Strong profunctor:
```ocaml
module type STRONG = sig
  include PROFUNCTOR
  val first : ('a, 'b) t -> ('a * 'c, 'b * 'c) t
  val second : ('a, 'b) t -> ('c * 'a, 'c * 'b) t
end
```
Lenses in the profunctor encoding: `type ('s, 't, 'a, 'b) lens = { run : 'p. (module STRONG with type ('a,'b) t = 'p) -> 'p a b -> 'p s t }`. This requires first-class modules and rank-2 types — OCaml handles this more naturally than Rust.

## Key Differences

1. **Lens encoding**: `Strong` captures exactly what makes lenses work; functions are Strong, making the function profunctor the reference implementation.
2. **`first` vs. `second`**: Both are derivable from each other given `swap`: `second p = dimap swap swap (first p)` — only one needs to be primitive.
3. **Profunctor class hierarchy**: `Strong` extends `Profunctor`; `Choice` extends `Profunctor` for prisms; their intersection is affine traversals.
4. **Type complexity**: The profunctor optics encoding requires rank-2 types for the full elegance; Rust and OCaml approximate it.

## Exercises

1. Implement `Strong` for `Parser<A, B>` (from example 238) — running a parser while threading context through.
2. Write `lens_via_strong(get, set)` that creates a lens from `get`/`set` pair using the Strong profunctor encoding.
3. Verify that `second(f) = dimap(swap, swap)(first(f))` produces the same result as a direct `second` implementation.
