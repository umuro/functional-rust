📖 **[View on hightechmind.io →](https://hightechmind.io/rust/241-comonad-basics)**

---

# Comonad Basics
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A comonad is the categorical dual of a monad. Where a monad has `pure: A -> M<A>` (inject a value) and `bind: (A -> M<B>) -> M<A> -> M<B>` (sequence effects), a comonad has `extract: W<A> -> A` (extract the focused value) and `extend: (W<A> -> B) -> W<A> -> W<B>` (apply a context-dependent function to every position). Comonads model computations that consume context: cellular automata, image processing, signal filtering, and zipper-based navigation.

## Learning Outcomes

- Understand the comonad as the dual of a monad: extract vs. pure, extend vs. bind
- Learn the three comonad operations: `extract`, `duplicate`, and `extend`
- See the `NonEmptyVec` (with a focused position) as a concrete comonad
- Connect comonads to cellular automata, spreadsheets, and context-dependent computations

## Rust Application

`trait Comonad { fn extract(&self) -> A; fn extend<B>(self, f: impl Fn(&Self) -> B) -> Self::W<B>; }`. A `Focused<T>` (a non-empty vec with a cursor) implements `Comonad`: `extract` returns the element at the cursor; `extend(f)` creates a new `Focused` where each position holds `f` applied to the original `Focused` focused at that position. This is how context-dependent transformations work: each output element depends on the neighborhood of the corresponding input.

## OCaml Approach

OCaml's comonad:
```ocaml
module type COMONAD = sig
  type 'a t
  val extract : 'a t -> 'a
  val extend : ('a t -> 'b) -> 'a t -> 'b t
end
module ZipperComonad : COMONAD with type 'a t = 'a zipper = struct
  let extract z = z.focus
  let extend f z = map (fun z' -> f z') (all_positions z)
end
```
The zipper comonad is the standard OCaml comonad example. Comonads appear in OCaml in data-flow programming and attribute grammars.

## Key Differences

1. **Extract vs. pure**: `extract` always succeeds (comonads are "full" containers with at least one element); `pure` always injects (monads can be empty).
2. **Contextual semantics**: `extend f w` computes `f` at every position of `w`, using the whole `w` as context for each position — this is the "cellular automaton" pattern.
3. **Dual laws**: Comonad laws are the dual of monad laws; `extract (extend f w) = f w` mirrors `bind return = id`.
4. **Practical use**: `extend` is used for image convolution (each pixel computed from a neighborhood), sliding window statistics, and breadcrumb navigation.

## Exercises

1. Implement `duplicate: W<A> -> W<W<A>>` in terms of `extend` and verify it gives all focused sub-contexts.
2. Write a cellular automaton step using `extend` on `Focused<u8>`: each cell's next state depends on itself and its two neighbors.
3. Implement a moving average using `extend`: each position's value is the average of the 3 surrounding positions.
