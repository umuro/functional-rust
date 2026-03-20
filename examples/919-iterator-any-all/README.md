📖 **[View on hightechmind.io →](https://hightechmind.io/rust/919-iterator-any-all)**

---

# 919-iterator-any-all — Iterator any and all

## Problem Statement

Existential and universal quantification over sequences — "does any element satisfy X?" and "do all elements satisfy X?" — are fundamental logic operations. In formal logic, ∃ (exists) and ∀ (for all). In SQL, `EXISTS` and `NOT EXISTS`. In Haskell, `any` and `all`. In OCaml, `List.exists` and `List.for_all`. Rust's `Iterator::any()` and `Iterator::all()` short-circuit: `any` stops at the first true, `all` stops at the first false. This makes them efficient even on large sequences and safe on infinite iterators (as long as the condition is eventually met).

## Learning Outcomes

- Use `.any(pred)` for existential checking (∃ — short-circuits on first true)
- Use `.all(pred)` for universal checking (∀ — short-circuits on first false)
- Understand vacuous truth: `.all()` on an empty iterator returns `true`
- Understand vacuous false: `.any()` on an empty iterator returns `false`
- Compare with OCaml's `List.exists` and `List.for_all`

## Rust Application

Tests demonstrate: `[1,2,3].iter().any(|&x| x == 2)` = `true`. `[1,2,3].iter().any(|&x| x == 9)` = `false`. `[2,4,6].iter().all(|&x| x % 2 == 0)` = `true`. `[1,2,3].iter().all(|&x| x % 2 == 0)` = `false`. Vacuous truth: `[].iter().all(|_| false)` = `true` (no elements to violate the condition). Vacuous false: `[].iter().any(|_| true)` = `false` (no elements to satisfy the condition). Both are standard logical behaviors following classical logic.

## OCaml Approach

`List.exists: ('a -> bool) -> 'a list -> bool` — equivalent to `any`. `List.for_all: ('a -> bool) -> 'a list -> bool` — equivalent to `all`. Both short-circuit. Vacuous behavior: `List.for_all pred []` = `true`; `List.exists pred []` = `false`. For sequences: `Seq.exists` and `Seq.for_all` (since 4.14). The semantics are identical to Rust's; the difference is method vs function call syntax.

## Key Differences

1. **Identical semantics**: `any`/`all` in Rust and `exists`/`for_all` in OCaml have exactly the same short-circuit behavior and vacuous truth/false rules.
2. **Syntax**: Rust uses method syntax `iter.any(pred)`; OCaml uses function syntax `List.exists pred xs`.
3. **Short-circuit on infinite**: Rust `(0..).any(|x| x > 100)` terminates; `(0..).all(|x| x < 100)` would loop forever — same constraint applies to OCaml `Seq`.
4. **Negation**: `!any(pred)` = `all(!pred)` by De Morgan's law — both languages implement this complementary relationship.

## Exercises

1. Use `.any()` and `.all()` to implement `is_valid_sudoku_row(row: &[u8]) -> bool` that checks for values 1-9 with no duplicates.
2. Write `has_balanced_parens(s: &str) -> bool` using `.scan()` on chars and `.all()` on the running count.
3. Implement `exactly_one<T>(data: &[T], pred: impl Fn(&T) -> bool) -> bool` that returns true if exactly one element satisfies the predicate.
