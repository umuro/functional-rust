[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 097 — Zipper
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a list zipper — a functional cursor data structure that provides O(1) navigation (left/right) and O(1) update at the focus position. The zipper stores `left` (reversed prefix), `focus` (current element), and `right` (suffix). Compare with OCaml's idiomatic record-based zipper.

## Learning Outcomes

- Model a zipper as `{ left: Vec<T>, focus: T, right: Vec<T> }` where `left` is reversed
- Implement `go_right`: pop from `right`, push current focus to `left`
- Implement `go_left`: pop from `left`, push current focus to front of `right`
- Use `update(f) -> Self` for pure functional modification at the focus
- Reconstruct the full list with `to_vec` by reversing `left` + focus + right
- Map Rust's struct-based zipper to OCaml's record with list fields

## Rust Application

`Zipper<T>` has `left: Vec<T>` (reversed — the last element of `left` is nearest the focus), `focus: T`, and `right: Vec<T>`. `go_right` takes the first element of `right` as the new focus, pushing the old focus onto `left`. `go_left` pops the top of `left` as the new focus, inserting the old focus at the front of `right`. Both return `Option<Self>` — `None` when navigation is impossible. `update(f)` applies `f` to the focus and clones the rest. All operations require `T: Clone`.

## OCaml Approach

OCaml's zipper uses immutable list fields: `{ left: 'a list; focus: 'a; right: 'a list }`. `go_right` matches `z.right` as `h :: t`, returning `{ left = z.focus :: z.left; focus = h; right = t }`. `{ z with focus = f z.focus }` updates in place. `to_list` is `List.rev z.left @ [z.focus] @ z.right`. The OCaml version is more concise because list cons `::` and pattern matching are built in.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Left field | `Vec<T>` (reversed) | `'a list` (reversed, by convention) |
| Push to left | `left.push(focus)` | `focus :: left` |
| Pop from left | `left.pop()` | `h :: t` pattern |
| Record update | `Zipper { left, focus: …, right }` | `{ z with focus = … }` |
| Navigation result | `Option<Self>` | `'a zipper option` |
| Clone requirement | `T: Clone` | Value semantics (no explicit clone) |

The zipper is the functional programmer's cursor. Instead of indices into a mutable array, it carries the context around the focus as a data structure. Navigation is O(1); reconstruction is O(n). Zippers generalise to trees (Huet zipper), enabling efficient functional editors.

## Exercises

1. Add `insert_after(value: T) -> Self` that inserts a new element immediately to the right of the focus.
2. Add `delete_focus(self) -> Option<Self>` that removes the current focus and moves right (or left if at the end).
3. Implement `goto_start(self) -> Self` that moves the focus to the leftmost position by repeatedly going left.
4. Extend the zipper to a `TreeZipper<T>` for binary trees, with `go_down_left`, `go_down_right`, and `go_up` operations.
5. In OCaml, implement a zipper for strings (treating the string as a list of characters) and use it to implement a simple text editor with insert/delete at cursor.
