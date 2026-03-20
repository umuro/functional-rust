**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  

[catamorphism on hightechmind.io](https://hightechmind.io/posts/functional-rust/catamorphism)

---

## Problem Statement

Implement a catamorphism for a binary tree — the generalized fold that replaces every constructor with a function. The catamorphism `cata(tree, leaf, node_fn)` collapses the entire tree into a single value by recursively substituting `Leaf` with `leaf` and `Node(l, v, r)` with `node_fn(cata(l), v, cata(r))`. Derive tree size, sum, height, and in-order list from this single combinator.

## Learning Outcomes

- Understand catamorphism as the universal eliminator for a recursive ADT — "fold the constructors"
- Implement `cata<T, R>(tree, leaf: R, node: &dyn Fn(R, &T, R) -> R) -> R` for a binary tree
- Derive `size`, `sum`, `height`, and `to_list` as specializations of `cata`
- Recognize that catamorphisms are the dual of anamorphisms (unfold) and are the foundation of structural recursion
- Connect to OCaml's catamorphism pattern via explicit fold functions on recursive types

## Rust Application

```rust
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

pub fn cata<T, R>(tree: &Tree<T>, leaf: R, node: &dyn Fn(R, &T, R) -> R) -> R
where
    R: Clone,
{
    match tree {
        Tree::Leaf => leaf,
        Tree::Node(l, v, r) => {
            let left_result  = cata(l, leaf.clone(), node);
            let right_result = cata(r, leaf, node);
            node(left_result, v, right_result)
        }
    }
}

pub fn size<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l + r)
}

pub fn sum(tree: &Tree<i64>) -> i64 {
    cata(tree, 0, &|l, v, r| l + v + r)
}

pub fn height<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l.max(r))
}

pub fn to_list<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    cata(tree, vec![], &|mut l, v, r| {
        l.push(v.clone());
        l.extend(r);
        l
    })
}
```

The catamorphism is called once per node. The `R: Clone` bound is required because `leaf` must be duplicated when passing to both `l` and `r` subtrees. In Haskell this is avoided via laziness; in Rust it requires explicit cloning or switching to passing `leaf` by constructor function.

Each derived operation passes a different algebra: `size` counts with `1 + l + r`, `sum` adds values, `height` takes `1 + max(l, r)`, and `to_list` builds a vector left-value-right.

## OCaml Approach

```ocaml
type 'a tree =
  | Leaf
  | Node of 'a tree * 'a * 'a tree

let rec cata leaf_val node_fn = function
  | Leaf -> leaf_val
  | Node (l, v, r) ->
    node_fn
      (cata leaf_val node_fn l)
      v
      (cata leaf_val node_fn r)

let size t   = cata 0 (fun l _ r -> 1 + l + r) t
let sum  t   = cata 0 (fun l v r -> l + v + r) t
let height t = cata 0 (fun l _ r -> 1 + max l r) t

let to_list t =
  cata [] (fun l v r -> l @ [v] @ r) t
```

OCaml's curried `cata leaf_val node_fn` produces a function `tree -> result` — natural point-free style. `to_list` uses `@` (list append) which is O(n) per step; production code would use an accumulator.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Constructor replacement | `leaf: R` value + `node: &dyn Fn(R, &T, R) -> R` | `leaf_val` + curried `node_fn` |
| Clone requirement | `R: Clone` for sharing leaf value | Immutable GC values — no cloning needed |
| Point-free style | Verbose (`cata(tree, ...)`) | Natural (`cata 0 (fun ...) tree`) |
| List append in algebra | `extend` — O(n) like OCaml `@` | `@` — also O(n); use accumulator for O(n log n) overall |

Catamorphisms embody the principle that all observation of a recursive type flows through its fold. Any function on a `Tree<T>` that does not require sharing/cycles can be expressed as a `cata`.

## Exercises

1. Implement `contains<T: PartialEq>(tree, target)` using `cata`.
2. Implement `flatten_preorder` (root before children) using `cata` — note that pre-order requires different placement of `v`.
3. Derive `mirror(tree)` as a `cata` — the node algebra returns `Node(r_result, v, l_result)`.
4. Implement `map_tree<T, U>(tree, f)` using `cata` where the output is a `Tree<U>`.
5. Write a `depth_first_values` that collects node values in pre-order without using `cata`, then compare it to the `cata`-based version for clarity.
