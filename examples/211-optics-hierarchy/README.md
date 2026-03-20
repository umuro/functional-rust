📖 **[View on hightechmind.io →](https://hightechmind.io/rust/211-optics-hierarchy)**

---

# Optics Hierarchy
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Iso, Lens, Prism, AffineTraversal, and Traversal form a hierarchy: every Iso is a Lens and a Prism; every Lens and Prism is an AffineTraversal; every AffineTraversal is a Traversal. This hierarchy means you can write generic algorithms at the most general level (Traversal) that automatically work for any more specific optic. Understanding the hierarchy prevents reimplementing the same operation for each optic type.

## Learning Outcomes

- Understand the full optics hierarchy: Iso ⊂ Lens ⊂ AffineTraversal ⊂ Traversal, Iso ⊂ Prism ⊂ AffineTraversal
- Learn how upcasting (narrowing capabilities) works in the hierarchy
- See why generic algorithms written at the Traversal level work for any optic
- Appreciate the hierarchy as a mathematical lattice of abstraction levels

## Rust Application

The hierarchy in Rust uses explicit upcast methods: `Lens::as_traversal(&self) -> Traversal` creates a traversal that focuses on exactly one element. `Prism::as_traversal(&self) -> Traversal` creates a traversal that focuses on zero or one element. Generic functions take `Traversal` parameters, and callers pass lenses or prisms via upcasting. The hierarchy is encoded via `From`/`Into` trait implementations or explicit conversion methods.

## OCaml Approach

OCaml's `optics` library uses module types to express the hierarchy:
```ocaml
module type TRAVERSAL = sig ... end
module type LENS = sig ... include TRAVERSAL ... end
module type ISO = sig ... include LENS ... include PRISM ... end
```
Module inclusion mirrors the subtyping relationship. Haskell's profunctor optics represent the hierarchy via type class constraints — a `Lens` is any `Strong + Profunctor`, a `Prism` is any `Choice + Profunctor`, etc.

## Key Differences

1. **Subtyping model**: OCaml uses module inclusion; Rust uses explicit conversion (`as_traversal`) — neither has native covariant subtyping for struct types.
2. **Profunctor encoding**: Haskell's Van Laarhoven (example 212) and profunctor encodings represent the hierarchy implicitly via type class bounds; Rust and OCaml use explicit struct types.
3. **Composition safety**: The hierarchy ensures composed optics are always well-typed — composing a lens and a traversal gives a traversal, never an invalid optic.
4. **Generic functions**: Functions parameterized by `T: AsTraversal` work for all optic types; this requires trait bounds in Rust and module type constraints in OCaml.

## Exercises

1. Implement `optic_sum<O: AsTraversal<S, f64>>(optic: &O, s: &S) -> f64` that works for lenses, prisms, and traversals.
2. Write `optic_count<O: AsTraversal>(optic: &O, s: &S) -> usize` that counts the number of focused elements.
3. Verify the hierarchy composition rules: `lens_then_prism` produces an affine traversal, `lens_then_traversal` produces a traversal.
