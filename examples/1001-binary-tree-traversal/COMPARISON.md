# OCaml vs Rust: Binary Tree — Size, Membership, Traversal

## Side-by-Side Code

### OCaml

```ocaml
type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec size = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + size l + size r

let rec depth = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)

let rec mem x = function
  | Leaf           -> false
  | Node (v, l, r) -> v = x || mem x l || mem x r

(* Linear-time preorder using accumulator trick *)
let preorder t =
  let rec go acc = function
    | Leaf           -> acc
    | Node (v, l, r) -> v :: go (go acc r) l
  in go [] t
```

### Rust (idiomatic)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

impl<T: PartialEq> Tree<T> {
    pub fn size(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(_, l, r) => 1 + l.size() + r.size(),
        }
    }

    pub fn mem(&self, x: &T) -> bool {
        match self {
            Tree::Leaf => false,
            Tree::Node(v, l, r) => v == x || l.mem(x) || r.mem(x),
        }
    }
}
```

### Rust (functional/recursive)

```rust
pub fn preorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    let mut result = Vec::new();
    preorder_go(tree, &mut result);
    result
}

fn preorder_go<T: Clone>(tree: &Tree<T>, acc: &mut Vec<T>) {
    if let Tree::Node(v, l, r) = tree {
        acc.push(v.clone());
        preorder_go(l, acc);
        preorder_go(r, acc);
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| tree type | `type 'a tree = Leaf \| Node of 'a * 'a tree * 'a tree` | `enum Tree<T> { Leaf, Node(T, Box<Tree<T>>, Box<Tree<T>>) }` |
| size | `val size : 'a tree -> int` | `fn size(&self) -> usize` |
| membership | `val mem : 'a -> 'a tree -> bool` | `fn mem(&self, x: &T) -> bool` (requires `T: PartialEq`) |
| preorder | `val preorder : 'a tree -> 'a list` | `fn preorder<T: Clone>(tree: &Tree<T>) -> Vec<T>` |

## Key Insights

1. **Box for heap allocation:** OCaml's runtime allocates all constructors on the heap automatically. Rust requires explicit `Box<Tree<T>>` inside the `Node` variant so the compiler can determine the enum's size at compile time — without `Box`, `Tree<T>` would be infinitely large.
2. **Trait bounds at the use site:** OCaml's polymorphic equality `=` works on any type at runtime. Rust's `mem` requires the explicit bound `T: PartialEq` — the compiler rejects the function without it. Bounds are declared where they are needed, not at the type definition.
3. **Mutable accumulator vs. functional accumulator:** OCaml's preorder uses the accumulator trick `go (go acc r) l` to achieve linear time without list concatenation. Rust uses a `&mut Vec<T>` shared across all recursive calls — semantically equivalent but expressed through mutable state rather than accumulator threading.
4. **Pattern matching syntax:** OCaml: `function | Leaf -> ... | Node(v, l, r) -> ...`. Rust: `match self { Tree::Leaf => ..., Tree::Node(v, l, r) => ... }`. The structure is identical; Rust requires qualified variant names and explicit `match`.
5. **Methods vs. free functions:** OCaml defines `size`, `depth`, and `mem` as top-level functions taking the tree as the last argument. Rust groups them as inherent methods on `impl<T: PartialEq> Tree<T>`, called as `tree.size()` — idiomatic for a data structure's core operations.

## When to Use Each Style

**Use idiomatic Rust when:** Implementing a data structure for a library — inherent methods on `impl Tree<T>` provide the cleanest API and integrate naturally with Rust's method call syntax.
**Use free functions when:** The operation is generic over the tree type and does not belong to the tree itself (e.g., serialization, drawing), or when demonstrating the OCaml parallel explicitly.
