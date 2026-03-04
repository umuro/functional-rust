# 020: Remove Kth

**Difficulty:** 1  **Level:** Foundations

Remove the element at position K (1-based), returning both the removed element and the remaining list.

## The Problem This Solves

You're implementing a deck of cards: draw the top card and get back the rest of the deck. Or you're processing a job queue: pop the Nth job by position and continue with what's left. The task: given `['a','b','c','d']` and `k=2`, produce `('b', ['a','c','d'])` — the extracted element paired with the remainder.

In Python: `elem = lst.pop(k-1)` mutates in place. That works, but mutation is invisible — callers can't tell from the function signature whether their list will be modified. Silent mutation causes subtle bugs when the same list is referenced elsewhere.

Rust returns `Option<(T, Vec<T>)>` — the result explicitly says "this might fail" (if `k` is out of bounds) and returns both the element *and* the modified list as a value. No mutation, no surprises, and the compiler forces you to handle the `None` case.

## The Intuition

In Python: `elem = lst.pop(k-1)` or `elem, rest = lst[k-1], lst[:k-1] + lst[k:]`

In JavaScript: `const [elem, ...rest] = [lst[k-1], ...lst.slice(0, k-1), ...lst.slice(k)]`

The key insight: Rust functions return values, not mutations. Instead of modifying the list in place, `remove_at` returns a *new* list with the element missing. This is the functional style — and it makes the operation composable and testable.

The `Option` wrapper is important. If `k = 0` or `k > lst.len()`, there's nothing to remove — returning `None` is cleaner than panicking or returning a dummy value.

## How It Works in Rust

```rust
fn remove_at<T: Clone>(lst: &[T], k: usize) -> Option<(T, Vec<T>)> {
    if k == 0 || k > lst.len() {
        return None;  // out of bounds: signal failure, don't panic
    }
    let idx = k - 1;               // convert 1-based to 0-based
    let elem = lst[idx].clone();   // clone the element out
    let mut remaining = lst.to_vec();
    remaining.remove(idx);         // remove from the cloned vec
    Some((elem, remaining))        // return both
}
```

- `k == 0 || k > lst.len()` — explicit bounds check returns `None` instead of panicking
- `k - 1` — 1-based to 0-based conversion
- `.clone()` — we need an owned value to return alongside the new list
- `Vec::remove(idx)` — the standard library method for in-place removal on a Vec

Pattern match the result at the call site:
```rust
if let Some((elem, rest)) = remove_at(&input, 2) {
    println!("Removed: {}, Remaining: {:?}", elem, rest);
}
```

## What This Unlocks

- **Deck/queue operations** — draw a card by position, returning the drawn card and remaining deck.
- **Undo stacks** — remove a specific action from a history list.
- **Validated extraction** — the `Option` return makes out-of-bounds a handled case, not a crash.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Return style | Returns `(elem, rest)` or raises | Returns `Option<(T, Vec<T>)>` |
| Bounds failure | Exception / `Invalid_argument` | `None` — handled at call site |
| 1-based index | Part of the problem spec | `k - 1` converts to 0-based internally |
| Mutation | OCaml lists are immutable (new list built) | `Vec::remove` mutates a cloned copy |
| Destructuring result | `let (e, r) = remove_at lst k` | `if let Some((e, r)) = remove_at(...)` |
