[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Cofree Comonad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The Cofree comonad is the categorical dual of the Free monad. Where Free builds up effects lazily for later interpretation, Cofree builds up a potentially infinite tree of values annotated with a functor at every node. It is the universal comonad: every comonad arises as a coalgebra for some Cofree instance. In Rust, Cofree enables elegant tree-shaped data structures where each node carries both a value and a collection of children described by a functor.

## Learning Outcomes

- Understand Cofree as `Cofree f a = (a, f (Cofree f a))`
- Implement `extract` and `extend` for Cofree
- See how Cofree generalizes rose trees, streams, and annotated ASTs
- Use Cofree to annotate a recursive AST with type information
- Compare Cofree with OCaml's coinductive (lazy) equivalent

## Rust Application

Cofree in Rust requires `Box` to break the recursive definition:

```rust
use std::rc::Rc;

// Cofree f a = (a, f (Cofree f a))
// We use Vec as the functor f (making it a rose tree)
struct Cofree<A> {
    head: A,
    tail: Vec<Box<Cofree<A>>>,
}

impl<A: Clone> Cofree<A> {
    fn leaf(val: A) -> Self {
        Cofree { head: val, tail: vec![] }
    }

    fn node(val: A, children: Vec<Cofree<A>>) -> Self {
        Cofree {
            head: val,
            tail: children.into_iter().map(Box::new).collect(),
        }
    }

    // Comonad: extract — the value at the root
    fn extract(&self) -> &A { &self.head }

    // Comonad: extend — annotate every node with a context-dependent value
    fn extend<B: Clone>(&self, f: impl Fn(&Cofree<A>) -> B + Clone) -> Cofree<B> {
        Cofree {
            head: f(self),
            tail: self.tail.iter()
                .map(|child| Box::new(child.extend(f.clone())))
                .collect(),
        }
    }

    // Count nodes in the subtree (coalgebra)
    fn size(&self) -> usize {
        1 + self.tail.iter().map(|c| c.size()).sum::<usize>()
    }

    // Collect all values in depth-first order
    fn values(&self) -> Vec<&A> {
        let mut result = vec![&self.head];
        for child in &self.tail {
            result.extend(child.values());
        }
        result
    }
}

// Annotated AST example: annotate each node with subtree depth
fn depth<A: Clone>(tree: &Cofree<A>) -> usize {
    if tree.tail.is_empty() {
        0
    } else {
        1 + tree.tail.iter().map(|c| depth(c)).max().unwrap_or(0)
    }
}

fn main() {
    // Build a rose tree: root(1) -> [node(2) -> [leaf(4), leaf(5)], leaf(3)]
    let tree = Cofree::node(1, vec![
        Cofree::node(2, vec![
            Cofree::leaf(4),
            Cofree::leaf(5),
        ]),
        Cofree::leaf(3),
    ]);

    println!("Root: {}", tree.extract());        // 1
    println!("Size: {}", tree.size());            // 5
    println!("Values: {:?}", tree.values());      // [1, 2, 4, 5, 3]

    // Annotate each node with its subtree size using extend
    let sized = tree.extend(|node| node.size());
    println!("Root subtree size: {}", sized.extract()); // 5

    // Annotate each node with its depth
    let depths = tree.extend(|node| depth(node));
    println!("Root depth: {}", depths.extract()); // 2
}
```

The `extend` operation is what makes Cofree a comonad: it annotates every node in the tree with a function of the subtree rooted at that node, enabling bottom-up tree annotations.

## OCaml Approach

OCaml's lazy evaluation makes Cofree natural for infinite structures:

```ocaml
type ('f, 'a) cofree = Cofree of 'a * ('f, 'a) cofree list Lazy.t

let extract (Cofree (a, _)) = a

let rec extend f tree =
  let (Cofree (_, children)) = tree in
  Cofree (f tree, lazy (List.map (extend f) (Lazy.force children)))

(* Infinite stream as Cofree over unit functor *)
let rec nats n = Cofree (n, lazy [nats (n + 1)])
```

OCaml's `Lazy.t` avoids the need for explicit `Box` and enables genuinely infinite Cofree structures without stack overflow.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Recursion breaking | `Box<Cofree<A>>` | `Lazy.t` list |
| Infinite structures | requires `Rc`/arena tricks | natural with `Lazy.t` |
| extend cloning | requires `A: Clone + 'static` | no explicit bounds |
| Functor parameter | baked in as `Vec` | polymorphic over list functor |
| Coinduction | not native | `codata` via laziness |

A fully generic `Cofree<F, A>` in Rust requires HKT simulation (GATs), making the Vec-specialized version far more practical for most use cases.

## Exercises

1. Implement Cofree with `Option` as the functor (making it a potentially-terminated stream).
2. Use `extend` to implement a Huffman encoding annotator: label each node with its subtree character frequency.
3. Implement `unfold: S -> (A, Vec<S>) -> Cofree<A>` as the Cofree anamorphism (corecursive unfold).
4. Show that `duplicate` (the Cofree version of `extend id`) produces a tree where each node's annotation is the subtree rooted there.
5. Implement a Cofree-based attribute grammar evaluator: given a grammar tree and inherited/synthesized attribute rules, use `extend` to compute the attributes in one traversal.
