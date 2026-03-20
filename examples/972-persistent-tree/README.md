**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

[persistent-tree on hightechmind.io](https://hightechmind.io/posts/functional-rust/persistent-tree)

---

## Problem Statement

Implement a persistent binary search tree where `insert` and `delete` return new tree versions sharing unchanged subtrees via `Rc`. Only the nodes along the modified path are newly allocated; nodes not on the insertion path are shared between old and new versions. This is the Rust analog of OCaml's immutable BST.

## Learning Outcomes

- Define `enum Bst<T> { Empty, Node(Rc<Bst<T>>, T, Rc<Bst<T>>) }` with `Rc` for child sharing
- Implement `insert(&self, x: T) -> Bst<T>` that allocates new nodes only on the path to the insertion point and `Rc::clone`s the unchanged subtree
- Implement `member(&self, x: &T) -> bool` as a simple recursive search
- Implement `min_val(&self) -> Option<&T>` by descending left until `Empty`
- Understand why `Bst` itself is `Clone` (not wrapped in `Rc`) — callers hold `Bst<T>` directly; `Rc` is used for internal child references

## Rust Application

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Bst<T> {
    Empty,
    Node(Rc<Bst<T>>, T, Rc<Bst<T>>),
}

impl<T: Ord + Clone> Bst<T> {
    pub fn empty() -> Self { Bst::Empty }

    pub fn insert(&self, x: T) -> Self {
        match self {
            Bst::Empty =>
                Bst::Node(Rc::new(Bst::Empty), x, Rc::new(Bst::Empty)),
            Bst::Node(l, v, r) => {
                if x < *v {
                    // new node: new left child, shared right
                    Bst::Node(Rc::new(l.insert(x)), v.clone(), Rc::clone(r))
                } else if x > *v {
                    // new node: shared left, new right child
                    Bst::Node(Rc::clone(l), v.clone(), Rc::new(r.insert(x)))
                } else {
                    self.clone()  // duplicate: share the whole subtree
                }
            }
        }
    }

    pub fn member(&self, x: &T) -> bool {
        match self {
            Bst::Empty => false,
            Bst::Node(l, v, r) => {
                if x == v { true }
                else if x < v { l.member(x) }
                else { r.member(x) }
            }
        }
    }

    pub fn min_val(&self) -> Option<&T> {
        match self {
            Bst::Empty => None,
            Bst::Node(l, v, _) =>
                if matches!(l.as_ref(), Bst::Empty) { Some(v) } else { l.min_val() }
        }
    }
}
```

On `insert(x)` where `x < v`, the right subtree `r` is shared (`Rc::clone(r)` is O(1)) and a new node is created with a fresh left child. The right subtree is not copied. An O(log n) insertion allocates O(log n) new nodes and shares O(log n) existing nodes.

## OCaml Approach

```ocaml
type 'a bst = Empty | Node of 'a bst * 'a * 'a bst

let rec insert x = function
  | Empty -> Node (Empty, x, Empty)
  | Node (l, v, r) as t ->
    if x < v      then Node (insert x l, v, r)  (* r is shared (GC) *)
    else if x > v then Node (l, v, insert x r)  (* l is shared *)
    else t  (* duplicate: return same node (GC shares it) *)

let rec member x = function
  | Empty -> false
  | Node (l, v, r) ->
    if x = v then true
    else if x < v then member x l
    else member x r
```

OCaml's GC makes structural sharing automatic. `Node (insert x l, v, r)` shares `r` without explicit `Rc::clone` because the GC tracks references. The Rust `Rc::clone` call is the explicit acknowledgment of the same operation.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Sharing mechanism | `Rc::clone` — explicit O(1) | GC — implicit |
| `T: Clone` bound | Required to copy `v` into new nodes | No bound; values are copied by the runtime |
| Duplicate insert | `self.clone()` | Return `t` (same pointer via GC) |
| Node allocation | `Rc::new(...)` | Constructor (GC managed) |

Persistent BSTs are the foundation of OCaml's `Map` and `Set` modules. Each `Map.add` or `Set.add` returns a new tree sharing unchanged branches — O(log n) allocation, O(log n) lookup.

## Exercises

1. Implement `delete(&self, x: &T) -> Bst<T>` — handle leaf, one-child, and two-child cases.
2. Implement `to_sorted_vec(&self) -> Vec<T>` via in-order traversal.
3. Implement `size(&self) -> usize` recursively and cache it in `Node` for O(1) lookup.
4. Implement a balanced persistent BST by adding AVL or red-black rebalancing to `insert`.
5. Verify structural sharing: insert 10 elements, show that each version shares nodes with the previous by comparing `Rc::ptr_eq` on unchanged subtrees.
