📖 **[View on hightechmind.io →](https://hightechmind.io/rust/272-tree-zipper)**

---

# Example 272: Tree Zipper
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a *zipper* for binary trees: a data structure that tracks a focused
subtree together with a breadcrumb trail back to the root, enabling O(1)
local navigation (`go_left`, `go_right`, `go_up`) and functional in-place
editing (`set_value`) without mutating the original tree.

## Learning Outcomes

- How zippers decompose a tree into a focus and a context (trail of crumbs)
- How Rust's ownership model shapes the API — consuming `Zipper<T>` by value makes navigation safe and allocation-free
- Why Rust requires an iterative `to_tree` loop where OCaml can express the same idea as a one-liner (`match go_up z with None -> z.focus | Some z' -> to_tree z'`)
- That functional "editing" means constructing a new path on the way back up, leaving the old tree intact

## OCaml Approach

OCaml uses a record `{ focus: 'a tree; trail: 'a crumb list }` with a prepend-cons trail.
Each navigation function returns an `option` zipper. Because OCaml passes records by
value (copying), `go_up` can pattern-match on `go_up z` and still reference `z.focus`
in the `None` branch — the compiler has no ownership concern.

## Rust Approach

Rust models the same structure with an owned `Zipper<T>` (focus + `Vec<Crumb<T>>`).
`go_left` / `go_right` / `go_up` consume the zipper and return `Option<Zipper<T>>`,
making the state transition explicit in the type. Because `go_up` moves `z`, `to_tree`
cannot read `z.focus` after the call; instead it checks `z.trail.is_empty()` first
and loops, which is both idiomatic and tail-call-free in practice.

## Key Differences

1. **Ownership vs. copying:** OCaml copies the record on each navigation step; Rust
   moves it, making it clear that the old zipper is consumed and the new one is
   returned.
2. **Trail representation:** OCaml uses a linked list (cons-prepend); Rust uses a
   `Vec` (push/pop), which is more cache-friendly and avoids allocation per step.
3. **`to_tree` idiom:** OCaml can read `z.focus` after `go_up z` in the same match;
   Rust requires checking `z.trail.is_empty()` before calling `go_up` because `go_up`
   consumes `z`.
4. **Null safety:** Both languages return `Option` for navigation — the only
   difference is syntax (`None`/`Some` is identical; OCaml's `Option.get` vs.
   Rust's `.expect()`).

## Exercises

1. Implement `navigate_right` and `navigate_left` for the tree zipper, enabling horizontal movement between siblings.
2. Write a `map_focused` function that applies a transformation to the currently focused node and returns the updated zipper.
3. Build a simple tree editor using the zipper: support commands `up`, `down_left`, `down_right`, `insert_left`, `insert_right`, and `delete_focus`, reconstructing the full tree after a sequence of operations.
