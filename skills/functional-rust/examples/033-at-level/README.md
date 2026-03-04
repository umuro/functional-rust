# 033: At Level

**Difficulty:** ⭐  **Level:** Foundations

Collect all node values at a given depth level of the tree (root = level 1).

## The Problem This Solves

Sometimes you don't want to traverse an entire tree — you want a specific *row*. Web scrapers often navigate HTML to a specific nesting depth. Game AI evaluates board states at a fixed lookahead depth. Breadth-first search processes one level at a time.

`at_level(tree, n)` is the targeted version: give me everything exactly `n` hops from the root. Combined with a loop, you get a level-order (breadth-first) traversal — one of the most important tree algorithms.

This example teaches a key recursive pattern: pass a counter down through the recursion, decrementing on each level. When it hits 1, you've arrived.

## The Intuition

Imagine you're descending a building by stairs. You start at level 1 (the root). Each time you take a step down, the level counter decrements. When it hits 1, you collect that node's value and stop descending.

In Python:
```python
def at_level(node, level):
    if node is None: return []
    if level == 1: return [node.val]
    return at_level(node.left, level - 1) + at_level(node.right, level - 1)
```

The Rust version is identical in structure — pattern matching handles the two base cases cleanly:

```rust
Tree::Leaf => vec![],        // no node here
Tree::Node(...) if level == 1 => vec![v.clone()]  // arrived
```

## How It Works in Rust

```rust
fn at_level<T: Clone>(tree: &Tree<T>, level: usize) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],  // ran off the edge

        Tree::Node(l, v, r) => {
            if level == 1 {
                vec![v.clone()]  // arrived at target depth
            } else {
                // Go one level deeper in both subtrees
                let mut result = at_level(l, level - 1);
                result.extend(at_level(r, level - 1));
                result
            }
        }
    }
}
```

For the sample tree:
```
       a     ← level 1
      / \
     b   c   ← level 2
    / \
   d   e     ← level 3
```
- `at_level(&t, 1)` → `['a']`
- `at_level(&t, 2)` → `['b', 'c']`
- `at_level(&t, 3)` → `['d', 'e']`
- `at_level(&t, 4)` → `[]`

The full breadth-first traversal collects all levels using a queue — the iterative `levels()` function in the code shows this pattern.

## What This Unlocks

- **BFS skeleton**: loop over `at_level(t, 1)`, `at_level(t, 2)`, ... for full level-order traversal.
- **Game AI**: evaluate all positions at a fixed search depth (minimax, alpha-beta pruning).
- **Tree visualization**: render one row at a time for console or UI display.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Depth counter | `at_level tree n` decrement n | `at_level(tree, level)` decrement level |
| Base case | `Leaf -> []` | `Tree::Leaf => vec![]` |
| BFS via queue | `Queue.push` | `Vec` as queue (`push`/`remove(0)`) or `VecDeque` |
| `usize` underflow | N/A (int) | Use `usize` — safe since `level == 1` stops before `0` |
