**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[rope-string on hightechmind.io](https://hightechmind.io/posts/functional-rust/rope-string)

---

## Problem Statement

Implement a rope — a tree-based string representation that enables O(log n) concatenation, splitting, and indexing for large strings. A `Rope` is either a `Leaf(String)` or a `Node(left, right, total_len)`. Concatenation creates a new `Node` without copying content. Collecting to a `String` is O(n) but infrequent.

## Learning Outcomes

- Define `enum Rope { Leaf(String), Node(Box<Rope>, Box<Rope>, usize) }` with length cached in `Node`
- Implement `concat(left, right) -> Rope` that creates a `Node` in O(1) — no string copying
- Implement `len(&self)` by reading the cached length from `Node` or `Leaf::len()`
- Implement `char_at(&self, idx: usize) -> Option<char>` by navigating left/right subtrees using cached lengths
- Implement `to_string_val(&self) -> String` as a recursive O(n) collection

## Rust Application

```rust
#[derive(Debug, Clone)]
pub enum Rope {
    Leaf(String),
    Node(Box<Rope>, Box<Rope>, usize),  // left, right, total_len
}

impl Rope {
    pub fn from_str(s: &str) -> Self { Rope::Leaf(s.to_string()) }

    pub fn len(&self) -> usize {
        match self {
            Rope::Leaf(s) => s.len(),
            Rope::Node(_, _, n) => *n,
        }
    }

    pub fn concat(left: Rope, right: Rope) -> Rope {
        let total = left.len() + right.len();
        Rope::Node(Box::new(left), Box::new(right), total)
    }

    pub fn char_at(&self, idx: usize) -> Option<char> {
        match self {
            Rope::Leaf(s) => s.chars().nth(idx),
            Rope::Node(left, right, _) => {
                if idx < left.len() {
                    left.char_at(idx)
                } else {
                    right.char_at(idx - left.len())
                }
            }
        }
    }

    pub fn to_string_val(&self) -> String {
        match self {
            Rope::Leaf(s) => s.clone(),
            Rope::Node(l, r, _) => {
                let mut out = l.to_string_val();
                out.push_str(&r.to_string_val());
                out
            }
        }
    }
}
```

Concatenation is O(1) — it only allocates one `Node` struct without touching the string content. The total length is cached in the node so `len()` is also O(1). `char_at` navigates left if the index falls within the left subtree, right otherwise, in O(log n) for a balanced rope.

`to_string_val` is O(n) — it must copy every character. In practice, ropes are used when concatenation and splitting dominate; final output is a single `to_string_val` call.

## OCaml Approach

```ocaml
type rope =
  | Leaf of string
  | Node of rope * rope * int  (* left, right, len *)

let from_str s = Leaf s

let len = function
  | Leaf s -> String.length s
  | Node (_, _, n) -> n

let concat l r = Node (l, r, len l + len r)

let rec char_at rope idx =
  match rope with
  | Leaf s -> if idx < String.length s then Some s.[idx] else None
  | Node (left, right, _) ->
    let ll = len left in
    if idx < ll then char_at left idx
    else char_at right (idx - ll)

let rec to_string = function
  | Leaf s -> s
  | Node (l, r, _) -> to_string l ^ to_string r  (* O(n) due to ^ *)
```

OCaml's `^` is string concatenation — O(n) for the concat step. The rope type delays this cost: `concat` in OCaml is O(1) just like in Rust.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Recursive enum | `Box<Rope>` in variants (required) | Direct recursion (GC handles indirection) |
| Concat | O(1) — allocates one `Node` | O(1) — allocates one `Node` record |
| `to_string` | O(n) — `push_str` into buffer | O(n) — `^` for each concat |
| `char_at` | O(log n) for balanced tree | O(log n) same |

Ropes are valuable for large text editors where users insert and delete characters frequently. For strings under ~1KB, plain `String` is faster due to CPU cache effects.

## Exercises

1. Implement `split(rope, pos) -> (Rope, Rope)` that splits the rope at character index `pos` in O(log n).
2. Implement `insert(rope, pos, s)` that inserts string `s` at position `pos` using split + concat.
3. Implement `delete(rope, l, r)` that removes characters in range `[l, r)` using split + concat.
4. Implement rope rebalancing: when a rope's tree depth exceeds `2 * log2(len)`, flatten to a `Leaf` and rebuild.
5. Implement `iter_chars(&self) -> impl Iterator<Item=char>` that lazily yields characters without materializing the full string.
