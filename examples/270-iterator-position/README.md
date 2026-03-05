📖 **[View on hightechmind.io →](https://hightechmind.io/rust/270-iterator-position)**

---

# 270: Finding Index with position()

**Difficulty:** 1  **Level:** Beginner

Find the zero-based index of the first element satisfying a predicate — returns `Option<usize>`.

## The Problem This Solves

You need to know *where* something is, not just *whether* it exists. The insertion point for a binary search result. The index at which to split a slice. The column number of the first error in a row. `find()` gives you the value; `position()` gives you the index — and indexes let you do things that values alone don't: slice the array, compute an offset, build an error message with a position.

Without `position()`, you'd use `enumerate()` + `find()` and then destructure the result — three operations instead of one. Or you'd write a manual loop with a counter. `position()` is the direct answer to "at what index is the first X?".

In OCaml, you'd write `let rec find_pos pred i = function [] -> None | x :: xs -> if pred x then Some i else find_pos pred (i+1) xs`. In Rust, `position()` is a one-liner.

## The Intuition

`position(pred)` returns `Some(index)` where `index` is the zero-based position of the first element for which the predicate returns `true`, or `None` if no element matches.

```rust
let nums = [10, 20, 30, 40, 50];
let idx = nums.iter().position(|&x| x > 25);
// → Some(2)  (first element > 25 is 30, at index 2)
```

## How It Works in Rust

```rust
let nums = [10i32, 20, 30, 40, 50];

// Basic: find index of first match
let idx = nums.iter().position(|&x| x > 25);
// → Some(2)

// With pattern matching
let words = ["apple", "banana", "cherry", "date"];
match words.iter().position(|&w| w == "cherry") {
    Some(i) => println!("'cherry' at index {}", i),  // → 2
    None    => println!("Not found"),
}

// Use index to slice around the found element
if let Some(split) = nums.iter().position(|&x| x == 30) {
    let before = &nums[..split];    // → [10, 20]
    let after  = &nums[split+1..];  // → [40, 50]
}

// rposition: search from the right
let data = [1i32, 2, 3, 2, 1];
let first = data.iter().position(|&x| x == 2);   // → Some(1)
let last  = data.iter().rposition(|&x| x == 2);  // → Some(3)

// Chain with map for a formatted message
let message = nums.iter()
    .position(|&x| x >= 35)
    .map(|i| format!("Found {} at index {}", nums[i], i))
    .unwrap_or_else(|| "Not found".to_string());
```

`position()` consumes the iterator up to (and including) the found element. Use `rposition()` on slices to search from the right.

## What This Unlocks

- **Slice splitting** — find the pivot point to split a slice into before/after halves.
- **Error location reporting** — report which element in a sequence failed validation and at what index.
- **Insertion point** — find where a new element should be inserted to maintain order (combine with `partition_point` for sorted slices).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Find index by predicate | Manual recursion with counter | `iter.position(pred)` |
| Search from right | Manual recursion | `slice.rposition(pred)` |
| Returns | `int option` | `Option<usize>` |
| vs. `find()` | `find` returns the value | `position` returns the index |
| vs. `enumerate().find()` | Equivalent but verbose | `position()` is the direct shortcut |
