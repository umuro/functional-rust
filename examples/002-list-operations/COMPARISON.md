## Core Insight

OCaml's list is an immutable singly-linked list (O(1) prepend, O(n) append). Rust's `Vec<T>` is a contiguous growable array (O(1) push to end, O(n) insert at front). This fundamental difference shapes idiomatic usage.

## OCaml Approach
- `List.hd` / `List.tl` for head/tail (raise exception on empty)
- `List.length` counts nodes — O(n)
- `@` or `List.append` for concatenation — O(n) in first list
- `List.rev` for reverse — O(n)
- Pattern matching preferred over `hd`/`tl`

## Rust Approach
- `.first()` / `.last()` return `Option<&T>`
- `.len()` is O(1) — stored metadata
- `.extend()` or `[a, b].concat()` for append
- `.reverse()` in-place or `.iter().rev().collect()`
- Slices `&[T]` for borrowing sub-ranges

## Comparison Table

| Operation | OCaml | Rust Vec |
|-----------|-------|----------|
| Head | `List.hd` / pattern match | `.first()` → `Option` |
| Tail | `List.tl` | `&v[1..]` slice |
| Length | `List.length` O(n) | `.len()` O(1) |
| Prepend | `x :: lst` O(1) | `.insert(0, x)` O(n) |
| Append | `lst @ [x]` O(n) | `.push(x)` O(1) amortized |
| Reverse | `List.rev` | `.reverse()` in-place |
