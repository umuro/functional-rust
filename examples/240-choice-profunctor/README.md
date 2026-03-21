📖 **[View on hightechmind.io →](https://hightechmind.io/rust/240-choice-profunctor)**

---

# Choice Profunctor
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

While `Strong` profunctors correspond to lenses (product types, always present), `Choice` profunctors correspond to prisms (sum types, possibly absent). `Choice` adds `left: P<A, B> -> P<Either<A, C>, Either<B, C>>` — the ability to "pass through" the `Right(C)` case while operating on `Left(A) -> Left(B)`. Functions implement `Choice` naturally. A Van Laarhoven prism requires `Choice`: `type Prism s a = ∀p. Choice p => p a b -> p s t`.

## Learning Outcomes

- Understand `Choice` as the profunctor class capturing prism-like behavior
- Learn `left` and `right` as the two `Choice` operations
- See how `Choice` enables "operate on one branch, pass through the other"
- Connect `Choice` to prisms: prisms are polymorphic over all Choice profunctors

## Rust Application

`trait Choice: Profunctor { fn left<A, B, C>(self: P<A, B>) -> P<Either<A, C>, Either<B, C>>; fn right<A, B, C>(self: P<A, B>) -> P<Either<C, A>, Either<C, B>>; }`. Implementing for functions: `left(f) = |e| match e { Left(a) => Left(f(a)), Right(c) => Right(c) }`. A prism `prism_s_a: forall p: Choice, p a b -> p s t` is a choice profunctor morphism — selecting the focused variant is threading through the `Left`/`Right` structure.

## OCaml Approach

OCaml's Choice profunctor:
```ocaml
module type CHOICE = sig
  include PROFUNCTOR
  val left : ('a, 'b) t -> (('a, 'c) Either.t, ('b, 'c) Either.t) t
  val right : ('a, 'b) t -> (('c, 'a) Either.t, ('c, 'b) Either.t) t
end
```
A prism `type ('s, 't, 'a, 'b) prism = { run : 'p. (module CHOICE with type ('a,'b) t = 'p) -> 'p a b -> 'p s t }` requires first-class module polymorphism — OCaml handles this better than Rust.

## Key Differences

1. **Prism encoding**: `Choice` captures prisms just as `Strong` captures lenses; the duality (products vs. sums) maps onto profunctor classes.
2. **Function implementation**: Functions implement both `Strong` and `Choice` — functions can thread both product and sum context; they are simultaneously lenses and prisms (isos) in the profunctor encoding.
3. **Traversal**: `Traversal` corresponds to profunctors that are both `Strong` and `Choice` with certain coherence conditions.
4. **Wander**: More general traversals require a `Wander` class beyond `Choice`; the full hierarchy is `Profunctor ⊂ Strong ⊂ Wander`.

## Exercises

1. Implement `left` for a `Kleisli<Option, A, B>` profunctor — threading `Option` context through choice.
2. Write `prism_via_choice(preview, review)` creating a prism from the Choice profunctor encoding.
3. Verify that `right(f) = dimap(Either::swap, Either::swap)(left(f))` — `right` is derivable from `left`.
