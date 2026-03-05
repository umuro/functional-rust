📖 **[View on hightechmind.io →](https://hightechmind.io/rust/586-pattern-multiple-arms)**

---

# 586: Consolidating Match Arms

**Difficulty:** 2  **Level:** Beginner

Collapse variants with the same result into a single arm using `|`.

## The Problem This Solves

A `match` with many variants quickly becomes repetitive if several variants share the same outcome. Writing a separate arm for `Token::Plus`, `Token::Minus`, `Token::Star`, and `Token::Slash` — all returning `"arithmetic"` — duplicates the body and makes the match harder to read and maintain.

OR patterns (`|`) solve this: list all the variants on one arm, separated by `|`, and share a single body. The result is a match that reads like a classification table — groups of related cases mapped to their outcome. Add a new `Token::Percent` variant? One line in the right group, and the compiler ensures exhaustiveness.

Range patterns (`100..=199 =>`) are OR patterns taken to the extreme for numeric types: they match an inclusive range of integer values, enabling HTTP status code routing or score bucketing in a single expression.

## The Intuition

`Token::Plus | Token::Minus | Token::Star | Token::Slash => "arithmetic"` reads like a rule: "if it's any of these, it's arithmetic." The `|` here is the same as in a regular `|` in boolean logic — it's OR. The compiler checks that together all the arms are exhaustive.

Range patterns are syntactic sugar for "any value from X to Y inclusive." They only work on integer and `char` types, and they compose with OR: `400..=499 | 500..=599 => "client or server error"` is valid.

## How It Works in Rust

1. **OR arm** — `Token::Plus | Token::Minus | Token::Star | Token::Slash => "arithmetic"` — any of the listed variants matches this arm.
2. **Multi-line OR arm** — split across lines with `|` at the start for readability; style choice, not required.
3. **Range pattern** — `100..=199 => "informational"` — matches any integer in that inclusive range; only for integers and `char`.
4. **Catch-all** — `_ => -1` catches everything not matched above; with OR arms the catch-all is rarely needed for full exhaustiveness.
5. **Binding in OR arms** — if any variant in an OR arm carries data, *all* variants in that arm must bind the same names with the same types.

## What This Unlocks

- Write classification logic as a readable table: groups of related cases mapped to their outcome.
- Use range patterns for HTTP status codes, score buckets, Unicode ranges — any integer-categorization problem.
- Keep `match` arms maintainable: add new variants to the right group without touching the body.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| OR patterns | `Plus \| Minus \| Star -> "arithmetic"` — identical | `Plus \| Minus \| Star => "arithmetic"` |
| Range patterns | `n when n >= 100 && n <= 199 -> ...` (guard) | `100..=199 => ...` — dedicated range pattern syntax |
| Exhaustiveness with OR | Compiler checks; missing case is a warning | Compile error; must cover all variants |
| Data binding in OR arms | All variants must bind same names | Same requirement |
