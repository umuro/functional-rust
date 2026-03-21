📖 **[View on hightechmind.io →](https://hightechmind.io/rust/037-tree-preorder)**

---

# 037 — Preorder Traversal Sequence
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Preorder traversal visits nodes in root-left-right order, producing a sequence where the root always comes before its descendants. This ordering has a critical property: in a full binary tree (where leaves are uniquely identifiable), the preorder sequence uniquely determines the tree. Combined with the inorder sequence (from example 038), any binary tree can be reconstructed.

Preorder traversal underlies expression tree serialization (prefix notation: `"+ 3 4"` rather than `"3 + 4"`), directory tree listing (`find` command), syntax tree serialization in compilers, and tree copying algorithms. Depth-first search (DFS) on graphs is a generalization of preorder traversal.

## Learning Outcomes

- Implement preorder traversal: visit root, then left subtree, then right subtree
- Use a dot-string encoding where `.` represents a leaf (self-delimiting format)
- Understand why the dot-string encoding is self-delimiting (unlike the format in #036)
- Reconstruct a tree from its preorder dot-string sequence
- Contrast preorder with inorder (root between children) and postorder (root after)

## Rust Application

`preorder(tree: &Tree<char>) -> String`: `Leaf` → `"."`, `Node(c, l, r)` → `format!("{}{}{}", c, preorder(l), preorder(r))`. The dot represents a leaf boundary — this makes the encoding self-delimiting without needing parentheses. To decode: consume one character; if it is `.`, return `Leaf`; otherwise read the value, then recursively parse left and right subtrees.

## OCaml Approach

OCaml's version: `let rec preorder = function | Leaf -> "." | Node (c, l, r) -> String.make 1 c ^ preorder l ^ preorder r`. Reconstruction: `let rec of_preorder s = match s.[0] with | '.' -> (Leaf, String.sub s 1 ...) | c -> let (l, s') = of_preorder (String.sub s 1 ...) in let (r, s'') = of_preorder s' in (Node (c, l, r), s'')`. The function returns `(tree, remaining_string)`.

## Key Differences

1. **String construction**: Rust uses `format!` or `String::push`. OCaml uses `^` (string concatenation) which is O(n) per call — avoid in inner loops; use `Buffer` instead.
2. **Self-delimiting encoding**: The dot-string encoding is self-delimiting because `.` consumes exactly one position, and a node character consumes one position followed by two self-delimiting subtrees. No length prefix needed.
3. **Reconstruction state**: Rust passes `&mut usize` cursor or slices. OCaml's functional version returns `(result, remaining_string)` pairs — the monadic parser style.
4. **Round-trip uniqueness**: Preorder dot-string uniquely determines the tree. This is used for equality testing: two trees are equal iff their preorder dot-strings are equal.

## Exercises

1. **Postorder**: Write `postorder(tree: &Tree<char>) -> String` with left-right-root order using a similar dot-string encoding. The dot for leaves must appear after the two empty subtrees.
2. **Tree from pre+in**: Given a preorder sequence and an inorder sequence (both without dots), reconstruct the unique binary tree. This is the classic interview question.
3. **Preorder vs BFS**: Compare the preorder sequence with the BFS level-order sequence on the same tree. Draw the tree, list both sequences, and explain the structural difference.
