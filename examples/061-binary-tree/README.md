📖 **[View on hightechmind.io →](https://hightechmind.io/rust/061-binary-tree)**

---

# Example 061: Binary Tree — Size, Membership, Traversal

**Difficulty:** ⭐⭐
**Category:** Data Structures
**Concept:** Recursive algebraic data types for binary trees, implementing fundamental operations (size, depth, membership, traversal) via structural recursion. This is the cornerstone pattern for tree-based data structures in functional programming.
**OCaml → Rust insight:** Rust requires `Box<T>` for recursive enum variants because it must know the size of every type at compile time — OCaml's GC handles indirection transparently.
