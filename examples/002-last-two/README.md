📖 **[View on hightechmind.io →](https://hightechmind.io/rust/002-last-two)**

---

# Example 002: Last Two Elements

**Difficulty:** ⭐  
**Category:** Lists & Pattern Matching  
**OCaml Source:** OCaml.org 99 Problems #2

## Problem Statement

Find the last two elements of a list. Return them as a pair (tuple), or `None` if the list has fewer than two elements.

## Learning Outcomes

- Pattern matching on lists/slices with multiple cases
- Returning tuples (pairs) from functions
- Understanding `Option<(T, T)>` vs OCaml's `option of (a * a)`
- Using `windows()` for sliding-window iteration
- Recursive slice destructuring with `[_, rest @ ..]`

## OCaml Approach

OCaml uses nested pattern matching: `[] | [_] -> None`, `[x; y] -> Some (x, y)`, and `_ :: t -> last_two t`. The recursive structure naturally peels off the head until two elements remain.

## Rust Approach

Three approaches: (1) direct length-based indexing for O(1) access, (2) recursive slice patterns mirroring OCaml, and (3) `windows(2).last()` for an iterator-based solution. All return references (`&T`) to avoid cloning.

## Key Differences

1. **Return type:** OCaml returns `Some (x, y)` (owned copy); Rust returns `Option<(&T, &T)>` (borrowed references)
2. **Tuple syntax:** OCaml `(x, y)` vs Rust `(x, y)` — similar, but Rust tuples can hold references
3. **Slice access:** Rust slices are contiguous memory → O(1) indexing; OCaml lists are linked → O(n) traversal
4. **Windows iterator:** Rust's `windows(n)` has no direct OCaml equivalent — it exploits contiguous memory
5. **Exhaustive matching:** Both languages require exhaustive patterns, but Rust's slice patterns (`[_, rest @ ..]`) are less common than OCaml's list patterns
