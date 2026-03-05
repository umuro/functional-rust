📖 **[View on hightechmind.io →](https://hightechmind.io/rust/362-trie-structure)**

---

# 362: Trie — Prefix Tree for String Lookups

**Difficulty:** 3  **Level:** Advanced

O(m) lookup and prefix search where m is the key length — independent of how many keys are stored.

## The Problem This Solves

You're building autocomplete. Given a prefix like "rust", return all stored keys that start with "rust". With a `HashMap`, you'd iterate every key and check `key.starts_with(prefix)` — O(n·m) where n is the number of keys and m is the prefix length. That's a full scan every query.

A Trie (prefix tree) solves this by sharing common prefixes in the tree structure. "rust" and "rustacean" both start with "rust" — in a Trie, they share the nodes for r→u→s→t. To find all words with prefix "rust", you walk four nodes to reach the "rust" node, then collect everything below it. The cost is O(m) to navigate to the prefix, then O(k) to collect k results — completely independent of total keys stored.

The second use case is dictionary-style operations: check if a word exists (vs. just being a prefix of another word), count words by prefix, or delete a word without affecting words that share its prefix. All are O(m) and naturally expressed in the tree structure.

## The Intuition

There's no direct Python standard library equivalent. The closest is a nested dict: `{"r": {"u": {"s": {"t": {"_end": True, "a": ...}}}}}`. A Trie is exactly this — each node is a map from character to child node, with a flag marking whether the path to this node forms a complete word.

The tradeoff: a Trie uses more memory than a `HashMap<String, V>` for sparse key sets (each node is a struct), but enables prefix operations that a flat hash map simply can't do efficiently.

## How It Works in Rust

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool, // true if this node marks the end of an inserted key
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { children: HashMap::new(), is_end: false }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self { Trie { root: TrieNode::new() } }

    // O(m) where m = key length
    fn insert(&mut self, key: &str) {
        let mut node = &mut self.root;
        for ch in key.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_end = true; // mark end of this key
    }

    // O(m) — walk the tree, return true only if key was inserted
    fn contains(&self, key: &str) -> bool {
        let mut node = &self.root;
        for ch in key.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    // O(m) to reach prefix node, then O(k) to collect k results
    fn words_with_prefix<'a>(&'a self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return vec![], // prefix not found
            }
        }
        // Collect all words below this node
        let mut results = Vec::new();
        collect(node, &mut prefix.to_string(), &mut results);
        results
    }
}

fn collect(node: &TrieNode, current: &mut String, out: &mut Vec<String>) {
    if node.is_end { out.push(current.clone()); }
    for (&ch, child) in &node.children {
        current.push(ch);
        collect(child, current, out);
        current.pop();
    }
}

// Usage
let mut trie = Trie::new();
trie.insert("rust");
trie.insert("rustacean");
trie.insert("rusty");
trie.insert("ruby");

println!("{}", trie.contains("rust"));     // true
println!("{}", trie.contains("rus"));      // false (not inserted as a key)

let matches = trie.words_with_prefix("rust");
println!("{matches:?}"); // ["rust", "rustacean", "rusty"] (order depends on HashMap)
```

## What This Unlocks

- **Autocomplete and search suggestions**: walk to the prefix node in O(m), collect all completions below — the canonical use case.
- **Spell checking with suggestions**: find all words within edit distance 1 of a query by branching at each mismatch during traversal.
- **IP routing tables**: longest-prefix matching on binary keys (CIDR notation) is a Trie on bits — the actual algorithm used in router hardware.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Prefix tree | not in stdlib | custom `HashMap`-based Trie |
| Insert | recursive functional | iterative with `entry()` |
| Exact lookup | O(m) with custom impl | O(m) walk to end node |
| Prefix search | O(n·m) with flat map | O(m + k) with Trie |
| Memory | cons-cell sharing possible | `HashMap` per node (higher overhead) |
| Alternative | `Map` with string keys | `HashMap<String, V>` (no prefix ops) |
