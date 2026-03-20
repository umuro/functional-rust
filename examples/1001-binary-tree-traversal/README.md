# Example 1001: Binary Tree — Size, Membership, Traversal

**Difficulty:** ⭐⭐
**Category:** Trees
**OCaml Source:** `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree`

## Problem Statement

Implement four core operations on an unbalanced binary tree: count the number of nodes (`size`), measure the longest root-to-leaf path (`depth`), check whether a value is stored anywhere in the tree (`mem`), and produce a preorder traversal as a flat list. The tree carries no balancing or ordering invariants — it is a pure recursive algebraic data type used to practice structural recursion. Mastering these operations is the prerequisite for every more advanced tree algorithm.

## Learning Outcomes

- How to define a recursive algebraic data type in Rust as an enum whose recursive variants require `Box` for heap allocation
- How Rust's `match` on an enum directly mirrors OCaml's `function` pattern matching over variant constructors
- Why `Box<Tree<T>>` is necessary in Rust while OCaml heap-allocates recursive types automatically
- How to write a linear-time traversal using a `&mut Vec<T>` accumulator rather than returning and concatenating intermediate lists
- Why the `PartialEq` trait bound on `mem` is the Rust equivalent of OCaml's polymorphic structural equality `=`

## OCaml Approach

OCaml defines `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree` and implements each operation as a `let rec` function using `function` for pattern matching. `size`, `depth`, and `mem` are textbook structural recursions. The `preorder` function uses an accumulator (`let rec go acc = function | Leaf -> acc | Node(v,l,r) -> v :: go (go acc r) l`) to avoid quadratic list concatenation: by reversing the traversal direction in the accumulator, it builds the result in a single linear pass. OCaml's garbage collector handles all heap allocation invisibly.

## Rust Application

`Tree<T>` is defined as `enum Tree<T> { Leaf, Node(T, Box<Tree<T>>, Box<Tree<T>>) }`. The `Box` pointers own the heap-allocated subtrees and provide the indirection the compiler requires to give the type a finite size. `size`, `depth`, and `mem` are inherent methods on `impl<T: PartialEq> Tree<T>` — each is a two-arm `match` that directly mirrors the OCaml. `preorder` avoids the accumulator threading trick by using an inner `fn go(tree, acc: &mut Vec<T>)` that pushes into a shared mutable vector, achieving the same linear time with clearer control flow and no cloning.

## Key Differences

1. **Heap allocation:** Rust requires explicit `Box<Tree<T>>` to make the recursive enum representable; OCaml allocates all heap values automatically through its runtime
2. **Trait bounds:** Rust's `mem` requires `T: PartialEq` to compare values; OCaml's polymorphic `=` works on any type at runtime without static annotation
3. **Accumulator style:** OCaml threads the accumulator as a function parameter and returns a new list; Rust passes `&mut Vec` so the helper mutates in place — same asymptotic cost, different ownership expression
4. **Traversal direction:** OCaml's accumulator preorder builds in reverse (`v :: go (go acc r) l`); Rust's mutable-push version visits naturally left-to-right — both are O(n) but the ownership model shapes the implementation choice

## Exercises

1. Add an `inorder` traversal method (left, root, right) and verify it produces a sorted sequence when the tree happens to be a valid binary search tree
2. Implement `is_balanced` — a tree is balanced if for every node the depth of the left and right subtrees differ by at most one; hint: write a helper that returns depth and balance simultaneously to avoid double traversal
3. Implement `map_tree` that applies a function `f: Fn(T) -> U` to every node value, returning a `Tree<U>` of the same shape, then verify `preorder(map_tree(t, f)) == preorder(t).into_iter().map(f).collect()`
