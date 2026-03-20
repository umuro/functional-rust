📖 **[View on hightechmind.io →](https://hightechmind.io/rust/362-trie-structure)**

---

# 362: Trie Structure

## Problem Statement

Hash maps give O(1) exact key lookup but can't answer prefix queries: "list all words starting with 'pre'" requires scanning all keys. A trie (retrieval tree, Fredkin 1960) stores strings by decomposing them into characters — each node represents one character, paths from root to end-marked nodes spell out stored words. Lookup is O(m) where m is the key length, independent of how many words are stored. Tries power autocomplete in IDEs and search engines, IP routing tables (compact trie on bit prefixes), spell checkers, and dictionary compression (compressed tries / DAWG). They're the data structure behind DNS resolution caches.

## Learning Outcomes

- Implement a `TrieNode` with `children: HashMap<char, TrieNode>` and `is_end: bool`
- Insert a word by traversing/creating one node per character
- Search for exact words by traversing the path and checking `is_end`
- Find all words with a given prefix using recursive subtree traversal (DFS)
- Understand why lookup is O(m) regardless of vocabulary size
- Compare trie prefix queries to linear scan over a sorted list

## Rust Application

```rust
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self { Self { root: TrieNode::default() } }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true // any path exists with this prefix
    }

    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        // navigate to prefix node, then DFS collect all end nodes
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return vec![],
            }
        }
        let mut results = Vec::new();
        collect_words(node, prefix.to_string(), &mut results);
        results
    }
}

fn collect_words(node: &TrieNode, prefix: String, results: &mut Vec<String>) {
    if node.is_end { results.push(prefix.clone()); }
    for (c, child) in &node.children {
        collect_words(child, format!("{}{}", prefix, c), results);
    }
}
```

`or_default()` creates an empty `TrieNode` on the fly — the entry API handles both traversal and creation in one statement. Prefix completion is a two-phase operation: navigate to the prefix tip (O(m)), then DFS-collect all words below (O(count × average_length)).

## OCaml Approach

OCaml's recursive types with `Map` children:

```ocaml
module CharMap = Map.Make(Char)

type trie = {
  children: trie CharMap.t;
  is_end: bool;
}

let empty = { children = CharMap.empty; is_end = false }

let rec insert node = function
  | [] -> { node with is_end = true }
  | c :: rest ->
    let child = try CharMap.find c node.children with Not_found -> empty in
    let updated = insert child rest in
    { node with children = CharMap.add c updated node.children }

let insert_word trie word =
  insert trie (List.of_seq (String.to_seq word))
```

OCaml's functional trie returns new nodes on insert (persistent). `CharMap` replaces `HashMap<char, TrieNode>` with a functional balanced BST. The structure is immutable — each `insert` returns a new trie sharing unchanged subtrees with the original.

## Key Differences

| Aspect | Rust `HashMap<char, TrieNode>` | OCaml `CharMap.t trie` |
|--------|-------------------------------|------------------------|
| Children map | `HashMap` (O(1) lookup) | `Map.Make(Char)` (O(log σ)) |
| Mutability | In-place (`&mut self`) | Persistent (new trie per insert) |
| Alphabet size | Any (unicode chars) | Any (via functor) |
| Memory sharing | None — copy on clone | Structural sharing |
| Compact trie | Manual or `radix-trie` crate | Manual implementation |

## Exercises

1. **Delete a word**: Implement `remove(&mut self, word: &str)` that marks `is_end = false` for the word's terminal node; also prune empty subtrees (nodes with no children and `is_end = false`).
2. **Longest common prefix**: Given a list of words inserted into a trie, find the longest common prefix by traversing the root while each node has exactly one child and `is_end = false`.
3. **Compressed trie**: Implement a Patricia trie where edge labels are substrings (not single chars), reducing the number of nodes from O(total_chars) to O(words) for strings with long common prefixes.
