# Comparison: Zipper — OCaml vs Rust

## Core Insight

OCaml's zipper is a textbook functional data structure: a record with three fields, navigated by pattern matching on lists. Rust's version faces the clone-vs-mutate dilemma: immutable zippers require cloning vectors on every move, while mutable zippers with `VecDeque` are more efficient but lose the functional purity.

## OCaml

```ocaml
type 'a zipper = { left: 'a list; focus: 'a; right: 'a list }

let go_right z = match z.right with
  | [] -> None
  | h :: t -> Some { left = z.focus :: z.left; focus = h; right = t }

let update f z = { z with focus = f z.focus }
```

## Rust — Immutable

```rust
pub fn go_right(&self) -> Option<Self> {
    let mut right = self.right.clone();
    if right.is_empty() { return None; }
    let new_focus = right.remove(0);
    let mut left = self.left.clone();
    left.push(self.focus.clone());
    Some(Zipper { left, focus: new_focus, right })
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type | `'a zipper` record | `Zipper<T>` struct |
| Polymorphism | Built-in `'a` | Generic `<T: Clone>` |
| Record update | `{ z with focus = ... }` | Must clone fields manually |
| List ops | O(1) cons/head | O(n) `remove(0)` on Vec |
| Navigation cost | O(1) | O(n) clone or O(1) mutable |
| Option wrapping | `Some { ... }` | `Some(Zipper { ... })` |

## Learner Notes

- **Clone cost**: OCaml's list cons is O(1) because it's just pointer manipulation. Rust's `Vec::clone` copies all elements — significant difference
- **`VecDeque`**: Rust's double-ended queue gives O(1) `pop_front`/`push_front`, making mutable zippers efficient
- **Functional record update**: OCaml's `{ z with field = ... }` is concise; Rust has no equivalent — you must construct a new struct
- **Ownership choice**: Immutable zipper = clone on every move (safe, slow). Mutable zipper = `&mut self` (fast, less composable)
- **The zipper pattern** is less common in Rust because iterators and indices often suffice
