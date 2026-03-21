📖 **[View on hightechmind.io →](https://hightechmind.io/rust/035-layout-binary-tree)**

---

# 035 — Layout a Binary Tree
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Assigning (x, y) coordinates to tree nodes for visualization (OCaml 99 Problems #35) is a tree layout algorithm. The simplest rule: y = depth (root at 1), x = position in inorder traversal (leftmost node at x=1, rightmost at x=n). This produces a planar embedding where no two nodes overlap and no edge crossings occur.

Tree layout algorithms are used in compiler visualization (AST display), file system browsers, organization charts, and graph drawing tools. The Reingold-Tilford algorithm (used in most modern tree visualizers) extends this idea with contour-based subtree fitting to minimize width.

## Learning Outcomes

- Assign inorder position as x-coordinate and depth as y-coordinate
- Thread an inorder counter through the tree using a mutable reference or state monad
- Produce a `Tree<(T, (usize, usize))>` that annotates each node with coordinates
- Understand inorder traversal as the basis for the x-position assignment
- Recognize that inorder position gives a non-overlapping horizontal layout

## Rust Application

The layout function threads an inorder counter `x` through the tree. For each node, recursively layout the left subtree (advancing x), assign the current x to this node (advancing x), then layout the right subtree. Using `&mut usize` for the counter: `fn layout<T: Clone>(tree: &Tree<T>, x: &mut usize, depth: usize) -> Tree<(T, (usize, usize))>`. The depth increments by 1 at each level.

## OCaml Approach

OCaml's version uses a mutable reference: `let layout_aux tree = let x = ref 0 in let rec lay depth = function | Leaf -> Leaf | Node (v, l, r) -> let left = lay (depth + 1) l in incr x; let pos = (!x, depth) in let right = lay (depth + 1) r in Node ((v, pos), left, right) in lay 1 tree`. The `x` reference is incremented in inorder (left → self → right).

## Key Differences

1. **Mutable state**: Both languages use mutable state for the inorder counter. Rust uses `&mut usize`, OCaml uses `ref int`. Both are equivalent; OCaml's `ref` is more explicit as a mutable cell.
2. **State threading**: In purely functional style, you would thread the counter as an argument and return it as part of the result. This is the "state monad" pattern. Both languages support it explicitly.
3. **Annotated tree type**: The result `Tree<(T, (usize, usize))>` adds coordinate information to each node. OCaml's `'a tree` becomes `('a * (int * int)) tree` — the same pattern.
4. **No TCO**: The tree layout recursion is not tail-recursive (left → self → right). Both languages handle this via the call stack, limited by tree depth.

## Exercises

1. **Compact layout**: Implement a layout where the x-positions are assigned based on the minimum spacing between subtrees (Reingold-Tilford first pass). This requires computing the "contour" of each subtree.
2. **SVG output**: Given the layout coordinates, write `to_svg(tree: &Tree<(char, (usize, usize))>) -> String` that produces an SVG string with circles for nodes and lines for edges.
3. **Center layout**: Modify the layout so the root is always centered over its children rather than at the inorder position. This produces a more aesthetically balanced tree.
