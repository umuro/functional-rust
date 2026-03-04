# 280: Iterator any() and all()

**Difficulty:** 1  **Level:** Beginner

Short-circuit boolean checks across an iterator: `any` stops at the first match, `all` stops at the first non-match.

## The Problem This Solves

You want to know if *any* element in a collection satisfies a condition, or if *all* of them do. Without `any` and `all`, you'd write an early-return loop — iterate, check, break if found, return the accumulated boolean. That's four lines of imperative code expressing a single logical question.

These are the functional programming existential (∃) and universal (∀) quantifiers, directly available on any iterator. They short-circuit: `any` stops consuming elements as soon as it finds a true result; `all` stops as soon as it finds a false result. For large collections where the answer is near the beginning, this is a significant performance difference.

Two surprising but correct edge cases: `any([])` returns `false` (no element satisfies anything — there are no elements) and `all([])` returns `true` (vacuous truth — no element *violates* the predicate). This is mathematically correct and matches OCaml's `List.for_all` and `List.exists`.

## The Intuition

`any` asks "does at least one element satisfy this?" and stops early on the first yes. `all` asks "do all elements satisfy this?" and stops early on the first no.

## How It Works in Rust

```rust
let nums = [2i32, 4, 6, 8, 10];

nums.iter().all(|&x| x % 2 == 0);  // true  — all even
nums.iter().any(|&x| x > 5);        // true  — 6, 8, 10 are > 5
nums.iter().any(|&x| x % 2 != 0);  // false — no odd numbers
nums.iter().all(|&x| x > 0);        // true  — all positive

// Vacuous truth and vacuous false
let empty: Vec<i32> = vec![];
empty.iter().all(|&x| x > 0);  // true  — vacuously: no element violates it
empty.iter().any(|&x| x > 0);  // false — no element satisfies it

// Check if a slice is sorted
let sorted = [1, 2, 3, 4, 5];
sorted.windows(2).all(|w| w[0] <= w[1]);  // true

// Validate all strings are lowercase
let words = ["hello", "world", "rust"];
words.iter().all(|w| w.chars().all(|c| c.is_lowercase()));  // true
// Note: nested .all() — outer on words, inner on chars
```

The predicate receives an element by reference in most cases — note the `|&x|` destructuring pattern when the iterator yields `&i32`.

## What This Unlocks

- **Input validation:** `inputs.iter().all(|x| x.is_valid())` — one expression to validate an entire collection.
- **Sorted-check / invariant assertion:** `windows(2).all(|w| w[0] <= w[1])` checks sortedness without writing a loop.
- **Permission checks:** `required_roles.iter().all(|role| user.has_role(role))` — clean authorization logic.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Exists | `List.exists pred lst` | `iter.any(pred)` |
| For all | `List.for_all pred lst` | `iter.all(pred)` |
| Short-circuit | Yes | Yes |
| Empty any | `false` | `false` |
| Empty all | `true` (vacuously) | `true` (vacuously) |
| Predicate type | `'a -> bool` | `Fn(T) -> bool` (receives by ref) |
