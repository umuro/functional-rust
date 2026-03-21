📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1048-zipper-vec)**

---

# 1048-zipper-vec — Vec Zipper
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A zipper is a data structure that provides a "cursor" into a sequence, allowing O(1) local navigation and modification. Introduced by Gérard Huet in 1997 for functional tree navigation, the zipper splits a sequence into three parts: elements to the left of the focus, the currently-focused element, and elements to the right.

The zipper is used in functional text editors (yi, kakoune's core), syntax tree cursors in parser combinators, and game state navigation for undo/redo.

## Learning Outcomes

- Understand the zipper data structure: left (reversed), focus, right
- Implement move_left and move_right as O(1) operations
- Modify the focused element without rebuilding the whole sequence
- Reconstruct the full sequence from the zipper representation
- Connect to OCaml's zipper applications in text editors and tree traversal

## Rust Application

`src/lib.rs` implements `Zipper<T>` with `left: Vec<T>` (reversed), `focus: T`, and `right: Vec<T>`. `move_right` pops from `right` and pushes the old focus to `left`. `move_left` does the reverse. Both operations are O(1) amortized. `modify` replaces the focus element in place. `to_vec` concatenates left (reversed), focus, and right to reconstruct the original sequence.

The reversed-left invariant ensures that `left.last()` is always the element immediately to the left of the focus — O(1) for the common case of sequential navigation.

## OCaml Approach

Huet's original zipper was for trees, but the list zipper is the canonical introduction:

```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let move_right z = match z.right with
  | [] -> None
  | x :: xs -> Some { left = z.focus :: z.left; focus = x; right = xs }

let move_left z = match z.left with
  | [] -> None
  | x :: xs -> Some { left = xs; focus = x; right = z.focus :: z.right }
```

OCaml's immutable lists make zipper navigation pure: each move returns a new zipper value, leaving the old one intact for undo.

## Key Differences

1. **Mutability**: Rust's `Zipper` mutates in place with `&mut self`; OCaml's zipper returns new values (pure navigation).
2. **Undo support**: OCaml's immutable zipper naturally supports undo (keep old zipper in scope); Rust needs explicit state saving.
3. **Left reversal**: Both represent the left side reversed for O(1) access to the nearest left neighbor.
4. **Huet's original**: Huet's paper used tree zippers for XML/S-expression navigation; this example simplifies to sequences. Tree zippers are more complex but follow the same principle.

## Exercises

1. Implement `find_right<F: Fn(&T) -> bool>(pred: F) -> bool` that moves the focus right until the predicate is satisfied.
2. Add `insert_after(&mut self, value: T)` that inserts a new element immediately to the right of the focus.
3. Implement a simple line editor using `Zipper<char>` where the focus represents the cursor position.
