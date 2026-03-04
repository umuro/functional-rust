# 207: Prism Laws — What Makes a Prism Well-Behaved

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

Two round-trip laws guarantee that a Prism's `preview` and `review` are consistent with each other.

## The Problem This Solves

A Prism is just two functions. Nothing stops you from writing a `preview` that transforms the value it returns — say, uppercasing a string before returning it — while `review` doesn't apply the same transformation. The code compiles. Basic tests might pass. But when you use this Prism in composition, the round-trip breaks: you put in `"hello"`, preview gives you `"HELLO"`, review puts back `"HELLO"` — and now your data has silently changed.

Unlike Lens laws (which verify that get and set are inverses of each other), Prism laws verify that **construction and deconstruction are consistent**. `review` builds a value; `preview` should be able to read that same value back out unchanged. If the Prism's two functions aren't compatible, it's not a Prism in any meaningful sense — it's just two unrelated functions wrapped in the same struct.

Laws make Prisms composable and trustworthy. When every Prism satisfies both laws, you can compose them confidently. This example exists to solve exactly that pain.

## The Intuition

A Prism is a relationship between a "big" type `S` (the enum) and a "small" type `A` (the variant's payload). The two laws express that this relationship is **consistent**:

**Law 1 — ReviewPreview**: "If I build an `S` using `review`, I can always get back my `A` using `preview`."
```
preview(review(a)) == Some(a)
```
You put `"hello"` into a `JString` via `review`. `preview` on that `JString` must give back `Some("hello")` — exactly what you put in. Not `Some("HELLO")`. Not `Some("hello ")`. Exactly `"hello"`.

**Law 2 — PreviewReview**: "If `preview` succeeds and gives me `a`, then `review(a)` reconstructs the original `s`."
```
if preview(s) == Some(a)  then  review(a) == s
```
If you look at a `JString("hello")` and preview gives `Some("hello")`, then building from `"hello"` with `review` must give back `JString("hello")` — the exact original.

An unlawful Prism in this file `preview`s with `.to_uppercase()` — it transforms the value during extraction. `review` doesn't uppercase. So `preview(review("hello"))` gives `Some("HELLO")` ≠ `Some("hello")`. Law 1 broken.

## How It Works in Rust

**Lawful Prisms for a `Json` type:**

```rust
fn jstring_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j { Json::JString(s) => Some(s.clone()), _ => None },
        |s| Json::JString(s),
    )
}
```

`preview` extracts the string as-is. `review` wraps it. No transformation — perfectly consistent.

**The unlawful Prism:**

```rust
fn bad_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j { Json::JString(s) => Some(s.to_uppercase()), _ => None },
        |s| Json::JString(s),  // review doesn't uppercase!
    )
}
```

**Law verification functions:**

```rust
// Law 1: preview(review(a)) == Some(a)
fn check_review_preview<S: PartialEq, A: Clone + PartialEq>(
    prism: &Prism<S, A>, a: &A,
) -> bool {
    let s = (prism.review)(a.clone());
    (prism.preview)(&s) == Some(a.clone())
}

// Law 2: if preview(s) == Some(a) then review(a) == s
fn check_preview_review<S: PartialEq + Clone, A: Clone>(
    prism: &Prism<S, A>, s: &S,
) -> bool {
    match (prism.preview)(s) {
        None    => true,                      // preview didn't match — vacuously true
        Some(a) => (prism.review)(a) == *s,  // reconstruct and compare
    }
}
```

**Running the checks:**

```rust
// jstring_prism is lawful
let p = jstring_prism();
assert!(check_review_preview(&p, &"hello".to_string()));
assert!(check_preview_review(&p, &Json::JString("hello".into())));
assert!(check_preview_review(&p, &Json::JNull));  // None case — trivially true

// bad_prism violates both laws
let bp = bad_prism();
assert!(!check_review_preview(&bp, &"hello".to_string()));
// preview(review("hello")) = preview(JString("hello")) = Some("HELLO") ≠ Some("hello")
assert!(!check_preview_review(&bp, &Json::JString("hello".into())));
// preview gives "HELLO", review("HELLO") = JString("HELLO") ≠ JString("hello")
```

## What This Unlocks

- **Trustworthy composition**: two lawful Prisms composed together give a lawful Prism — no audit required at the composition site.
- **Data integrity**: law-satisfying Prisms guarantee no silent data transformation during round-trips — `preview` and `review` are true inverses when the variant matches.
- **Property testing**: both law checkers are generic — drop them into `proptest` or `quickcheck` with random `A` and `S` values for exhaustive coverage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Law 1 name | ReviewPreview | ReviewPreview |
| Law 2 name | PreviewReview | PreviewReview |
| Equality check | Structural `=` on variants | `PartialEq` required on both `S` and `A` |
| Vacuous case (Law 2) | `None -> true` in match | `None => true` — same pattern |
| Unlawful example | `String.uppercase_ascii` in preview | `.to_uppercase()` in preview closure |
| Batch testing | `List.for_all` over test values | `slice.iter().all(…)` or loop |
