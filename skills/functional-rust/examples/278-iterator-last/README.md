# 278: Iterator last()

**Difficulty:** 1  **Level:** Beginner

Consume the entire iterator and return the final element wrapped in `Option<T>`, or `None` if empty.

## The Problem This Solves

You want the last element of a sequence — the final log entry, the last matching result, the most recently added item. On a slice you can use `.last()` for an O(1) reference. But on an arbitrary iterator — one produced by `filter`, `map`, or `lines()` — there's no O(1) shortcut. You have to consume the whole thing to find the end.

Without `last()`, you'd fold with `None` as the accumulator: `iter.fold(None, |_, x| Some(x))`. That works but hides intent. `last()` is one word that says exactly what you mean.

For slices: prefer `slice.last()` directly — it returns `Option<&T>` in O(1). The iterator's `last()` is for pipelines where you don't have a slice reference, like after a chain of adapters.

## The Intuition

Advance through the entire iterator, discarding everything except the final element, then return it as `Option<T>`.

## How It Works in Rust

```rust
let nums = [1i32, 2, 3, 4, 5];

// Iterator's last() — O(n), consumes the iterator
nums.iter().last();  // Some(&5)

// Slice's last() — O(1), returns a reference, doesn't consume
nums.last();  // Some(&5)

// Empty → None (safe, no panic)
let empty: Vec<i32> = vec![];
empty.iter().last();  // None

// Last after filtering — O(n), traverses the whole range
let last_even = (1..=10).filter(|x| x % 2 == 0).last();  // Some(10)

// Last word in a sentence
"the quick brown fox".split_whitespace().last();  // Some("fox")

// Last non-empty line of a file
let text = "line1\nline2\nline3";
text.lines().last();  // Some("line3")
```

Key distinction: when you call `last()` on an iterator adapter chain, the *entire* chain runs to completion. This is fine for small data; think twice before calling `last()` on a million-element filtered stream when you only need the final value.

## What This Unlocks

- **Log tail:** Get the most recent log entry matching a pattern — `logs.iter().filter(|l| l.level == Error).last()`.
- **Pipeline final result:** When building up a transformation chain, grab the final produced value without collecting everything.
- **Safe sequence end:** `Option<T>` forces you to handle empty input — no index-out-of-bounds, no crash on empty files.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Last of list | `List.nth lst (List.length lst - 1)` | `iter.last()` |
| Empty handling | `Invalid_argument` exception | Returns `None` |
| O(1) shortcut | `Array.(arr.(Array.length arr - 1))` | `slice.last()` (not iter) |
| Complexity | O(n) for lists | O(n) for iterators, O(1) for slices |
| After calling | List still available | Iterator consumed |
