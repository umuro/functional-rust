📖 **[View on hightechmind.io →](https://hightechmind.io/rust/821-trie-autocomplete)**

---

# 821: Trie for Autocomplete and Prefix Search

**Difficulty:** 3  **Level:** Intermediate

Build a prefix tree for O(|prefix|) insert, lookup, and prefix enumeration — the backbone of autocomplete engines.

## The Problem This Solves

A trie (prefix tree) stores strings so that all strings sharing a common prefix share the same path from the root. This makes prefix queries — "find all words starting with `com`" — cost O(|prefix| + number of results) rather than O(n × |word|) for a linear scan.

Use tries for autocomplete (type-ahead search), IP routing tables (longest prefix match), spell-check suggestion, and dictionary compression. Any system where you need fast prefix lookup over a set of strings benefits from a trie. Unlike a hash map, a trie can enumerate all words with a given prefix efficiently and supports ordered iteration naturally.

This implementation supports `insert`, exact `search`, `starts_with` (prefix existence), and `autocomplete` (enumerate all completions for a prefix). It stores complete words at terminal nodes.

## The Intuition

Each node represents a character position. The path from the root to a node spells out a prefix. Each node has a map from characters to child nodes, and a flag indicating whether this node ends a complete word.

Insert: walk character by character, creating nodes as needed. Mark the final node as a word ending.
Search: walk character by character; if any step has no matching child, the word isn't present.
Autocomplete: find the node for the prefix, then DFS the subtree collecting all paths to word-ending nodes.

O(|word|) for insert/search. O(|prefix| + total_matching_chars) for autocomplete. Space: O(total chars across all words) in the worst case, but shared prefixes save memory proportionally.

In OCaml, you'd define a recursive type `type node = { children: (char, node) Hashtbl.t; is_end: bool }`. In Rust, `HashMap<char, Box<TrieNode>>` achieves the same — boxed children to handle the recursive type with known size.

## How It Works in Rust

```rust
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

struct Trie { root: TrieNode }

impl Trie {
    fn new() -> Self { Trie { root: TrieNode::default() } }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            // entry API: insert if absent, then navigate into child
            node = node.children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::default()));
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true // reached end of prefix — it exists
    }

    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        // Navigate to prefix node
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return vec![], // prefix not in trie
            }
        }
        // DFS the subtree, collecting completions
        let mut results = vec![];
        Self::collect(node, &mut prefix.to_string(), &mut results);
        results
    }

    fn collect(node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_end { results.push(current.clone()); }
        for (&ch, child) in &node.children {
            current.push(ch);
            Self::collect(child, current, results);
            current.pop(); // backtrack
        }
    }
}
```

The `entry().or_insert_with()` pattern is idiomatic Rust for "insert-if-absent then get reference." It avoids a double lookup compared to `get` + `insert`. The recursive `collect` uses a mutable `current` string with `push`/`pop` — the same backtracking pattern as tree enumeration.

`Box<TrieNode>` is required because the recursive type would otherwise be infinite size. The compiler cannot determine `TrieNode`'s stack size without the indirection.

## What This Unlocks

- **Search engine autocomplete**: index all query suggestions in a trie; retrieve completions in O(prefix length + results).
- **IP routing (longest prefix match)**: routers use binary tries to find the most specific matching route for a destination IP.
- **Spell checker suggestions**: walk the trie with edit-distance tolerance to find near-matches for misspelled words.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive node type | `type node = { ... }` (size known by GC) | `Box<TrieNode>` — explicit indirection for recursive type |
| Child map | `Hashtbl.t` or `Map.Make(Char)` | `HashMap<char, Box<TrieNode>>` |
| Insert idiom | Pattern match + `Hashtbl.replace` | `entry().or_insert_with()` — single lookup |
| Autocomplete DFS | Recursive with accumulator | Mutable `current: &mut String` + `push`/`pop` backtrack |
| Ownership of subtree | GC-managed | `Box` owns child; parent owns `Box` via `HashMap` value |
