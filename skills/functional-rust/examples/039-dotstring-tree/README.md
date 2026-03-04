# 039: Dotstring Tree

**Difficulty:** ⭐⭐  **Level:** Foundations

Serialize a binary tree to a compact "dotstring" format using preorder traversal, where `.` represents an empty node.

## The Problem This Solves

Example 036 used a parenthetical format `a(b,c)` that's readable but verbose. The dotstring format is more compact: do a preorder traversal, emit each node's value, and emit `.` wherever there's an empty subtree.

For example, a single node `x` becomes `"x.."` — the node value, then `.` for its missing left child, then `.` for its missing right child. The tree `a(b(d,e),c(,f))` becomes `"abd..e..c.f.."`.

This format is widely used in competitive programming and algorithm courses because it's minimal, unambiguous, and can be parsed in a single left-to-right pass. Any tree becomes a string; any valid string decodes to exactly one tree.

## The Intuition

A dotstring encodes the *structure* of the tree as dots. Read it as instructions: "create a node with this value, then build its left subtree, then its right subtree — unless you see a dot, which means empty."

Compare to JSON:
```json
{"val": "a", "left": {"val": "b", ...}, "right": null}
```

The dotstring equivalent strips all the keywords and just encodes the shape: `"ab..."`

**Serialization** is simple preorder:
```
tree_to_dotstring(Leaf)      = "."
tree_to_dotstring(Node(l,v,r)) = v + tree_to_dotstring(l) + tree_to_dotstring(r)
```

**Parsing** is the same preorder, but reading characters instead of building them. The trick: parsing is naturally recursive. Each call to `parse` consumes exactly one subtree from the string.

## How It Works in Rust

```rust
fn tree_to_dotstring(tree: &Tree) -> String {
    match tree {
        Tree::Leaf => ".".to_string(),          // empty → dot
        Tree::Node(l, v, r) => {
            format!("{}{}{}", v,                 // root value
                tree_to_dotstring(l),            // left subtree
                tree_to_dotstring(r))            // right subtree
        }
    }
}

fn parse(chars: &[char], pos: usize) -> (Tree, usize) {
    if pos >= chars.len() {
        return (Tree::leaf(), pos);
    }
    match chars[pos] {
        '.' => (Tree::leaf(), pos + 1),   // dot → Leaf, advance 1

        c => {
            // Node: consume left subtree, then right subtree
            let (left, pos2) = parse(chars, pos + 1);
            let (right, pos3) = parse(chars, pos2);
            (Tree::node(left, c, right), pos3)
        }
    }
}
```

Notice the parser returns a `(Tree, usize)` — the parsed tree *and* the new position. This is a functional approach to the mutable-position-pointer pattern from example 036. Each call knows exactly how many characters it consumed.

**Tracing `"abd..e..c.f.."` through the parser:**
```
pos=0: 'a' → node; recurse left from pos=1
  pos=1: 'b' → node; recurse left from pos=2
    pos=2: 'd' → node; left=pos=3: '.', right=pos=4: '.' → Node('d',Leaf,Leaf), pos=5
  pos=5: 'e' → node; left='.', right='.' → Node('e',Leaf,Leaf), pos=8
  → Node('b', d, e), pos=8
pos=8: 'c' → node; recurse left from pos=9
  pos=9: '.' → Leaf, pos=10
  pos=10: 'f' → node; '.','.' → Node('f',Leaf,Leaf), pos=13
  → Node('c', Leaf, f), pos=13
pos=13: '.' → Leaf (consumed as right of 'a')
→ Node('a', b-subtree, c-subtree), pos=14 (end)
```

## What This Unlocks

- **Compact serialization**: dotstrings are more space-efficient than parenthetical formats for dense trees.
- **Streaming parsers**: the single-pass left-to-right parse is easy to adapt for streaming input.
- **Competitive programming**: dotstrings are a standard encoding challenge — you'll see them in coding interviews.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dot-string serialization | Preorder: `v ^ to_ds l ^ to_ds r` | `format!("{}{}{}", v, ds(l), ds(r))` |
| Parser return type | Tuple `(tree * int)` | Tuple `(Tree, usize)` |
| Character match | `match c with '.' -> ...` | `match chars[pos] { '.' => ..., c => ... }` |
| Vs parenthetical format (036) | More compact, no delimiters needed | Same — dots encode structure implicitly |
