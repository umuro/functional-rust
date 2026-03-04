# 021: Insert At

**Difficulty:** 1  **Level:** Foundations

Insert an element at a specific position (1-based) in a list, shifting everything else right.

## The Problem This Solves

You're maintaining an ordered list — a playlist, a ranked leaderboard, a priority queue — and need to insert a new item at a specific position without replacing anything. The task: given `['a','b','c','d']`, element `'X'`, and position `2`, produce `['a','X','b','c','d']`.

In Python: `lst.insert(k-1, elem)` — straightforward, but mutates the original list. If the caller still holds a reference to the original, they now see the modified list. Python doesn't warn you.

Rust's version takes a slice (a reference), clones it into a new Vec, and returns the result — the original is untouched. The function signature itself communicates "I won't modify your data." And if `k` is beyond the list end, inserting at the tail is the natural fallback — no panic, no error, just sensible behavior.

## The Intuition

In Python: `new_list = lst[:k-1] + [elem] + lst[k-1:]` (non-mutating version)

In JavaScript: `[...lst.slice(0, k-1), elem, ...lst.slice(k-1)]`

Inserting at position `k` (1-based) means: "everything before position k stays, then the new element, then everything from position k onward." In 0-based terms, the new element goes at index `k-1`.

Rust's `Vec::insert(pos, elem)` does this in one call. The tricky edge cases — inserting at position 0, inserting past the end — are handled by `.saturating_sub(1).min(lst.len())`: `saturating_sub` prevents underflow on 0, and `.min(lst.len())` clamps beyond-end insertions to append.

## How It Works in Rust

```rust
fn insert_at<T: Clone>(elem: T, lst: &[T], k: usize) -> Vec<T> {
    // k=1 → index 0; k=99 on a 4-element list → index 4 (append)
    let pos = k.saturating_sub(1).min(lst.len());
    let mut result = lst.to_vec();  // clone: don't mutate the original
    result.insert(pos, elem);       // Vec::insert shifts everything right
    result
}
```

- `k.saturating_sub(1)` — converts 1-based to 0-based, but if `k=0` it saturates to 0 (no underflow)
- `.min(lst.len())` — if `k` exceeds the length, insert at the end
- `lst.to_vec()` — create an owned copy so we don't modify the input
- `Vec::insert(pos, elem)` — O(n) shift; inserts `elem` at `pos`, shifting right

## What This Unlocks

- **Ordered collections** — insert into a sorted list at the found position.
- **Editor operations** — insert text at a cursor position.
- **Queue injection** — insert a high-priority item at the front or middle of a work queue.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Insert operation | Recursive: rebuild list up to k, prepend elem | `Vec::insert(pos, elem)` — stdlib method |
| Mutation | OCaml lists immutable; new list returned | Clone first (`to_vec()`), then mutate clone |
| 1-based index | Decremented in recursive countdown | `saturating_sub(1)` converts at entry |
| Out-of-bounds k | Inserts at end (recursive base case) | `.min(lst.len())` clamps to append |
| Ownership | `'a list` — GC managed | `Vec<T>` — owned, heap-allocated |
