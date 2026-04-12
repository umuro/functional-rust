# OCaml vs Rust: Map and Fold on Trees

## Side-by-Side Code

### OCaml

```ocaml
let rec map_tree f = function
  | Leaf           -> Leaf
  | Node (v, l, r) -> Node (f v, map_tree f l, map_tree f r)

let rec fold_tree f acc = function
  | Leaf           -> acc
  | Node (v, l, r) -> f v (fold_tree f acc l) (fold_tree f acc r)

(* Derived operations — zero additional recursion *)
let size     t = fold_tree (fun _ l r -> 1 + l + r)    0  t
let depth    t = fold_tree (fun _ l r -> 1 + max l r)  0  t
let sum      t = fold_tree (fun v l r -> v + l + r)    0  t
let preorder t = fold_tree (fun v l r -> [v] @ l @ r) [] t
let inorder  t = fold_tree (fun v l r -> l @ [v] @ r) [] t
```

### Rust (idiomatic)

```rust
pub fn map_tree<T, U, F: Fn(T) -> U>(tree: Tree<T>, f: &F) -> Tree<U> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(v, l, r) => Tree::Node(
            f(v),
            Box::new(map_tree(*l, f)),
            Box::new(map_tree(*r, f)),
        ),
    }
}

pub fn fold_tree<T, U: Clone, F: Fn(T, U, U) -> U>(tree: Tree<T>, acc: U, f: &F) -> U {
    match tree {
        Tree::Leaf => acc,
        Tree::Node(v, l, r) => {
            let left  = fold_tree(*l, acc.clone(), f);
            let right = fold_tree(*r, acc, f);
            f(v, left, right)
        }
    }
}
```

### Rust (functional/recursive)

```rust
// Derived from fold_tree — same pattern as OCaml, zero extra recursion
pub fn tree_size<T>(t: Tree<T>) -> usize {
    fold_tree(t, 0usize, &|_, l, r| 1 + l + r)
}

pub fn tree_sum(t: Tree<i32>) -> i32 {
    fold_tree(t, 0, &|v, l, r| v + l + r)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| map | `val map_tree : ('a -> 'b) -> 'a tree -> 'b tree` | `fn map_tree<T, U, F: Fn(T) -> U>(tree: Tree<T>, f: &F) -> Tree<U>` |
| fold | `val fold_tree : ('a -> 'b -> 'b -> 'b) -> 'b -> 'a tree -> 'b` | `fn fold_tree<T, U: Clone, F: Fn(T, U, U) -> U>(tree: Tree<T>, acc: U, f: &F) -> U` |
| combining fn | `'a -> 'b -> 'b -> 'b` | `Fn(T, U, U) -> U` |
| accumulator | `'b` (shared freely) | `U: Clone` (must be cloned for both branches) |

## Key Insights

1. **Clone requirement for fold:** In OCaml, `acc` is a GC-managed value passed to both subtrees freely — no copying needed at the language level. In Rust, passing `acc` to the left subtree would consume it, leaving nothing for the right subtree. The bound `U: Clone` and explicit `acc.clone()` call make this sharing explicit.
2. **Closure passed as `&F`:** OCaml closures are heap values that can be shared by reference automatically. Rust closures are owned. Passing `f: &F` through recursive calls avoids consuming the closure on the first node visited — without the `&`, the first call would move `f` and subsequent calls would fail to compile.
3. **Catamorphism:** `fold_tree` is the tree catamorphism — it completely characterizes the tree's recursive structure. Every recursive function on `Tree<T>` can be expressed as a single `fold_tree` call. OCaml's `size`, `depth`, `sum`, `preorder`, and `inorder` are all one-liners. The same applies in Rust.
4. **Ownership through map:** `map_tree` takes ownership of the source `Tree<T>` and produces a `Tree<U>`. This is the natural functional style — no shared mutable state, no aliasing. OCaml's GC handles the old tree's memory; Rust's ownership system drops it automatically when the last reference leaves scope.
5. **Functor structure:** `map_tree` makes `Tree` a functor over its element type — a concept from category theory that OCaml expressses through `map` conventions and Rust approximates through generic functions (there is no `Functor` trait in `std`).

## When to Use Each Style

**Use `fold_tree` when:** You need any aggregation over the tree (sum, count, depth, traversal list) — derive it as a one-liner rather than writing a new recursive function each time.
**Use `map_tree` when:** Transforming values while preserving the tree's shape — it is the structural equivalent of `List.map` for lists.
