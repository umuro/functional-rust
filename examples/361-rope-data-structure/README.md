📖 **[View on hightechmind.io →](https://hightechmind.io/rust/361-rope-data-structure)**

---

# 361: Rope Data Structure
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Concatenating strings with `String::push_str` or `+` is O(n) — it copies all bytes each time. For a text editor that concatenates hundreds of small edits, this becomes O(n²) across all operations. A rope (Boehm, Atkinson & Plass, 1995) solves this with a binary tree of string fragments: concatenation is O(1) (just create a new tree node), and converting to a flat string is O(n) (tree traversal). Text editors (VS Code's Monaco uses ropes), version control systems, and collaborative editing frameworks use ropes to handle large documents with frequent insertions and deletions efficiently.

## Learning Outcomes

- Implement a `Rope` enum with `Leaf(String)` and `Node { left, right, length }` variants
- Achieve O(1) concatenation by creating a new `Node` wrapping two sub-ropes
- Cache the `length` in each `Node` to avoid recomputing it on every query
- Flatten a rope to `String` with a recursive tree traversal in O(n)
- Implement O(log n) index access via `char_at` by comparing index to subtree sizes
- Understand the tradeoff: constant factors favor `String` for small texts

## Rust Application

```rust
#[derive(Debug, Clone)]
pub enum Rope {
    Leaf(String),
    Node { left: Box<Rope>, right: Box<Rope>, length: usize },
}

impl Rope {
    pub fn leaf(s: impl Into<String>) -> Self {
        Self::Leaf(s.into())
    }

    pub fn length(&self) -> usize {
        match self {
            Self::Leaf(s) => s.len(),
            Self::Node { length, .. } => *length,
        }
    }

    pub fn concat(left: Rope, right: Rope) -> Rope {
        if left.length() == 0 { return right; }
        if right.length() == 0 { return left; }
        let length = left.length() + right.length();
        Rope::Node {
            left: Box::new(left),
            right: Box::new(right),
            length, // cached — O(1) length query
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Leaf(s) => s.clone(),
            Self::Node { left, right, .. } => {
                let mut result = left.to_string();
                result.push_str(&right.to_string());
                result
            }
        }
    }
}
```

The `length` field in `Node` is crucial — without it, `length()` would be O(n) (full tree traversal). With it, it's O(1) since the value was precomputed at construction. The `Box` indirection is required because `Rope` would otherwise have infinite size (recursive type).

## OCaml Approach

OCaml's algebraic types map directly to this structure:

```ocaml
type rope =
  | Leaf of string
  | Node of { left: rope; right: rope; length: int }

let leaf s = Leaf s

let length = function
  | Leaf s -> String.length s
  | Node { length; _ } -> length

let concat left right =
  let length = length left + length right in
  Node { left; right; length }

let rec to_string = function
  | Leaf s -> s
  | Node { left; right; _ } -> to_string left ^ to_string right
```

OCaml's pattern matching and algebraic types make rope implementation especially clean. The structure is identical to Rust's enum — both languages excel at recursive algebraic data type definitions. Persistent (immutable) ropes are natural in OCaml; in Rust you need explicit `Clone` or use `Rc`/`Arc`.

## Key Differences

| Aspect | Rust `Rope` enum | OCaml `rope` type |
|--------|----------------|-------------------|
| Recursive box | Explicit `Box<Rope>` | Automatic (GC-managed) |
| Immutability | `&self` methods, `Clone` for copies | Immutable by default |
| Pattern matching | `match` on enum | `match` on variant |
| Memory | Heap allocation per node | Heap allocation per node (GC) |
| Production library | `ropey` crate | `Rope` in `containers` library |

## Exercises

1. **char_at**: Implement `char_at(&self, index: usize) -> Option<char>` that navigates the tree: if `index < left.length()`, recurse left; else recurse right with `index - left.length()`.
2. **Rebalancing**: After many concatenations, the tree may be unbalanced (skewed right if always appending). Implement `rebalance(rope) -> Rope` using `to_string().chars().collect()` plus a divide-and-conquer tree builder.
3. **Insert at position**: Implement `insert(rope, pos, text) -> Rope` by splitting the rope at `pos` (two halves) and concatenating `left + leaf(text) + right` — no copying of original content.
