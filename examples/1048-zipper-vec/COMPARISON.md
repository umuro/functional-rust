# Vec Zipper — Comparison

## Core Insight
A zipper splits a data structure into context + focus, enabling O(1) local operations. OCaml's list zipper is elegant (two lists, one reversed). Rust's version uses Vec but faces ownership friction — `std::mem::take`/`replace` needed to move the focus value.

## OCaml Approach
- `{ left: 'a list; focus: 'a; right: 'a list }` — classic zipper
- `left` is reversed: closest element at head
- Navigation: cons focus onto one side, pop from other
- Immutable — each move returns a new zipper
- Shares structure with original list

## Rust Approach
- `{ left: Vec<T>, focus: T, right: Vec<T> }` — Vec-based
- Mutable: `move_right`/`move_left` modify in place
- `std::mem::replace` / `std::mem::take` for ownership transfer
- `remove(0)` on right Vec is O(n) — could use VecDeque for O(1)
- `Default` trait bound needed for safe modify

## Comparison Table

| Feature | OCaml | Rust |
|---|---|---|
| Left storage | List (reversed) | Vec (push/pop at end) |
| Right storage | List | Vec (insert/remove at 0) |
| Move cost | O(1) | O(1) left, O(n) right* |
| Mutability | Immutable | Mutable |
| Focus transfer | Pattern match | `std::mem::replace` |
| Sharing | Structural sharing | No (owned) |

*Right-side O(n) can be fixed with VecDeque or reversed Vec.
