# 373: Custom B-Tree Implementation

**Difficulty:** 5  **Level:** Master

Build a B-tree from scratch — the self-balancing multi-key tree that powers every database index and filesystem.

## The Problem This Solves

A binary search tree degenerates to O(n) when keys arrive in sorted order. AVL and red-black trees fix this with rotations, but they still store one key per node. For disk-based storage this is catastrophic: each key-lookup requires a separate disk seek. A B-tree packs 2t-1 keys per node, keeping the tree short and wide — perfect for sequential disk reads.

Even in memory, a B-tree's cache locality beats pointer-chasing binary trees. Rust's standard `BTreeMap` is itself a B-tree. Building one from scratch teaches you what that means: node splitting, degree invariants, and the key-per-node tradeoff.

The minimum degree `t` is a tuneable parameter: larger `t` means fewer levels (fewer I/O seeks) but more work per node scan. Database engines set `t` based on their page size.

## The Intuition

Every node is a sorted array of keys with child pointers between them. A key `k` at position `i` means: everything in `children[i]` is less than `k`, and everything in `children[i+1]` is greater. Finding a key is binary search within a node, then follow the right pointer.

When a node overflows (hits `2t-1` keys), it splits: the middle key bubbles up to the parent, and the node becomes two. This is the only way the tree grows, keeping all leaves at the same depth.

## How It Works in Rust

1. **`BTreeNode`** — holds `keys: Vec<i32>`, `children: Vec<Box<BTreeNode>>`, and `is_leaf: bool`.
2. **`const T: usize = 2`** — minimum degree. Max keys per node = `2*T - 1 = 3`. Min keys = `T - 1 = 1`.
3. **`search`** — `partition_point` finds the insertion index; if `keys[i] == key` we found it; if leaf, not found; else recurse into `children[i]`.
4. **`insert`** — if root is full, create a new root and split the old one. Then `insert_non_full` recurses, splitting children proactively.
5. **`split_child`** — finds the median key, promotes it to the parent, divides the full node into two half-full nodes.

```rust
// Proactive split: split full children before descending
if node.children[i].is_full() {
    split_child(node, i);
}
insert_non_full(&mut node.children[i], key);
```

## What This Unlocks

- **Database index internals** — PostgreSQL, SQLite, and MySQL all use B-tree variants for their primary indexes.
- **Tunable branching** — change `T` and watch how tree height (and I/O seeks) scale.
- **Foundation for B+ trees** — the next step: store data only in leaves, link leaves for range scans.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| B-tree in stdlib | Not provided | `BTreeMap` (built-in B-tree) |
| Node ownership | GC-managed pointers | `Box<BTreeNode>` — owned heap nodes |
| Node degree | Fixed branching | Const `T`: 2t-1 keys max per node |
| Split strategy | Lazy or eager | Proactive (split on way down) |
| Sorted search | `compare` function | `partition_point` (binary search) |
