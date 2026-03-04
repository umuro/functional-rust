# Example 269: Trie Prefix Tree

**Difficulty:** ⭐⭐⭐
**Category:** Data Structures | Trees | String Algorithms
**OCaml Source:** Classic functional data structures — persistent trie with `Map.Make(Char)`

## Problem Statement

Build a prefix tree (trie) that supports efficient word insertion and membership testing. The trie maps each character to a sub-trie, and nodes carry a flag marking complete words.

## Learning Outcomes

- How Rust's `HashMap`/`BTreeMap` replace OCaml's functor-parameterized `Map.Make`
- The mutable-vs-persistent trade-off: OCaml returns new nodes; Rust mutates in place
- Using `entry().or_default()` to insert-or-retrieve children idiomatically
- Nested recursive helper functions (`fn go`) inside public methods to mirror OCaml's `let rec go`

## OCaml Approach

OCaml uses `Map.Make(Char)` to get a character-keyed ordered map, then defines a record `{ is_word; children }`. Both `insert` and `mem` are purely functional — each call returns a structurally-shared copy of the trie rather than mutating any node in place. `List.fold_left` builds the trie from a list of words in a single expression.

## Rust Approach

Rust offers two natural styles. The idiomatic mutable version uses `HashMap<char, Trie>` and walks the tree with a `&mut self` reference, updating nodes in place with `entry().or_default()` — zero allocation overhead per lookup. The functional version mirrors OCaml exactly: `insert` consumes `self` and returns a new `FunctionalTrie`, using `BTreeMap` (ordered, like OCaml's balanced-tree map) and a nested helper `fn go` for recursive descent.

## Key Differences

1. **Map implementation:** OCaml `Map.Make(Char)` is a balanced BST; Rust uses `HashMap` (O(1) amortized) or `BTreeMap` (ordered, O(log n)).
2. **Mutability:** OCaml insert returns a new tree (persistent); idiomatic Rust mutates in place with `&mut self`.
3. **Structural sharing:** OCaml gets sharing for free via the GC; Rust would need `Rc`/`Arc` to share sub-tries across snapshots.
4. **Fold idiom:** OCaml `List.fold_left (fun t w -> insert w t)` maps directly to Rust `Iterator::fold(trie, |t, w| t.insert(w))`.
