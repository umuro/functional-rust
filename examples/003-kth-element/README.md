# Example 003: K-th Element

**Difficulty:** ⭐  
**Category:** Lists & Indexing  
**OCaml Source:** OCaml.org 99 Problems #3

## Problem Statement

Find the k-th element of a list. The OCaml version uses 1-based indexing. We provide both 1-based (matching OCaml) and 0-based (idiomatic Rust) versions.

## Learning Outcomes

- Safe indexing with `Option` return types (no panics)
- Understanding 0-based vs 1-based indexing conventions
- `slice.get(i)` as the idiomatic safe accessor in Rust
- Recursive indexing via slice pattern matching
- How Rust's `ExactSizeIterator` makes `.nth()` O(1) on slices

## OCaml Approach

Recursive pattern match: if `k = 1`, return the head; otherwise recurse on the tail with `k - 1`. Returns `None` for empty lists. Natural and concise with linked lists.

## Rust Approach

`slice.get(index)` provides O(1) safe access. The recursive version uses slice patterns to mirror OCaml. A 1-based wrapper handles the indexing convention difference.

## Key Differences

1. **Indexing convention:** OCaml uses 1-based; Rust uses 0-based (we provide both)
2. **Access complexity:** Rust slices → O(1) random access; OCaml lists → O(k) traversal
3. **Safe access:** Rust's `.get()` returns `Option<&T>`; OCaml's pattern match returns `option`
4. **Borrowing:** Rust returns `&T` (reference); OCaml copies the value
5. **Underflow risk:** 1-based `k - 1` can underflow `usize` in Rust — must guard against `k = 0`
