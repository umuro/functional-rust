📖 **[View on hightechmind.io →](https://hightechmind.io/rust/202-lens-basics)**

---

# Lens Basics — Get and Set
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

A lens is formally a pair of functions: `get: S -> A` (extract a field of type `A` from a structure `S`) and `set: A -> S -> S` (return a new `S` with the field replaced). This simple representation enables all lens operations: `view` (= `get`), `set`, `over` (apply function to field), and composition. Understanding the basic `Lens<S, A>` struct and its derived operations is the prerequisite for all optics concepts that follow.

## Learning Outcomes

- Implement a `Lens<S, A>` struct with `get` and `set` functions
- Derive `view`, `set`, and `over` from the basic lens definition
- Create lenses for specific record fields by providing concrete `get`/`set` pairs
- Understand that a lens must satisfy three laws (covered in detail in example 203)

## Rust Application

`struct Lens<S, A> { get: Box<dyn Fn(&S) -> A>, set: Box<dyn Fn(A, &S) -> S> }`. `view(&lens, s)` calls `(lens.get)(s)`. `set_val(&lens, a, s)` calls `(lens.set)(a, s)`. `over(&lens, f, s) = lens.set(f(lens.get(s)), s)` — extract, transform, and put back. Lenses for `Point.x` and `Point.y` are constructed by providing the field getter and a `set` that clones `Point` with the new value.

## OCaml Approach

OCaml lenses are typically records:
```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}
let view l s = l.get s
let set l a s = l.set a s
let over l f s = l.set (f (l.get s)) s
```
OCaml's record syntax makes the definition clean. Haskell's `lens` package uses a different (Van Laarhoven) encoding for better composition, but the record encoding is clearer for learning.

## Key Differences

1. **Struct vs. record**: OCaml uses records; Rust uses structs — structurally identical, syntactically different.
2. **Box<dyn Fn> overhead**: Rust's `Box<dyn Fn>` adds heap allocation; OCaml's function fields are heap-allocated via GC — equivalent overhead.
3. **Derive macros**: Production Rust code uses `#[derive(Lens)]` from the `lens-rs` crate; OCaml's `ppx_lens` generates the same.
4. **Immutable update**: Both `set` functions return a new structure without modifying the original — pure functional update.

## Exercises

1. Create lenses for `Circle { center: Point, radius: f64 }` — one for `center` and one for `radius`.
2. Implement `lens_compose(outer: Lens<A, B>, inner: Lens<B, C>) -> Lens<A, C>` that composes two lenses.
3. Write `view_opt: Lens<S, Option<A>> -> S -> Option<A>` that handles optional fields.
