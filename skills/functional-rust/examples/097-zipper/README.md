# 097: Zipper — Functional List Cursor

**Difficulty:** 3  **Level:** Intermediate

A data structure that marks a position in a list for O(1) local edits — focus + context, no index arithmetic.

## The Problem This Solves

When you need to navigate and edit a position inside a list repeatedly — a text editor cursor, a game board traversal, a tree walk — the naive approach is to keep an integer index and slice. But every edit requires rebuilding the whole structure around that index.

A Zipper splits the list into three parts: **elements to the left** (reversed), the **focused element**, and **elements to the right**. Moving left or right, replacing the focus, inserting, or deleting are all O(1) — no rebuilding, no index bounds to manage.

The Zipper originated in OCaml/Haskell as a functional answer to "how do you efficiently walk and edit a position in an immutable data structure."

## The Intuition

Imagine holding a bead curtain. Your hand is the focus. Beads to your left fall behind you (reversed in memory for O(1) access). Beads to your right hang in front. Moving left: take the nearest bead from your left pile, make it the new focus, push old focus onto the right. Moving right: mirror operation.

The reversed-left invariant is the key insight. The top of `left` (the last element pushed) is always the immediate left neighbor of `focus` — so `left.pop()` and `left.push()` are the move operations.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
pub struct Zipper<T> {
    left: Vec<T>,   // reversed: top = nearest to focus
    focus: T,
    right: Vec<T>,  // normal order: front = nearest to focus
}

impl<T: Clone> Zipper<T> {
    pub fn from_vec(v: Vec<T>) -> Option<Self> {
        let mut iter = v.into_iter();
        let focus = iter.next()?;       // first element becomes focus
        Some(Zipper { left: vec![], focus, right: iter.collect() })
    }

    pub fn go_right(&self) -> Option<Self> {
        let mut right = self.right.clone();
        if right.is_empty() { return None; }
        let new_focus = right.remove(0);      // take from front of right
        let mut left = self.left.clone();
        left.push(self.focus.clone());         // push old focus onto left
        Some(Zipper { left, focus: new_focus, right })
    }

    pub fn go_left(&self) -> Option<Self> {
        let mut left = self.left.clone();
        let new_focus = left.pop()?;           // pop from top of reversed-left
        let mut right = self.right.clone();
        right.insert(0, self.focus.clone());   // push old focus onto right
        Some(Zipper { left, focus: new_focus, right })
    }

    pub fn set_focus(&self, value: T) -> Self {
        // Replace focus: O(1) — only touch one field
        Zipper { left: self.left.clone(), focus: value, right: self.right.clone() }
    }

    pub fn to_vec(&self) -> Vec<T> {
        // Reconstruct: reverse left, add focus, add right
        let mut v: Vec<T> = self.left.iter().rev().cloned().collect();
        v.push(self.focus.clone());
        v.extend(self.right.iter().cloned());
        v
    }
}
```

The clone-based approach is idiomatic for functional style. For high-performance use cases, use `VecDeque` with split positions.

## What This Unlocks

- **Text editors**: cursor movement and insertion without rebuilding the buffer.
- **Tree traversal**: extend Zipper to trees — walk up, down, left, right with a "breadcrumb" context stack.
- **Game boards**: navigate cells and modify them locally without full-board copies.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Record type | `type 'a t = { left: 'a list; focus: 'a; right: 'a list }` | `struct Zipper<T> { left: Vec<T>, focus: T, right: Vec<T> }` |
| Functional update | `{ z with focus = v }` | `.set_focus(v)` clones and replaces |
| Left invariant | Reversed list | Reversed `Vec` (same) |
| Move right | `List.hd right`, cons to left | `right.remove(0)`, push to left |
| Immutable by default | Yes | Via clone; or use `&mut self` for in-place |
