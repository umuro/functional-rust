📖 **[View on hightechmind.io →](https://hightechmind.io/rust/570-pattern-slice)**

---

# 570: Slice Patterns: [first, rest @ ..]

**Difficulty:** 2  **Level:** Beginner

Match arrays and slices by shape — branch on length and extract elements in one expression.

## The Problem This Solves

Processing a slice differently based on its length means a nest of index checks: `if s.is_empty()`, then `else if s.len() == 1`, then `else if s.len() == 2`, then access `s[0]`, `s[1]`, hope you got the bounds right. It's boilerplate scaffolding around a conceptually simple idea.

The deeper issue is structural recursion. Functional algorithms like sum and product are naturally recursive on lists: "if empty return base case, otherwise take the head and recurse on the tail." In most imperative languages you'd convert this to a loop, losing the clean recursive structure. Slice patterns restore it: `[x, rest @ ..]` gives you the head and tail directly, matching the mathematical definition.

The `..` wildcard is particularly useful for "first and last" patterns. You want the first element for one thing and the last for another, and you don't care how many are in between. Previously you'd call `.first()` and `.last()` separately. With slice patterns: `[first, .., last]`.

## The Intuition

Slice patterns describe the *shape* of a slice. `[]` means empty. `[x]` means exactly one element, bound to `x`. `[a, b]` means exactly two. `[a, b, ..]` means "at least two, ignore the rest." `[first, .., last]` means "at least two, I want the first and last."

This is OCaml's list pattern `x :: xs` generalized to contiguous memory. OCaml matches on cons cells (`::` is the list constructor), so it's `[]` for empty and `head :: tail` for non-empty. Rust matches on actual memory slices, so `[x, rest @ ..]` is the equivalent. The `rest @ ..` part uses `@` to bind the remaining elements as a sub-slice.

The key insight: this works on `&[T]`, `[T; N]` (fixed-size arrays), and anything that coerces to a slice. One syntax, any contiguous collection.

## How It Works in Rust

```rust
// Recursive sum via slice pattern
fn sum(s: &[i32]) -> i32 {
    match s {
        []             => 0,              // base case: empty
        [x, rest @ ..] => x + sum(rest),  // head + recurse on tail
    }
}

// Describe by shape — matches on exact lengths and first/last
fn describe(s: &[i32]) -> String {
    match s {
        []              => "empty".into(),
        [x]             => format!("one: {}", x),
        [a, b]          => format!("pair: ({},{})", a, b),
        [first, .., last] => format!("many: {}..{}", first, last),
    }
}

// Extract only what you need
fn first_two(s: &[i32]) -> Option<(i32, i32)> {
    match s {
        [a, b, ..] => Some((*a, *b)),  // at least 2 elements
        _          => None,
    }
}

// Mutation via ref mut in slice pattern
fn double_first(v: &mut [i32]) {
    if let [ref mut first, ..] = v {
        *first *= 2;
    }
}
```

## What This Unlocks

- **Structural recursion on slices** — write sum, product, fold naturally without index arithmetic.
- **First-and-last extraction** — `[first, .., last]` in one arm, no `.first()` + `.last()` + bounds checks.
- **Length-based dispatch** — different behavior for empty, singleton, pair, and many-element slices in clean match arms.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Head/tail | `x :: xs` (list cons) | `[x, rest @ ..]` (slice pattern) |
| Empty | `[]` | `[]` |
| Fixed two elements | `[a; b]` | `[a, b]` |
| First and last | No direct syntax | `[first, .., last]` |
| Data structure | Linked list (`list`) | Contiguous slice (`&[T]`) |
| Sub-slice binding | N/A (list tail is natural) | `rest @ ..` binds remaining elements |
