# 374: Radix Tree / Patricia Trie

**Difficulty:** 4  **Level:** Expert

Compress a trie by merging single-child chains into multi-character edge labels — the data structure behind IP routing and autocomplete.

## The Problem This Solves

A standard trie stores one character per node. For a dictionary of English words, most internal nodes have exactly one child — a long chain spelling out a common prefix like "inter-". This wastes memory and makes lookups traverse many nodes for no branching benefit.

A Radix tree (also called Patricia Trie or Compressed Trie) solves this by collapsing single-child chains into a single edge with a multi-character label. "international" and "internet" share the prefix "intern" — that prefix becomes one edge label, and only the node after it branches.

This structure is ideal for routing tables (IP prefixes), DNS lookups, and filesystem path trees. Real-world routers use Patricia Tries because longest-prefix matching is O(key length) regardless of table size.

## The Intuition

Imagine a filing cabinet where folders can have multi-word labels. Instead of one folder per letter ("i", then "n", then "t"...), you have one folder labeled "inter" with two sub-folders "national" and "net". Lookup: find the folder whose label is a prefix of your search word, then recurse into it with the remaining characters.

When inserting a new word that shares only part of an existing edge label, you split the edge: create an intermediate node for the common prefix, with two children for the differing suffixes.

## How It Works in Rust

1. **`RadixNode`** — `children: HashMap<String, RadixNode>` maps edge labels to child nodes. `is_end: bool` marks complete words.
2. **`insert_node`** — scan children for a matching edge (one that is a prefix of `remaining`, or vice versa):
   - **Full match**: edge is a prefix of `remaining` → recurse with the rest.
   - **Partial match**: find the common prefix length, split the edge, create an intermediate node.
   - **No match**: insert a new edge with `remaining` as the label.
3. **`search_node`** — for each child edge, check if `remaining` starts with it; if so, recurse with the suffix.
4. **`starts_with`** — similar, but succeeds as soon as `remaining` is empty (prefix matched) or an edge starts with `remaining`.

```rust
// Split an edge: "card" → insert "care"
// common = "car", edge_rest = "d", word_rest = "e"
let mut new_node = RadixNode::new();
new_node.children.insert(edge_rest, old_child); // "d" → old "card" subtree
new_node.children.insert(word_rest, word_leaf);  // "e" → new "care" leaf
node.children.insert(common, new_node);           // "car" → split node
```

## What This Unlocks

- **IP routing** — longest-prefix match on CIDR prefixes; O(32) steps regardless of table size.
- **Autocomplete** — `starts_with("inter")` finds all completions in one traversal.
- **Efficient prefix APIs** — `search` (exact) and `starts_with` (prefix) are both O(key length).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Trie node | Recursive ADT | `struct RadixNode` with owned fields |
| Edge labels | Single `char` (standard trie) | `String` (multi-char labels) |
| Children map | `Map.Make(Char)` | `HashMap<String, RadixNode>` |
| Edge splitting | Explicit recursive case | `remove` → build intermediate → `insert` |
| Prefix search | Walk tree, check `is_end` | Same, short-circuit on empty `remaining` |
