**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

[trie-map on hightechmind.io](https://hightechmind.io/posts/functional-rust/trie-map)

---

## Problem Statement

Implement a trie (prefix tree) that maps string keys to values of type `V`. Each `TrieNode` contains an optional value and a `HashMap<char, TrieNode<V>>` of children. Operations include insert, get, contains, prefix search, and collecting all key-value pairs. This demonstrates recursive data structures, self-referential ownership, and generic types.

## Learning Outcomes

- Implement `TrieNode<V>` as a recursive struct: `value: Option<V>`, `children: HashMap<char, TrieNode<V>>`
- Navigate the trie character by character with `node.children.entry(c).or_default()`
- Implement `get<'a>(&'a self, key: &str) -> Option<&'a V>` using a fold over characters
- Implement `starts_with(prefix: &str) -> bool` for O(prefix_length) prefix queries
- Collect all entries with a depth-first walk that accumulates the current path prefix

## Rust Application

```rust
pub struct TrieNode<V> {
    value: Option<V>,
    children: HashMap<char, TrieNode<V>>,
}

pub struct Trie<V> {
    root: TrieNode<V>,
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Trie { root: TrieNode { value: None, children: HashMap::new() } }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.children.entry(c).or_default();
        }
        node.value = Some(value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let mut node = &self.root;
        for c in key.chars() {
            node = node.children.get(&c)?;
        }
        node.value.as_ref()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}
```

`entry(c).or_default()` inserts a new empty node if the character child does not exist, returning a mutable reference to the child. This "insert or get" pattern avoids double lookup.

The `?` operator in `get` short-circuits the traversal as soon as any character is not found in the children map. If the full key is traversed successfully, `node.value.as_ref()` returns the optional stored value.

`#[derive(Default)]` on `TrieNode` enables `or_default()` to construct empty nodes without an explicit `TrieNode::new()` call.

## OCaml Approach

```ocaml
module CharMap = Map.Make(Char)

type 'v trie_node = {
  value: 'v option;
  children: 'v trie_node CharMap.t;
}

let empty_node = { value = None; children = CharMap.empty }

let rec insert key value node =
  match String.length key with
  | 0 -> { node with value = Some value }
  | _ ->
    let c = key.[0] in
    let rest = String.sub key 1 (String.length key - 1) in
    let child = try CharMap.find c node.children with Not_found -> empty_node in
    let new_child = insert rest value child in
    { node with children = CharMap.add c new_child node.children }

let rec get key node =
  match String.length key with
  | 0 -> node.value
  | _ ->
    let c = key.[0] in
    let rest = String.sub key 1 (String.length key - 1) in
    match CharMap.find_opt c node.children with
    | None -> None
    | Some child -> get rest child
```

OCaml's trie uses an immutable `CharMap` for children — each insert creates new nodes along the path (persistent data structure). Rust's version uses `HashMap` for O(1) average child lookup, with mutable in-place updates.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Children map | `HashMap<char, TrieNode<V>>` — O(1) avg | `Map.Make(Char)` — O(log n) balanced BST |
| Insert style | Mutable traversal with `entry().or_default()` | Functional recursion producing new nodes |
| Persistence | Mutable — single copy | Persistent — structural sharing |
| Self-reference | Box not needed (HashMap owns children directly) | Immutable records with functional update |

Tries are efficient for prefix queries and autocomplete. The `HashMap` child map is O(1) average per character; `Map.Make(Char)` is O(log 26) = O(1) in practice since the alphabet is bounded.

## Exercises

1. Implement `collect_all(&self) -> Vec<(String, &V)>` that returns all stored key-value pairs via depth-first traversal.
2. Implement `delete(&mut self, key: &str) -> bool` that removes a key and prunes now-empty nodes.
3. Implement `autocomplete(&self, prefix: &str) -> Vec<String>` that returns all keys starting with the prefix.
4. Add a `longest_prefix<'a>(&self, s: &'a str) -> &'a str` method that returns the longest prefix of `s` that is stored as a key.
5. Implement a compressed trie (radix tree) where common prefixes are merged into single edges.
