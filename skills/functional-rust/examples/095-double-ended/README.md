# 095: DoubleEndedIterator

**Difficulty:** 3  **Level:** Intermediate

Iterate from both ends simultaneously — `.rev()`, `.next_back()`, palindrome checks, and symmetric traversal.

## The Problem This Solves

Some algorithms naturally work inward from both ends: palindrome verification, symmetric reductions, trimming from both sides, interleaving front and back. Without bidirectional traversal, you'd reverse the entire collection first — allocating a copy just to read it backwards.

`DoubleEndedIterator` makes both ends available simultaneously. You can pull from the front with `.next()` and from the back with `.next_back()`, narrowing toward the middle without any extra allocation.

This is what makes `.rev()` zero-cost — it just swaps which end `.next()` reads from.

## The Intuition

Think of a double-ended queue (deque): `.next()` pops the front, `.next_back()` pops the back. Both shrink the same underlying view.

OCaml lists are singly-linked — back access requires `List.rev` which copies. Arrays work with index arithmetic. Rust slices implement `DoubleEndedIterator` natively, so both directions work on the same data without copies.

Python's `reversed()` creates a separate reversed iterator — similar but doesn't allow simultaneous front/back consumption.

## How It Works in Rust

```rust
// Palindrome: compare elements from both ends moving inward
fn palindrome_check<T: PartialEq>(data: &[T]) -> bool {
    let mut iter = data.iter();
    loop {
        match (iter.next(), iter.next_back()) {
            (Some(a), Some(b)) => if a != b { return false; },
            // Met in the middle (odd length) or exhausted (even length)
            _ => return true,
        }
    }
}

// Get first and last without two separate passes
fn ends(data: &[i32]) -> Option<(i32, i32)> {
    let mut iter = data.iter();
    let first = iter.next()?;
    let last = iter.next_back().unwrap_or(first); // handles single-element
    Some((*first, *last))
}

// .rev() is zero-cost: just swaps next() ↔ next_back()
fn last_n<T: Clone>(data: &[T], n: usize) -> Vec<T> {
    data.iter()
        .rev()
        .take(n)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()  // restore original order
        .collect()
}

// rfind: search from the back (built-in on DoubleEndedIterator)
fn last_even(data: &[i32]) -> Option<&i32> {
    data.iter().rfind(|&&x| x % 2 == 0)
}
```

Note: adapters like `.filter()` remove `DoubleEndedIterator` because they can't know the back length. `.map()` and `.take()` preserve it.

## What This Unlocks

- **Palindrome and symmetry checks**: compare inward from both ends without reversing.
- **Symmetric trimming**: strip matching elements from front and back (like `trim()` but generic).
- **Efficient last-N**: `.rev().take(n)` without reversing the whole collection.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Back iteration | `List.rev` then forward | `.next_back()` built-in |
| Reverse | `List.rev` (O(n) copy) | `.rev()` zero-cost adapter |
| rfind | Manual | `.rfind()` built-in |
| Simultaneous ends | Not possible on lists | `iter.next()` + `iter.next_back()` |
| filter preserves DEI | N/A | No (unknown back length) |
