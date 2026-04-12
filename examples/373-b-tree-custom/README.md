📖 **[View on hightechmind.io →](https://hightechmind.io/rust/373-b-tree-custom)**

---

# 373: Custom B-Tree Implementation
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Database indexes, file system directory trees (NTFS, ext4), and key-value store storage engines (RocksDB, SQLite) all use B-trees as their core data structure. Rudolf Bayer and Edward McCreight invented the B-tree in 1972 to optimize for disk access patterns: keeping data in large nodes (pages) minimizes the number of disk seeks. A B-tree of degree T stores up to 2T-1 keys per node, keeping the tree very shallow (height O(log_T n)). Unlike binary trees, B-tree nodes are wide — millions of records fit in a tree just 3-4 levels deep, matching the typical disk block size of 4KB-16KB.

## Learning Outcomes

- Implement a B-tree node with `keys: Vec<i32>`, `children: Vec<Box<BTreeNode>>`, and `is_leaf: bool`
- Understand the minimum degree T: nodes hold T-1 to 2T-1 keys (except root)
- Implement recursive search through node keys and children
- Implement split-child: when a child is full (2T-1 keys), split it into two T-1 key nodes
- Implement non-full insert recursively, splitting full nodes on the way down
- Recognize that Rust's `BTreeMap` uses a B-tree variant with order ~11 internally

## Rust Application

```rust
const MIN_DEGREE: usize = 2; // t=2 means 1..3 keys per node (2-3-4 tree)

pub struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Box<BTreeNode>>,
    is_leaf: bool,
}

impl BTreeNode {
    fn new_leaf() -> Box<Self> {
        Box::new(Self { keys: Vec::new(), children: Vec::new(), is_leaf: true })
    }

    fn is_full(&self) -> bool {
        self.keys.len() == 2 * MIN_DEGREE - 1
    }
}

pub struct BTree {
    root: Box<BTreeNode>,
}

impl BTree {
    pub fn search(&self, key: i32) -> bool {
        Self::search_node(&self.root, key)
    }

    fn search_node(node: &BTreeNode, key: i32) -> bool {
        let mut i = node.keys.partition_point(|&k| k < key);
        if i < node.keys.len() && node.keys[i] == key { return true; }
        if node.is_leaf { return false; }
        Self::search_node(&node.children[i], key)
    }

    pub fn insert(&mut self, key: i32) {
        if self.root.is_full() {
            // Root full: create new root, split old root as child
            let old_root = std::mem::replace(&mut self.root, Box::new(BTreeNode {
                keys: Vec::new(),
                children: vec![/* old root */],
                is_leaf: false,
            }));
            // ... split and insert
        }
    }
}
```

`partition_point` finds the first index where `k >= key` — equivalent to binary search for the insertion point. The B-tree's invariant: all keys in the left subtree of `keys[i]` are less than `keys[i]`, and all keys in the right subtree are greater.

## OCaml Approach

OCaml's `Map.Make` is implemented as a balanced AVL tree (not a B-tree), but the B-tree structure translates naturally to OCaml algebraic types:

```ocaml
type btree_node =
  | Leaf of int list
  | Internal of int list * btree_node list

(* Search: binary search keys, recurse into correct child *)
let rec search node key = match node with
  | Leaf keys -> List.mem key keys
  | Internal (keys, children) ->
    let i = List.length (List.filter (fun k -> k < key) keys) in
    if i < List.length keys && List.nth keys i = key then true
    else search (List.nth children i) key
```

The functional version avoids mutation — insert returns a new tree with structural sharing where possible. Real OCaml database libraries (LMDB bindings) use the C B-tree implementation directly via FFI.

## Key Differences

| Aspect | Rust custom B-tree | OCaml `Map.Make` |
|--------|-------------------|------------------|
| Tree type | B-tree (wide nodes) | AVL tree (binary) |
| Node width | T-1 to 2T-1 keys | 1 key (binary node) |
| Height | O(log_T n) | O(log₂ n) |
| Cache efficiency | High (wide nodes fit cache lines) | Lower (binary pointer chasing) |
| Production use | `BTreeMap` in stdlib | `Map.Make` in stdlib |

## Exercises

1. **Delete**: Implement `remove(&mut self, key: i32)` for the B-tree; handle the case where removal from a node with fewer than T-1 keys requires borrowing from a sibling or merging two nodes.
2. **Disk page simulation**: Set `MIN_DEGREE = 50` so each node holds up to 99 keys; generate 1 million sequential keys and measure tree height — it should be ≤ 4 levels.
3. **B+ tree leaf chain**: Modify the B-tree so leaf nodes are linked in a doubly-linked list, enabling O(n) sequential scan of all keys in sorted order after lookup.
