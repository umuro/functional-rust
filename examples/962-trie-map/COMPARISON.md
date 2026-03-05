# Trie Map — Comparison

## Core Insight
A trie stores strings by sharing common prefixes — each node is a character + optional value + map of children. The algorithm is identical in both languages. OCaml uses a functional `Map.Make(Char)` (immutable BST) or mutable fields; Rust uses `HashMap<char, TrieNode<V>>` with `entry().or_insert_with()` for clean "get or create" logic.

## OCaml Approach
- `module CharMap = Map.Make(Char)` — ordered map over chars
- Mutable fields (`mutable value`, `mutable children`) on a record
- Imperative traversal with `ref` node pointer
- `String.iter (fun c -> ...)` to walk characters
- `CharMap.find_opt` / `CharMap.add` for children

## Rust Approach
- `HashMap<char, TrieNode<V>>` for children (O(1) vs O(log n) for BST)
- `node.children.entry(c).or_insert_with(TrieNode::default)` — get-or-create idiom
- `for c in key.chars()` — Unicode-safe char iteration
- `let mut node = &mut self.root` — mutable reference traversal
- Generic over `V` with `Default` bound for empty nodes
- `collect_keys` recursive DFS for prefix enumeration

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Children map | `CharMap.t` (balanced BST) | `HashMap<char, TrieNode<V>>` |
| Get-or-create child | `match find_opt c children with None -> create` | `entry(c).or_insert_with(Default::default)` |
| Node pointer | `let node = ref trie` | `let mut node = &mut self.root` |
| Char iteration | `String.iter (fun c -> ...)` | `for c in key.chars()` |
| Value storage | `mutable value: 'a option` | `value: Option<V>` |
| Lookup result | `nd.value` (option) | `node.value.as_ref()` |
| Prefix DFS | Manual recursion | Recursive helper with `&mut String` |
