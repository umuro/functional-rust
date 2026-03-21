📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1165-building-a-huffman-tree-from-character-frequencies)**

---

# 1165-building-a-huffman-tree-from-character-frequencies — Huffman Encoding: Greedy Tree Building
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Huffman coding, invented by David Huffman in 1952, is an optimal prefix-free encoding: common characters get short bit sequences, rare characters get long ones. It is used in ZIP, gzip, PNG, JPEG, and MP3 — virtually every compression format incorporates Huffman coding.

The algorithm builds a binary tree greedily using a priority queue: repeatedly merge the two lowest-frequency nodes until one tree remains. This greedy strategy provably produces the optimal prefix code (Shannon entropy bound).

## Learning Outcomes

- Build a Huffman tree using a min-heap (priority queue)
- Understand the greedy merge strategy: always merge the two smallest
- Generate prefix-free codes by walking the tree (left=0, right=1)
- Encode and decode text using the generated codebook
- Understand why Huffman codes are optimal (proof by exchange argument)

## Rust Application

The Huffman tree is built in three steps: count character frequencies, insert all characters into a `BinaryHeap<Reverse<(freq, Node)>>`, and repeatedly extract the two minimum nodes and merge them. The resulting tree encodes characters in its leaves; walking the tree generates the prefix codes.

```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
enum HuffNode {
    Leaf { ch: char, freq: u64 },
    Internal { freq: u64, left: Box<HuffNode>, right: Box<HuffNode> },
}

pub fn build_tree(text: &str) -> Option<HuffNode> {
    let mut freq: HashMap<char, u64> = HashMap::new();
    for c in text.chars() { *freq.entry(c).or_insert(0) += 1; }
    let mut heap: BinaryHeap<Reverse<(u64, usize, HuffNode)>> = BinaryHeap::new();
    for (i, (ch, &f)) in freq.iter().enumerate() {
        heap.push(Reverse((f, i, HuffNode::Leaf { ch: *ch, freq: f })));
    }
    let mut counter = freq.len();
    while heap.len() > 1 {
        let Reverse((f1, _, left)) = heap.pop().unwrap();
        let Reverse((f2, _, right)) = heap.pop().unwrap();
        let merged = HuffNode::Internal {
            freq: f1 + f2,
            left: Box::new(left),
            right: Box::new(right),
        };
        heap.push(Reverse((f1 + f2, counter, merged)));
        counter += 1;
    }
    heap.pop().map(|Reverse((_, _, node))| node)
}
```

## OCaml Approach

```ocaml
type huffnode =
  | Leaf of char * int
  | Internal of int * huffnode * huffnode

module PQ = Set.Make(struct
  type t = int * int * huffnode
  let compare (f1, i1, _) (f2, i2, _) = compare (f1, i1) (f2, i2)
end)
```

OCaml uses `Set.Make` as a priority queue (sorted by frequency), or the `psq` library for a proper priority queue. The algorithm structure is identical.

## Key Differences

1. **Priority queue**: Rust uses `BinaryHeap<Reverse<(freq, id, Node)>>` with a tie-breaking counter; OCaml uses `Set.Make` with a custom comparator.
2. **Recursive type**: Rust's `HuffNode::Internal` uses `Box<HuffNode>` for the children (recursive type); OCaml's `huffnode` is recursive without annotation.
3. **`Ord` requirement**: Rust's `BinaryHeap` requires `Ord` on the element type — the counter `id` breaks ties for equal frequencies; OCaml's `Set` comparator handles this directly.
4. **Codebook generation**: Both walk the tree recursively, passing a prefix bit string down: left branch appends `0`, right branch appends `1`.

## Exercises

1. Implement `generate_codes(tree: &HuffNode) -> HashMap<char, String>` that produces the codebook by walking the tree.
2. Write `encode(text: &str, codes: &HashMap<char, String>) -> String` and `decode(bits: &str, tree: &HuffNode) -> String`.
3. Compute the compression ratio: compare the number of bits in the Huffman encoding to the 8-bit ASCII encoding of the same text.
