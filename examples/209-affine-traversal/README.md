📖 **[View on hightechmind.io →](https://hightechmind.io/rust/209-affine-traversal)**

---

# Affine Traversal — At Most One Focus
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Between a lens (exactly one focus) and a traversal (zero or more focuses) lies the affine traversal (at most one focus). It is the combination of a prism (might not exist) and a lens (focused access). `HashMap::get` is an affine traversal: it focuses on the value at a key if it exists, with no-op updates when the key is absent. Optional record fields and possibly-missing array elements are natural affine traversals.

## Learning Outcomes

- Understand affine traversals as the intersection of prisms and lenses
- Implement `preview` (extract if present) and `over` (modify if present) for affine traversals
- See `HashMap` lookups as canonical examples of affine traversal
- Understand where affine traversals fit in the optics hierarchy (between prism and traversal)

## Rust Application

`AffineTraversal<S, A>` has `preview: &S -> Option<A>` and `set: A -> &S -> S`. When `preview` returns `None`, `set` and `over` are no-ops — the structure is returned unchanged. For `HashMap<K, V>`, the traversal focused on key `k`: `preview` = `map.get(k).cloned()`, `set(v, m)` = clone the map with `m[k] = v`. This is equivalent to `Option`-returning lens — the most practical optic for map-like structures.

## OCaml Approach

OCaml's affine traversal is called an "optional" in the optics literature. The `optics` library provides `Optional.t` with `getOption` and `set` operations. OCaml's `Map.find_opt : key -> 'a Map.t -> 'a option` is the standard affine traversal for ordered maps. The composition of a lens into an `option` field produces an affine traversal automatically.

## Key Differences

1. **Naming**: Rust and Haskell call it "affine traversal"; OCaml's optics library calls it "optional"; the semantics are the same.
2. **HashMap focus**: Rust's `HashMap` operations naturally implement the affine traversal interface; OCaml's `Map.S.find_opt` similarly provides affine access.
3. **Composition**: Composing a lens with an affine traversal gives an affine traversal; composing two affine traversals gives an affine traversal — the class is closed under composition.
4. **Practical use**: Affine traversals are the most practically used optic after lenses — optional field access is ubiquitous.

## Exercises

1. Implement an affine traversal for `Option<T>` (same as a prism for `Some` but with `set` updating the inner value).
2. Write `modify_at_key<K, V>(key: K, f: impl Fn(V) -> V, map: HashMap<K, V>) -> HashMap<K, V>`.
3. Compose a lens (into a struct field containing a `HashMap`) with the `at_key` affine traversal.
