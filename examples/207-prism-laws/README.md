📖 **[View on hightechmind.io →](https://hightechmind.io/rust/207-prism-laws)**

---

# Prism Laws
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A prism that violates its consistency laws — where `review` followed by `preview` does not give back the original value, or where `preview` succeeds but `review` of the result does not reconstruct the original — is an unreliable abstraction that breaks compositions silently. The two prism laws (ReviewPreview and PreviewReview) define what it means for a prism to be lawful and trustworthy, just as the three lens laws define a trustworthy lens.

## Learning Outcomes

- Understand the two prism laws: ReviewPreview and PreviewReview
- Learn how prism laws guarantee round-trip consistency between `preview` and `review`
- See examples of lawful and unlawful prisms
- Write tests that verify prism law compliance

## Rust Application

The two prism laws:
1. **ReviewPreview**: `prism.preview(prism.review(a)) == Some(a)` — constructing and then extracting gives back the original.
2. **PreviewReview**: `if prism.preview(s) == Some(a) then prism.review(a) == s` — if extraction succeeds, reconstruction gives back the original.

A lawful `ok_prism` satisfies both: `ok_prism.preview(ok_prism.review(42)) == Some(42)` and if `ok_prism.preview(Ok(42)) == Some(42)`, then `ok_prism.review(42) == Ok(42)`. An unlawful prism that normalizes values on `review` violates law 1.

## OCaml Approach

OCaml's prism laws are expressed identically — mathematical properties. Law-checking in OCaml uses `QCheck` for property-based testing:
```ocaml
QCheck.Test.make QCheck.int (fun a ->
  prism.preview (prism.review a) = Some a)
```
OCaml's pattern matching makes `preview` implementations more naturally lawful than Rust's closure-based approach, reducing the risk of accidentally violating laws.

## Key Differences

1. **Two laws vs. three**: Prisms have two laws; lenses have three — fewer laws because prisms lack the "SetSet" idempotency requirement.
2. **Law strength**: ReviewPreview is the strongest constraint — it forces `review` to be injective (different `a` values produce different `s` values).
3. **Property testing**: Both languages benefit from property-based testing over many random inputs to verify laws; a single test case is insufficient.
4. **Lawful by construction**: Prisms generated from exact variant pattern matches are always lawful; prisms with transformations in `review` are suspect.

## Exercises

1. Verify that the `some_prism` from example 206 is lawful by testing both laws with 100 random inputs.
2. Construct an unlawful prism that normalizes strings on `review` (lowercase) and show which law it violates.
3. Prove mathematically that law 2 implies that `review` is injective (different inputs produce different outputs).
