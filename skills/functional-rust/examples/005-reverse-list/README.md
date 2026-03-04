# 005: Reverse a List

**Difficulty:** ⭐  **Level:** Beginner

Flip the order of elements in a list — and choose whether to create a copy or reverse in place.

## The Problem This Solves

Reversing a list comes up constantly: displaying history newest-first, reversing a traversal order, building a result by prepending items then flipping at the end. Every language handles it a bit differently.

Python's `lst[::-1]` creates a copy. JavaScript's `arr.reverse()` mutates in place (and many JS programmers have been surprised to learn this). Java's `Collections.reverse()` also mutates. In functional languages like OCaml, everything creates a new list — there's no mutation.

Rust is explicit about the choice. Want a new reversed copy? `iter().rev().collect()`. Want to reverse the existing data in place (faster, no allocation)? `slice.reverse()`. The type system makes the difference obvious: in-place requires `&mut`, creating a copy requires `T: Clone`.

## The Intuition

```python
# Python
original = [1, 2, 3, 4]
reversed_copy = original[::-1]   # new list, original unchanged
original.reverse()                # in-place, original is now [4, 3, 2, 1]
```

```javascript
// JavaScript — careful! this mutates!
arr.reverse()  // [4, 3, 2, 1] — arr is changed
// To get a copy: [...arr].reverse()
```

```rust
// Rust — the choice is explicit in the function signature

// Creates a new Vec (original unchanged)
let reversed: Vec<_> = list.iter().rev().cloned().collect();

// Reverses in place (no allocation)
list.reverse();  // requires `mut` — you can't do this by accident
```

The `iter().rev()` trick is elegant: `.rev()` doesn't actually move any data — it just reverses the *direction* the iterator walks. Only `.collect()` materializes the result into a new `Vec`.

## How It Works in Rust

```rust
// Option 1: Create a reversed copy (immutable input)
fn rev<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().rev().cloned().collect()
    //          ^^^^              ^^^^^^
    //          lazy reverse      now allocate
}

// Option 2: Reverse in place (zero allocation)
fn rev_mut<T>(list: &mut [T]) {
    list.reverse();  // swaps elements: O(n) time, O(1) extra space
}
```

Why `clone()`? The input is a borrowed slice `&[T]` — we don't own those values. To put them into a new `Vec`, we need to copy them. The `T: Clone` bound says "this only works for types that support copying."

The functional fold version (mirrors OCaml's accumulator pattern):

```rust
// Mirrors: let rec aux acc = function h :: t -> aux (h :: acc) t
// Note: insert(0, ...) is O(n²) — use iter().rev() instead
fn rev_fold<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        acc.insert(0, x.clone());  // prepend — educational, not optimal
        acc
    })
}
```

## What This Unlocks

- **Processing history or logs in reverse chronological order** without disturbing the original
- **Building lists by appending then reversing** — the classic functional pattern for O(n) list construction
- **Bidirectional iteration** — `iter().rev()` works on any slice without creating a copy

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Always creates new list | Yes (GC manages memory) | Optional — in-place possible |
| In-place reversal | Not available | `slice.reverse()` with `&mut` |
| Functional accumulator | `h :: acc` (O(1) prepend) | `iter().rev().collect()` |
| Lazy reversal | Not applicable | `iter().rev()` — no data moved |
| Requires `Clone` | Implicit copy on GC heap | Explicit `T: Clone` bound |
| Stack safety | TCO via accumulator | Iterators — no stack concern |
