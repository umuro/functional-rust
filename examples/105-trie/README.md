📖 **[View on hightechmind.io →](https://hightechmind.io/rust/105-trie)**

---

# 105-trie — Trie (Prefix Tree)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A trie (from re-trie-val, sometimes called a prefix tree) stores strings with shared prefixes, enabling O(|key|) insertion and lookup regardless of how many strings are stored. It is the data structure behind autocomplete, IP routing (longest-prefix match), spell checkers, and dictionary compression.

Unlike a hash map, a trie naturally supports prefix queries: "find all strings starting with `foo`" requires no additional structure.

## Learning Outcomes

- Implement a trie using `HashMap<char, Trie>` for children
- Insert and search strings in O(|key|) time
- Perform prefix search to list all completions
- Compare `HashMap`-backed vs `BTreeMap`-backed trie (sorted output)
- Understand the memory trade-offs of trie vs hash map for string keys

## Rust Application

`src/lib.rs` implements `Trie` with `is_word: bool` and `children: HashMap<char, Trie>`. `insert` walks the trie character by character, creating nodes as needed. `contains` walks the trie and checks `is_word` at the terminal node. `starts_with` checks whether any string in the trie has the given prefix. A `BTreeMap`-based variant provides sorted prefix iteration.

Tries are the backbone of `fst` (finite state transducer) crate used in `tantivy` search, and of the Linux kernel's radix tree for memory address lookup.

## OCaml Approach

OCaml's trie uses a map for children:

```ocaml
module CharMap = Map.Make(Char)

type trie = { is_word: bool; children: trie CharMap.t }

let empty = { is_word = false; children = CharMap.empty }

let insert word trie =
  let rec go i node =
    if i = String.length word then { node with is_word = true }
    else
      let c = word.[i] in
      let child = try CharMap.find c node.children with Not_found -> empty in
      let new_child = go (i + 1) child in
      { node with children = CharMap.add c new_child node.children }
  in
  go 0 trie
```

OCaml's persistent map makes the trie persistent — each insert returns a new trie with structural sharing.

## Key Differences

1. **Mutability**: Rust's `Trie` is mutable (insert mutates in place via `or_default()`); OCaml's trie is persistent.
2. **Children map type**: Rust offers both `HashMap<char, Trie>` (O(1) lookup, unsorted) and `BTreeMap<char, Trie>` (sorted, like OCaml's `Map.Make(Char)`).
3. **Default initialization**: Rust's `or_default()` inserts a new empty `Trie` node if the character is not present; OCaml uses `Not_found` exception handling.
4. **Memory layout**: Rust's `Box<Trie>` inside `HashMap` requires heap allocation per node; OCaml's GC manages this automatically.

## Exercises

1. Implement `all_words(&self) -> Vec<String>` that returns all strings stored in the trie in alphabetical order.
2. Write `delete(&mut self, word: &str) -> bool` that removes a word from the trie, cleaning up unused nodes.
3. Implement `longest_common_prefix(words: &[&str]) -> String` using a trie to find the longest prefix shared by all words.
