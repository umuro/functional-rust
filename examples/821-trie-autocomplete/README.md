📖 **[View on hightechmind.io →](https://hightechmind.io/rust/821-trie-autocomplete)**

---

# Trie Autocomplete
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Autocomplete and prefix search are fundamental to user interfaces: search bars, IDE completion, spell checkers, and IP routing tables all need to answer "what words start with this prefix?" efficiently. A hash map of all words answers exact lookups in O(1) but cannot answer prefix queries. A trie (prefix tree) organizes strings by shared prefixes, enabling prefix queries in O(m) time where m is the prefix length — independent of how many words are stored. This also makes insertion, deletion, and "does any word start with X?" all O(m). Tries power the autocomplete in Google Search, the routing table lookup in routers, and the dictionary in word games.

## Learning Outcomes

- Build a trie where each node maps characters to children and marks word endings
- Implement insert in O(m) by traversing/creating nodes character by character
- Implement prefix search by traversing to the prefix endpoint then collecting all completions via DFS
- Understand memory tradeoffs: tries use more memory than hash maps but enable prefix operations
- Recognize compressed tries (patricia/radix tries) as space optimization for sparse tries

## Rust Application

```rust
#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}
impl TrieNode {
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }
}
```

`HashMap<char, TrieNode>` at each node handles arbitrary Unicode characters. The `entry().or_default()` pattern creates missing nodes lazily — idiomatic Rust that avoids double lookup. The `is_end` boolean flags complete words vs. internal nodes. Autocomplete starts at the prefix endpoint and does DFS collecting words; this naturally extends with `Vec<String>` accumulation and a `prefix` string parameter that grows during descent. Rust's ownership prevents accidental aliasing between nodes, ensuring the tree structure stays valid.

## OCaml Approach

OCaml represents trie nodes as `{ children: (char * node) list; is_end: bool }` or with a `Hashtbl`. Functional insertion creates a new node path, relying on structural sharing for efficiency. With mutable `Hashtbl`, insertion mutates like Rust's approach. OCaml's `Map.Make(Char)` creates a balanced BST per node for sorted child iteration — convenient for alphabetical autocomplete results. The DFS for completions uses continuation-passing or an accumulator list. OCaml's algebraic types make `is_end` a natural `bool` field.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Node storage | `HashMap<char, TrieNode>` | `Hashtbl` or `Map.Make(Char).t` |
| Insertion | Mutable traversal with `entry()` | Mutable `Hashtbl` or immutable path copy |
| Completions DFS | Recursive with `&mut Vec<String>` | Accumulator list or `Buffer` |
| Memory | Each node heap-allocated | GC-managed, similar cost |
| Sorted output | Extra sort step | `Map` gives sorted iteration free |
| End marker | `bool` field | `bool` field or `unit option` |

## Exercises

1. Implement `delete(word)` that removes a word while preserving words that share its prefix.
2. Add a frequency count per word and implement autocomplete returning the top-k most frequent completions.
3. Implement fuzzy search: return all words within Levenshtein distance 1 from a query.
4. Build a compressed radix trie (patricia trie) that merges single-child chains into single edges.
5. Measure memory usage of trie vs. sorted `Vec<String>` + binary search for a dictionary of 100k words.
