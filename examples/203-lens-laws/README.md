📖 **[View on hightechmind.io →](https://hightechmind.io/rust/203-lens-laws)**

---

# Lens Laws
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A "lens" that violates basic consistency properties is useless — if `set` does not set or `get` does not get what was set, compositions break silently. Three laws define what it means for a lens to be "lawful": GetSet (setting then getting gives back what you set), SetGet (getting then setting the same value is a no-op), and SetSet (setting twice is the same as setting once). Law-checking tests catch implementation bugs early.

## Learning Outcomes

- Understand the three lens laws: GetSet, SetGet, and SetSet
- Learn how to write property-based tests that verify lens law compliance
- See examples of lawful and unlawful lenses
- Appreciate why laws matter: lawless lenses break when composed

## Rust Application

The three laws as Rust assertions:
1. **GetSet**: `lens.get(&lens.set(a, &s)) == a` — after setting `a`, getting gives back `a`.
2. **SetGet**: `lens.set(lens.get(&s), &s) == s` — setting with the existing value is identity.
3. **SetSet**: `lens.set(b, &lens.set(a, &s)) == lens.set(b, &s)` — second set overwrites first.

A lawful lens for `Point.x` satisfies all three. An example lawless lens (normalizing a value on `get`) violates GetSet: `get` returns the normalized value, but `set` stores the un-normalized value.

## OCaml Approach

OCaml's optics libraries ship with property-based tests using `QCheck`. The laws are phrased identically — they are mathematical properties independent of the programming language. OCaml's `ppx_lens`-generated lenses are always lawful by construction (they directly access record fields). Hand-written lenses require manual law verification.

## Key Differences

1. **Law testing**: Both languages use property-based testing (Rust: `proptest`/`quickcheck`, OCaml: `QCheck`) to verify lens laws over many random inputs.
2. **Derived lenses**: Lenses derived from direct field access always satisfy all three laws; manually constructed lenses (with transformations) may violate them.
3. **Unlawful lenses**: "Lenses" that fail the laws are sometimes useful (e.g., a lens with normalization) but must not be composed with other lenses.
4. **Weaker optics**: Prisms and affine traversals have analogous laws (PreviewReview, ReviewPreview) that all lawful instances must satisfy.

## Exercises

1. Construct an unlawful lens that normalizes a string to lowercase on `get`, and show which law it violates.
2. Add `#[cfg(test)]` property-based tests using `quickcheck` that verify all three laws for the `Point.x` lens.
3. Verify that the composition of two lawful lenses is also lawful by testing the composed lens against all three laws.
