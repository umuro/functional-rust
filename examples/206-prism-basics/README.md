📖 **[View on hightechmind.io →](https://hightechmind.io/rust/206-prism-basics)**

---

# Prism Basics — Optics for Enum Variants
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Lenses focus on product types (struct fields that always exist). Prisms focus on sum types (enum variants that may or may not be present). A prism for the `Err` variant of `Result<T, E>` lets you view the error (if present) and construct a `Result::Err` from an error value. Prisms are the dual of lenses in the optics hierarchy: lenses are for "always there" fields, prisms are for "maybe there" variants.

## Learning Outcomes

- Understand the `Prism<S, A>` type with `preview` (partial get) and `review` (construction)
- Learn when prisms are appropriate: enum variants, optional fields, filtered views
- See the duality with lenses: prism's `preview` returns `Option<A>` where lens's `get` returns `A`
- Implement prisms for `Option` and `Result` variants

## Rust Application

`Prism<S, A>` has two operations: `preview: S -> Option<A>` (extract the inner value if this variant is present) and `review: A -> S` (construct the outer type from the inner value). The `over` operation on prisms: if `preview` succeeds, apply the function; otherwise return the original value unchanged. Example: `ok_prism.preview(Ok(42)) == Some(42)`, `ok_prism.preview(Err("e")) == None`, `ok_prism.review(42) == Ok(42)`.

## OCaml Approach

OCaml's prisms are natural with GADTs and pattern matching:
```ocaml
type ('s, 'a) prism = {
  preview : 's -> 'a option;
  review : 'a -> 's;
}
let ok_prism = {
  preview = (function Ok x -> Some x | Err _ -> None);
  review = (fun x -> Ok x);
}
```
OCaml's pattern matching directly expresses the "which variant is this" logic. The `ppx_fields_conv` annotation can generate prisms for variant types automatically.

## Key Differences

1. **Partial get**: Prism's `preview` returns `Option<A>`; lens's `get` always returns `A` — the key structural difference.
2. **Dual construction**: Prism's `review` constructs the outer type; there is no dual in lenses (you cannot construct a struct from just one field value).
3. **Over semantics**: Lens `over` always modifies; prism `over` is a no-op when the variant is absent — it is "if this variant, then modify, else pass through."
4. **Composition**: Prisms compose with other prisms and with lenses/traversals in the optics hierarchy (example 211).

## Exercises

1. Implement a `some_prism: Prism<Option<A>, A>` and verify `preview(None) = None`, `preview(Some(5)) = Some(5)`.
2. Write `modify_ok<T, E>(f: impl Fn(T) -> T, result: Result<T, E>) -> Result<T, E>` using the `ok_prism`.
3. Build a prism for a specific enum variant: `Circle` of `Shape { Circle(f64), Rect(f64, f64) }`.
