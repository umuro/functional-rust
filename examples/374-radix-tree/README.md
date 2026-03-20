📖 **[View on hightechmind.io →](https://hightechmind.io/rust/374-radix-tree)**

---

# 374: Radix Tree (Compressed Trie)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Standard tries use one node per character — a word like "programming" requires 11 nodes. When many keys share long common prefixes (URLs, file paths, IP addresses), most trie nodes have exactly one child and waste memory. A radix tree (Patricia trie, compressed trie) collapses chains of single-child nodes into a single edge with a multi-character label. The word "programming" and "program" share a node labeled "program" with two children: one for "" (end) and one for "ming". This compression can reduce node count from O(total_chars) to O(words) for typical key sets. Radix trees power IP routing (longest prefix match), HTTP router matching, and autocomplete in shells.

## Learning Outcomes

- Implement a `RadixNode` with `children: HashMap<String, RadixNode>` (multi-char edge labels)
- Insert by finding the longest common prefix with an existing edge label
- Split an edge when the new word shares only part of an existing label
- Search by matching prefixes of the remaining string against edge labels
- Find all words sharing a prefix by traversing the subtree below the prefix node
- Compare node count between a standard trie and a radix tree for the same word set

## Rust Application

```rust
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RadixNode {
    children: HashMap<String, RadixNode>,
    is_end: bool,
}

pub struct RadixTree {
    root: RadixNode,
}

impl RadixTree {
    pub fn insert(&mut self, word: &str) {
        Self::insert_node(&mut self.root, word);
    }

    fn insert_node(node: &mut RadixNode, remaining: &str) {
        if remaining.is_empty() { node.is_end = true; return; }

        // Find an existing edge that shares a prefix with remaining
        if let Some(key) = node.children.keys()
            .find(|k| k.starts_with(remaining.chars().next().unwrap()))
            .cloned()
        {
            let common_len = common_prefix_len(&key, remaining);
            if common_len == key.len() {
                // remaining extends beyond key — recurse into child
                Self::insert_node(
                    node.children.get_mut(&key).unwrap(),
                    &remaining[common_len..]
                );
            } else {
                // Split: key[0..common] is shared, key[common..] and remaining[common..] diverge
                let shared = key[..common_len].to_string();
                let old_suffix = key[common_len..].to_string();
                let mut old_child = node.children.remove(&key).unwrap();
                let mut new_split = RadixNode::default();
                new_split.children.insert(old_suffix, old_child);
                Self::insert_node(&mut new_split, &remaining[common_len..]);
                node.children.insert(shared, new_split);
            }
        } else {
            // No matching edge — insert new edge with full remaining string
            let mut child = RadixNode::default();
            child.is_end = true;
            node.children.insert(remaining.to_string(), child);
        }
    }
}

fn common_prefix_len(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}
```

The edge-splitting logic is the key operation: when "intern" is inserted after "interface", the edge "interface" splits into "inter" → ["face" → {}, "n" → {}]. The shared "inter" prefix becomes the new edge label.

## OCaml Approach

OCaml's standard trie approach uses `String` keys in a `Map`:

```ocaml
module StringMap = Map.Make(String)

type radix_node = {
  children: radix_node StringMap.t;
  is_end: bool;
}

(* Functional radix tree: returns new node per insert *)
let common_prefix a b =
  let n = min (String.length a) (String.length b) in
  let i = ref 0 in
  while !i < n && a.[!i] = b.[!i] do incr i done;
  String.sub a 0 !i
```

OCaml's persistent radix tree is structurally simpler to reason about — each insert returns a new node, sharing unchanged subtrees. The mutation-based Rust version is more cache-efficient but requires careful borrow management.

## Key Differences

| Aspect | Rust radix tree | OCaml radix tree |
|--------|----------------|------------------|
| Edge labels | `HashMap<String, RadixNode>` | `StringMap.t` |
| Mutability | In-place edge splitting (`&mut`) | Persistent (new tree) |
| Prefix matching | `str::starts_with` + `common_prefix_len` | `String.sub` comparison |
| Production | `radix` or `patricia-tree` crates | No standard library |
| IP routing | Bit-level radix tree (256-way branches) | Same concept |

## Exercises

1. **Node count comparison**: Insert 100 English words into both a standard `Trie` (char-per-node) and a `RadixTree`; count total nodes in each and compute the compression ratio.
2. **Longest prefix match**: Implement `longest_prefix_match(&self, query: &str) -> Option<String>` that finds the longest stored prefix of `query` — the core operation in IP routing tables.
3. **Delete**: Implement `remove(&mut self, word: &str) -> bool` that marks `is_end = false` for the word's node; also merge single-child non-end nodes back into their parent edge (re-compress after deletion).
