📖 **[View on hightechmind.io →](https://hightechmind.io/rust/935-tree-map-fold)**

---

# 935-tree-map-fold — Tree Map and Fold

## Problem Statement

Map and fold over lists generalize naturally to trees. Tree map preserves structure while transforming values; tree fold collapses the tree into a single value. Once `fold_tree` is defined, all aggregate operations — size, depth, sum, flatten to list — can be expressed without any additional explicit recursion. This is the principle behind the `Foldable` typeclass and catamorphisms: define one fold, derive everything else. OCaml's standard `List.map` and `List.fold_left` extend to trees through the same pattern. This example demonstrates the full power of a well-designed fold.

## Learning Outcomes

- Implement `map_tree` that preserves tree structure while transforming each node value
- Implement `fold_tree` (catamorphism) that generalizes all tree reductions
- Derive `size`, `depth`, `sum`, `flatten` from `fold_tree` without additional recursion
- Understand why fold is the "universal" tree consumer
- Compare with OCaml's equivalent tree map and fold patterns

## Rust Application

`map_tree(tree, f)` matches `Leaf -> Leaf` and `Node(v, l, r) -> Node(f(v), map_tree(l, f), map_tree(r, f))`. `fold_tree(tree, acc, f)` where `f` receives `(&T, A, A) -> A` — the left result, the value, and the right result. Derived operations: `size = fold_tree(t, 0, |_, l, r| 1 + l + r)`. `sum = fold_tree(t, 0, |v, l, r| l + v + r)`. `depth = fold_tree(t, 0, |_, l, r| 1 + l.max(r))`. Each is one line using the fold. No explicit recursion needed after defining fold.

## OCaml Approach

`let rec map_tree f = function | Leaf -> Leaf | Node(v, l, r) -> Node(f v, map_tree f l, map_tree f r)`. `let rec fold_tree leaf node = function | Leaf -> leaf | Node(v, l, r) -> node (fold_tree leaf node l) v (fold_tree leaf node r)`. Derived: `let size t = fold_tree 0 (fun l _ r -> 1 + l + r) t`. `let depth t = fold_tree 0 (fun l _ r -> 1 + max l r) t`. The OCaml version is slightly more concise due to currying and the `function` keyword.

## Key Differences

1. **Fold argument order**: Rust `f: &impl Fn(&T, A, A) -> A` takes `(value, left_result, right_result)`; OCaml typically uses `f left_result value right_result` or varies by convention.
2. **Cloning accumulator**: Rust `fold_tree` requires `A: Clone` to pass `acc` to both subtrees; OCaml passes the same immutable value to both branches without cloning.
3. **Recursive boxing**: Rust `Node(T, Box<Tree<T>>, Box<Tree<T>>)` requires explicit Box; OCaml `Node of 'a tree * 'a * 'a tree` is implicit.
4. **Derived operations**: Both languages derive all operations from fold with equal elegance — the one-liner pattern works in both.

## Exercises

1. Implement `flatten_inorder(t: &Tree<T>) -> Vec<&T>` using `fold_tree` with a `Vec` accumulator.
2. Write `count_leaves(t: &Tree<T>) -> usize` and `count_inner_nodes(t: &Tree<T>) -> usize` using `fold_tree`.
3. Implement `mirror(t: Tree<T>) -> Tree<T>` using `map_tree` and node reconstruction that swaps left and right subtrees.
