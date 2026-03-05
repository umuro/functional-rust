📖 **[View on hightechmind.io →](https://hightechmind.io/rust/576-pattern-matches-macro)**

---

# 576: `matches!` Macro

**Difficulty:** 2  **Level:** Beginner

Test a value against a pattern and get a `bool` — without a full `match` expression.

## The Problem This Solves

You often need to answer a simple yes/no question: "is this value `Status::Active`?" or "is this `Ok` with a positive number?" Without `matches!`, the answer requires a full `match` or an `if let` block — several lines for what should be a one-liner. In iterator `filter` chains, this is especially awkward.

`matches!(value, pattern)` collapses this to a boolean expression. It works with any pattern Rust supports: enum variants, OR patterns (`A | B`), guards (`if condition`), and nested destructuring. It's purely syntactic — the compiler expands it to a match with `true` and `false` arms. There's no runtime overhead.

## The Intuition

`matches!(x, Pattern)` is exactly `if let Pattern = x { true } else { false }`, but readable at a glance. It slots into any boolean context: `filter`, `assert!`, `if` conditions, `while` conditions.

OR patterns (`matches!(x, A | B | C)`) test multiple variants in one expression — the boolean equivalent of consolidating match arms.

## How It Works in Rust

1. **Basic use** — `matches!(status, Status::Active)` returns `true` or `false`.
2. **In `filter`** — `.filter(|u| matches!(u, Status::Active | Status::Pending))` — concise, no closure body needed.
3. **With guards** — `matches!(n, x if x % 2 == 0 && x <= 6)` — the guard runs after the pattern matches.
4. **With enum data** — `matches!(s, Shape::Circle(_))` — the `_` wildcard matches any wrapped value.
5. **In `assert!`** — `assert!(matches!(r, Ok(n) if n > 0))` — readable assertions in tests.

## What This Unlocks

- Write filter predicates inline without verbose `if let` blocks.
- Express "is this one of these variants?" as a readable boolean in any context.
- Use guards and destructuring in boolean tests without a full match expression.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern as boolean | No direct equivalent; use `match ... -> bool` | `matches!(value, pattern)` macro — expands to `match` |
| OR patterns in boolean | `match x with A \| B -> true \| _ -> false` | `matches!(x, A \| B)` — same semantics, much shorter |
| Guards in boolean test | `match x with p when cond -> true \| _ -> false` | `matches!(x, p if cond)` |
| In filter predicates | `List.filter (function A -> true \| _ -> false)` | `.filter(\|x\| matches!(x, A))` |
