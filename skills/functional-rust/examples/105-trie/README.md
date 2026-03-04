# 105: Trie — Prefix Tree for Strings

**Difficulty:** 3  **Level:** Intermediate

Store strings efficiently by sharing common prefixes — the data structure behind autocomplete and spell-checkers.

## The Problem This Solves

A dictionary of 100,000 words starting with "pre" — "prefix", "prevent", "preview", "previous" — shares those three characters in every word. A trie stores them once. This makes prefix lookup fast: to find all words starting with "pre", you follow three edges and explore from there.

Tries power autocomplete dropdowns: type "pre", follow three edges, collect all words reachable from that node. They're used in routers (IP prefix matching), spell-checkers (word + prefix suggestions), and command-line completion.

The core structure: each node has a flag (is this a complete word?) and a map of children, one per character.

## The Intuition

Imagine a tree where each edge is a letter. To insert "cat", you create three edges: `c → a → t`, and mark the `t` node as a word endpoint. To insert "car", you follow the existing `c → a`, then create a new `r` edge. The `ca` prefix is shared.

`HashMap<char, Trie>` is the natural Rust structure: fast (O(1) per character) and supports any Unicode character. `BTreeMap<char, Trie>` gives sorted traversal (like OCaml's `Map.Make(Char)`). An array `[Option<Box<Trie>>; 26]` is fastest for ASCII-only text.

## How It Works in Rust

```rust
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            // entry().or_default() creates a new empty Trie node if missing
            node = node.children.entry(c).or_default();
        }
        node.is_word = true;  // mark the final node as a complete word
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,  // prefix not found
            }
        }
        node.is_word  // true only if this is a complete word, not just a prefix
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true  // reached the end of prefix — exists in the trie
    }
}
```

Note: `contains("ca")` returns `false` even if "cat" and "car" are inserted — `ca` is a prefix, not a word. `starts_with("ca")` returns `true`.

## What This Unlocks

- **Autocomplete** — given a prefix, collect all reachable words with DFS from the prefix node
- **Spell-checker** — fast exact lookup + prefix suggestions for "did you mean?"
- **IP routing** — longest prefix match for routing tables

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Children map | `Map.Make(Char)` functor → immutable sorted map | `HashMap<char, Trie>` (fast) or `BTreeMap` (sorted) |
| Mutability | Immutable — insert returns a new trie | Mutable — `insert(&mut self, ...)` modifies in place |
| Create-or-insert | Recursive rebuild on each path | `entry().or_default()` — in-place |
| Memory layout | Heap-allocated tree nodes | Same — each `Trie` owns its children |
| ASCII optimization | N/A | `[Option<Box<Trie>>; 26]` array — avoids map overhead |
