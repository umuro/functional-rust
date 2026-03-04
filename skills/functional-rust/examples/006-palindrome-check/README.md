# 006: Palindrome Check

**Difficulty:** ⭐  **Level:** Beginner

Determine if a sequence reads the same forwards and backwards — with zero extra memory.

## The Problem This Solves

A palindrome reads the same in both directions: `[1, 2, 3, 2, 1]`, the word "racecar", a DNA sequence. You need to compare elements at mirror positions — first with last, second with second-to-last, and so on.

The obvious approach: reverse the list, then compare. That works, but it allocates a whole new copy of your data just to compare it. If you're checking a 10MB log sequence, that's 10MB wasted.

Rust's iterator model offers a better path: walk from both ends at the same time, without ever materializing the reversed sequence. Short-circuit on the first mismatch. Zero allocation. This is possible because Rust slices support efficient access from either end.

## The Intuition

```python
# Python — creates a full reversed copy
def is_palindrome(lst):
    return lst == lst[::-1]

# JavaScript — also creates a copy
function isPalindrome(arr) {
  return arr.join(',') === [...arr].reverse().join(',');
}
```

Rust's iterator trick:

```rust
// Zero allocation — walks both ends toward the middle simultaneously
list.iter().eq(list.iter().rev())
```

`iter()` walks left-to-right. `iter().rev()` walks right-to-left. The `.eq()` method zips them together and compares element by element, stopping the moment it finds a mismatch. No reversed copy needed.

It's like checking a palindrome by hand: you don't write it backwards on paper, you just point at the first letter and the last letter, then move your fingers toward the middle.

## How It Works in Rust

```rust
// The idiomatic solution — O(n) time, O(1) space
fn is_palindrome<T: PartialEq>(list: &[T]) -> bool {
    list.iter().eq(list.iter().rev())
}
```

The `T: PartialEq` bound means "this works for any type that supports equality comparison" — integers, strings, custom structs (if they implement `PartialEq`).

Alternative (mirrors the OCaml approach — creates a reversed copy):

```rust
fn is_palindrome_alloc<T: PartialEq + Clone>(list: &[T]) -> bool {
    let reversed: Vec<_> = list.iter().rev().cloned().collect();
    list == reversed.as_slice()
}
```

This is simpler to understand but allocates `O(n)` memory. The first version is preferred in production code.

```rust
// Edge cases — all handled correctly:
is_palindrome::<i32>(&[])          // true  (empty is a palindrome)
is_palindrome(&[1])                // true  (single element)
is_palindrome(&[1, 2, 3, 2, 1])   // true
is_palindrome(&[1, 2, 3, 4])      // false (stops at first mismatch)
```

## What This Unlocks

- **Input validation** — symmetric structures in parsers, network packets, or file formats
- **String palindromes** — apply the same logic to `&[char]` or byte slices
- **DoubleEndedIterator** — the trait that makes `.rev()` work; any iterator that supports it can use this pattern

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Idiomatic approach | `lst = List.rev lst` (allocates) | `iter().eq(iter().rev())` (zero alloc) |
| Reversal cost | Always O(n) allocation | Lazy — only if you `collect()` |
| Equality | Polymorphic `=` (implicit) | `PartialEq` trait (explicit bound) |
| Short-circuit | No (full reverse then compare) | Yes (stops at first mismatch) |
| Bidirectional iteration | Not available on linked lists | `DoubleEndedIterator` on slices |
