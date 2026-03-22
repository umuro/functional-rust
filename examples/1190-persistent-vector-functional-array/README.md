# Example 1190: Persistent Vector — Functional Array

**Difficulty:** ⭐⭐⭐
**Category:** Persistent Data Structures | Trees | Functional Patterns
**OCaml Source:** Classic functional programming — persistent array via balanced binary tree

## Problem Statement

Implement a persistent (immutable) vector backed by a balanced binary tree. `get` reads by index; `set` returns a new version with one element replaced while the original remains unchanged.

## Learning Outcomes

- How structural sharing with `Rc` enables persistent data structures without full copies
- Why `Option<T>` is idiomatic over `panic!`/`failwith` for out-of-bounds access
- How slice patterns (`[]`, `[x]`, `_`) replace OCaml list/variant pattern matching
- The difference between `Box` (exclusive ownership, deep clone on update) and `Rc` (shared ownership, O(log n) update cost)

## OCaml Approach

OCaml uses an algebraic type `'a pvec = Nil | One of 'a | Two of 'a pvec * 'a pvec`. Since OCaml values are immutable by default, `set` naturally returns a new tree sharing the unchanged subtree — the GC handles lifetime. `failwith` signals out-of-bounds errors.

## Rust Approach

Rust uses an enum matching OCaml's ADT. For the idiomatic version, `Rc<PVec<T>>` provides reference-counted shared ownership so unchanged subtrees are genuinely shared (not copied) between versions. For the recursive version, `Box<PVecRec<T>>` gives simple tree ownership but requires cloning unchanged subtrees on each `set`. Both use `Option<T>` instead of panics.

## Key Differences

1. **Memory management:** OCaml GC automatically shares subtrees; Rust uses `Rc` for explicit reference-counted sharing or `Box` for exclusive ownership with cloning.
2. **Error handling:** OCaml `failwith "index"` raises an exception; Rust returns `Option<T>` — the caller decides how to handle out-of-bounds.
3. **Type parameters:** OCaml `'a` is implicitly polymorphic; Rust requires explicit `<T>` with trait bounds (`T: Clone` for `from_slice`).
4. **Pattern matching on slices:** OCaml matches on list constructors `[] | [x] | _`; Rust uses slice patterns `[] | [x] | _` on `&[T]` — same shape, different memory model.
