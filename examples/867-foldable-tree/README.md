📖 **[View on hightechmind.io →](https://hightechmind.io/rust/867-foldable-tree)**

---

# 867-foldable-tree — Foldable Tree
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A fold over a list collapses it into a single value by processing each element sequentially. Extending this idea to trees requires choosing a traversal order — in-order, pre-order, or post-order — each of which imposes a different visitation sequence on nodes. The Foldable abstraction from Haskell and OCaml formalizes this: any container that supports a fold is "foldable," meaning every aggregate operation (sum, count, to-list, membership) can be derived from a single generic fold. This example implements three tree folds in Rust and derives higher-order operations from them.

## Learning Outcomes

- Understand in-order, pre-order, and post-order traversal as distinct fold variants
- Derive higher-order operations (sum, collect, count) from a single fold abstraction
- Use recursive pattern matching on Rust enums to walk recursive data structures
- Compare Rust's explicit mutable closure passing with OCaml's polymorphic fold style
- Recognize the Foldable pattern as a key abstraction in functional programming

## Rust Application

The code defines `Tree<T>` with `Leaf` and `Node(Box<Tree<T>>, T, Box<Tree<T>>)` variants. Three fold methods take `&mut impl FnMut(B, &T) -> B` and an initial accumulator, recursing into subtrees and applying the closure at different traversal points. Higher-level operations like `to_vec_inorder` and sum are implemented on top of these folds with no additional recursion. The `&mut impl FnMut` pattern allows stateful closures such as pushing to a `Vec` during traversal.

## OCaml Approach

OCaml fold functions use curried style: `fold_inorder f acc tree`. Pattern matching on `Leaf | Node(l, v, r)` threads the accumulator left-to-right. Derived operations like `to_list_inorder` are one-liners combining fold with cons-prepend and `List.rev`. OCaml's implicit currying makes partial application natural, and `let rec` co-defines mutually recursive functions. The Rust version achieves the same generality using explicit trait bounds and monomorphized closures.

## Key Differences

1. **Closure mutability**: Rust requires `&mut impl FnMut` for stateful accumulators; OCaml closures can capture mutable state through `ref` cells or functional accumulation.
2. **Recursive type boxing**: Rust needs `Box<Tree<T>>` to break the infinite-size cycle; OCaml's GC-managed heap handles this transparently.
3. **Monomorphization**: Rust generates specialized code per closure type; OCaml uses uniform value representation.
4. **Derived operations**: Both languages derive all operations from a single fold, but OCaml expresses this more concisely via the pipe operator and partial application.

## Exercises

1. Add `fold_levelorder` that visits nodes breadth-first using a `VecDeque` as the traversal queue.
2. Implement `zip_trees` that pairs corresponding nodes from two same-shaped trees, returning `None` on shape mismatch.
3. Implement `find_first` on top of `fold_inorder` using `Option` as the accumulator to short-circuit at the first matching element.
