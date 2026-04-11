📖 **[View on hightechmind.io →](https://hightechmind.io/rust/029-binary-tree)**

---

# 029 — Binary Tree (Algebraic Data Type)

## Problem Statement

A binary tree is the most important recursive data structure in computer science. It underlies binary search trees (std::collections::BTreeMap), heaps (priority queues), Huffman coding trees (compression), parse trees (compilers), and spatial partitioning (k-d trees, R-trees). The recursive definition — a tree is either a leaf or a node with a value and two subtrees — maps perfectly to algebraic data types.

OCaml 99 Problems #29 introduces the `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree` as the central data type for problems 29-40. This is the canonical example of an algebraic data type (ADT), a sum type with two constructors. Understanding how to define and process ADTs is foundational to functional programming.

## Learning Outcomes

- Define a generic recursive enum `Tree<T>` in Rust as the equivalent of OCaml's `'a tree`
- Understand why `Box` is required for recursive enum variants in Rust
- Use `Tree::node()` and `Tree::leaf()` helper constructors for cleaner tree construction
- Implement basic operations: size, depth, membership via pattern matching
- Recognize how the tree structure mirrors the recursive function structure

- Define `Tree<T>` as a recursive enum using `Box<Tree<T>>` to break the otherwise infinite-size type
- Use `node(v, l, r)` and `leaf()` helper constructors to reduce `Box::new` boilerplate at construction sites

## Rust Application

The `Tree<T>` enum has two variants: `Leaf` (no data) and `Node(T, Box<Tree<T>>, Box<Tree<T>>)`. The `Box` is required because Rust must know the size of each variant at compile time — a `Node` containing a `Tree` directly would be infinitely large. Helper constructors `Tree::node(val, left, right)` and `Tree::leaf()` hide the boxing. `size` and `depth` use pattern matching recursion mirroring the structure of the type. `mem` adds a `PartialEq` bound for value comparison.

## OCaml Approach

OCaml defines `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`. No boxing is needed because OCaml values are uniformly represented as tagged pointers on the heap — the GC handles recursive types automatically. Functions follow the same recursive structure: `let rec size = function Leaf -> 0 | Node (_, l, r) -> 1 + size l + size r`. The `function` keyword is shorthand for `fun x -> match x with`.

OCaml's tree type: `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`. This is the canonical definition used in all OCaml textbooks and the 99 Problems series. No annotation is needed for recursive types — OCaml's GC allocates nodes on the heap automatically. Helper: `let node x l r = Node (x, l, r)` and `let leaf = Leaf`.

## Key Differences

1. **`Box` for recursion**: Rust needs `Box<Tree<T>>` because the compiler must compute the stack frame size. OCaml's uniform heap representation avoids this — all values are pointer-sized.
2. **Type parameter syntax**: Rust: `Tree<T>`. OCaml: `'a tree`. Both are parametric polymorphism; the syntax differs.
3. **Helper constructors**: Rust often defines `fn node(val, left, right)` to hide boxing. OCaml's `Node (v, l, r)` is direct — no hiding needed.
4. **`PartialEq` for `mem`**: Rust requires explicit `T: PartialEq` trait bound for value comparison. OCaml's structural equality works on all types automatically.

1. **`Box<Tree<T>>` for recursion:** Rust requires `Box` in recursive enum variants to give them a known size. OCaml's heap-allocated types need no annotation — the compiler handles it transparently.
2. **GC vs ownership:** OCaml's GC manages tree node lifetimes. Rust's ownership rules require the tree to be the sole owner of its subtrees — no shared references across the tree without `Rc`.
3. **Constructor ergonomics:** `Box::new(Tree::Node(...))` is verbose in Rust. Helper constructors `tree::node(v, l, r)` hide this. OCaml's `Node (x, l, r)` is already concise.
4. **Pattern matching depth:** Both languages support nested pattern matching to arbitrary depth — matching `Node(v, Node(_, _, _), Leaf)` to detect a tree with a left child and no right child.

## Exercises

1. **Mirror**: Write `mirror(tree: Tree<T>) -> Tree<T>` that swaps left and right children at every node. Verify that `mirror(mirror(t)) == t`.
2. **Balanced**: Write `is_balanced<T>(tree: &Tree<T>) -> bool` that returns true if no two leaves differ in depth by more than 1. Use `depth` from the implementation.
3. **Path to node**: Write `path_to<T: PartialEq>(tree: &Tree<T>, target: &T) -> Option<Vec<bool>>` that returns the path from root to the target (false = go left, true = go right), or `None` if not found.

4. **Tree equality**: Implement `PartialEq` for `Tree<T> where T: PartialEq` — two trees are equal if they have the same structure and values at every node. Write property tests to verify reflexivity, symmetry, and transitivity.
5. **Mirror**: Implement `mirror(tree: Tree<T>) -> Tree<T>` that swaps left and right subtrees at every level, producing the mirror image of the original tree.
