📖 **[View on hightechmind.io →](https://hightechmind.io/rust/237-day-convolution)**

---

# Day Convolution
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Day convolution is a monoidal product for functors: given two applicative functors `F` and `G`, `Day<F, G>` is their "tensor product." The construction packages values from both functors with a combining function, enabling them to be applied together. Day convolution arises in effect systems (combining independent effect types), parser combinators (running two parsers on the same input), and signal processing abstractions. It is the categorical generalization of `zip` for applicatives.

## Learning Outcomes

- Understand Day convolution as the monoidal product for functors/applicatives
- Learn the structure: `Day<F, G, A> = exists B, C. (F<B>, G<C>, B -> C -> A)`
- See how Day convolution relates to `zip` and parallel applicative composition
- Connect to the applicative functor laws through Day's monoidal structure

## Rust Application

`struct Day<F, G, A> { fb: Box<dyn Any>, gc: Box<dyn Any>, func: Box<dyn Fn(Box<dyn Any>, Box<dyn Any>) -> A> }`. The existential types `B` and `C` are erased via `Box<dyn Any>`. `day(fb, gc, f)` lifts two functors with a combining function. The applicative `ap` operation emerges: `day(f, a, |f, a| f(a))` is the standard `<*>`. Running Day on `(Option<B>, Option<C>)` gives `Option<A>` via `Option::zip` semantics.

## OCaml Approach

OCaml's Day convolution using GADTs:
```ocaml
type ('f, 'g, 'a) day =
  Day : 'b 'f * 'c 'g * ('b -> 'c -> 'a) -> ('f, 'g, 'a) day
let run_option (Day (fb, gc, f)) =
  match (fb, gc) with
  | (Some b, Some c) -> Some (f b c)
  | _ -> None
```
OCaml's GADT hides `'b` and `'c` cleanly. The `run_option` interpreter specializes Day to the `Option` functor.

## Key Differences

1. **Monoidal structure**: Day convolution gives functors a tensor product; the unit of the monoid is the `Identity` functor — this is the categorical structure.
2. **Two existentials**: Day requires two existential types (`B` and `C`); Coyoneda requires one — Day is more complex.
3. **Applications**: Day convolution is used in `adjunctions`, `comonad-transformers`, and `lens` internals; it is primarily a theoretical/library tool.
4. **Practical analogy**: `zip` for lists and `liftA2` for applicatives are the everyday incarnations of Day convolution in programming.

## Exercises

1. Implement `run_vec: Day<Vec, Vec, A> -> Vec<A>` using `zip` semantics.
2. Show that `day(identity, fa, |(), a| a)` is isomorphic to `fa` — the identity is the unit for Day.
3. Implement the applicative `ap: Day<F, F, A> -> F<A>` for `Option<_>`.
