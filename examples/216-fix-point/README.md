📖 **[View on hightechmind.io →](https://hightechmind.io/rust/216-fix-point)**

---

# Fix Point — How Recursive Types Work Under the Hood
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

All recursive data types — lists, trees, expressions — have the same structure: a "base functor" that describes one layer, plus a mechanism for recursion. The Fix point separates these concerns: `Fix<F>` is the type-level fixed point of a functor `F`. `ListF<A>` describes one list node; `Fix<ListF>` is a full list. This abstraction enables writing a single `cata` (fold) function that works for any recursive type — just provide the one-step algebra.

## Learning Outcomes

- Understand the fixed point of a functor: `Fix<F> ≅ F<Fix<F>>`
- Learn how `ListF<A>` and `TreeF<A>` are base functors for lists and trees
- See how `Fix<F>` builds full recursive structures from non-recursive base functors
- Understand why this abstraction enables the recursion scheme patterns (examples 217-225)

## Rust Application

`enum ListF<A> { NilF, ConsF(i64, A) }` is the base functor — one layer of a list with children of type `A`. `struct FixList(Box<ListF<FixList>>)` is the fixed point — children are themselves `FixList`. `unfold_list` builds a `FixList` from a seed using a coalgebra. `fold_list` reduces a `FixList` using an algebra. The pattern generalizes: `ExprF<A>` is the expression base functor, `FixExpr` is the expression fixed point.

## OCaml Approach

OCaml's fixed point pattern:
```ocaml
type 'a list_f = NilF | ConsF of int * 'a
type fix_list = Fix of fix_list list_f
let fold f (Fix lf) = f (List.map fold lf)  (* simplified *)
```
OCaml's `let rec` allows directly recursive types without explicit `Box`, making the fix-point pattern more natural. The `Fix` wrapper is still needed to create the fixed point, but the recursion is implicit.

## Key Differences

1. **Box requirement**: Rust needs `Box<ListF<FixList>>` to break the recursive size cycle; OCaml's GC-managed values don't need this.
2. **Functor map**: Both require implementing `map` for the base functor to enable generic recursion schemes; this is the only per-type requirement.
3. **Practical use**: The fix-point pattern is primarily educational/library-design territory; `recursion-schemes` crate in Rust and OCaml provide production implementations.
4. **Performance**: Each layer of a `Fix`-based structure allocates one `Box`; this is worse than native recursive types for performance-critical code.

## Exercises

1. Implement `ExprF<A> { LitF(i64), AddF(A, A), MulF(A, A) }` as a base functor with `map`.
2. Build the corresponding `FixExpr` type and create the expression `(2 + 3) * 4`.
3. Implement a `size` function on `FixList` that counts elements using the fold pattern.
