# Example 1001: Binary Tree — Size, Membership, Traversal

## Problem Statement
Implement core operations on a binary tree: count nodes, measure depth, check membership, and produce a preorder traversal. The tree is a recursive algebraic data type with no balancing invariants.

## Learning Outcomes
- How to define recursive algebraic data types in Rust using enums with `Box` for heap allocation
- Implement structural recursion on trees using `match` — the direct equivalent of OCaml's pattern matching
- Build a linear-time traversal using a mutable accumulator to avoid quadratic list concatenation

## Rust Application
`Tree<T>` is defined as an enum with `Leaf` and `Node(T, Box<Tree<T>>, Box<Tree<T>>)`. `size`, `depth`, and `mem` are inherent methods using recursive `match`. `preorder` uses an inner helper that pushes into a `&mut Vec<T>` accumulator, achieving linear time without allocating intermediate vectors.

## OCaml Approach
OCaml defines the same type with variant constructors and implements all operations as `let rec` functions using `function` pattern matching. The `preorder` accumulator uses list prepending (`v :: go (go acc r) l`) to build the result without intermediate allocations.

## Key Differences
1. **Heap allocation:** Rust requires `Box` for recursive enum variants; OCaml heap-allocates automatically
2. **Accumulator style:** OCaml threads the accumulator as a function parameter returning a new list; Rust passes `&mut Vec` avoiding allocation overhead
3. **Membership:** Both use short-circuit `||`; OCaml's polymorphic `=` works on any type, while Rust requires the `PartialEq` bound

## Exercises
1. Add an `inorder` traversal and verify it produces a sorted sequence when the tree is a binary search tree
2. Implement `height` (same as `depth`) and `is_balanced` — a tree is balanced if the depths of its two subtrees differ by at most 1 at every node
3. Implement `map_tree` that transforms every node value with a function `f: T -> U`, returning a `Tree<U>` of the same shape
