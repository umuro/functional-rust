📖 **[View on hightechmind.io →](https://hightechmind.io/rust/238-profunctor-basics)**

---

# Profunctor Basics
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A profunctor is a type constructor `P<A, B>` that is contravariant in `A` and covariant in `B`. Functions `fn(A) -> B` are the canonical profunctor: you can pre-map the input (`A -> A'` gives `P<A', B>`) and post-map the output (`B -> B'` gives `P<A, B'>`). Profunctors generalize both `Functor` (covariant only) and `Contravariant` (contravariant only). They are the mathematical foundation of the profunctor optics encoding (Van Laarhoven generalization).

## Learning Outcomes

- Understand what a profunctor is: covariant in B, contravariant in A
- Learn `dimap: (A' -> A, B -> B') -> P<A, B> -> P<A', B'>`
- See functions `fn(A) -> B` as the canonical profunctor
- Connect profunctors to profunctor optics: lenses, prisms, and traversals via `Strong` and `Choice`

## Rust Application

`trait Profunctor { fn dimap<A, B, C, D>(self, f: impl Fn(C) -> A, g: impl Fn(B) -> D) -> Self::P<C, D>; }`. Implementing for functions: `dimap(f, g)(input) = g((self.func)(f(input)))`. The `lmap` (pre-map only) and `rmap` (post-map only) are derived: `lmap(f) = dimap(f, id)`, `rmap(g) = dimap(id, g)`. `rmap` alone makes functions a `Functor`; `lmap` alone makes them a `Contravariant`; `dimap` unifies both.

## OCaml Approach

OCaml's profunctor:
```ocaml
module type PROFUNCTOR = sig
  type ('a, 'b) t
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end
module FunctionProfunctor : PROFUNCTOR with type ('a, 'b) t = 'a -> 'b = struct
  type ('a, 'b) t = 'a -> 'b
  let dimap f g h = g >> h >> (fun x -> x) >> (fun _ -> failwith "") (* simplified *)
  let dimap f g h x = g (h (f x))
end
```
OCaml's module system naturally expresses the profunctor abstraction. The `lens` and `prism` libraries use profunctors directly.

## Key Differences

1. **Variance**: Profunctors have a two-parameter variance: contravariant in the first (input), covariant in the second (output); Rust's trait system can express this but requires careful bounds.
2. **Profunctor optics**: Lenses are `Strong` profunctors, prisms are `Choice` profunctors — the Van Laarhoven encoding selects the optic type via the profunctor class.
3. **Practical use**: Profunctors appear in `pipes` (streaming), `profunctor-optics` (Haskell), and data flow libraries.
4. **Category theory**: A profunctor `P: C^op × C -> Set` is a functor from the product of a category and its opposite; functions are hom-sets, the canonical profunctor.

## Exercises

1. Implement `lmap` and `rmap` for the function profunctor in terms of `dimap`.
2. Write a `Parser<A, B>` profunctor where `A` is the input type and `B` is the output: `dimap` transforms inputs before parsing and outputs after.
3. Implement `Strong` for the function profunctor: `first: P<A, B> -> P<(A, C), (B, C)>`.
