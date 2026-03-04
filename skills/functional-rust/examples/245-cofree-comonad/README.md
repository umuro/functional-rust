# 245: Cofree Comonad

**Difficulty:** 5  **Level:** Master

A tree where every node holds a label AND a subtree — the universal comonad for any functor.

## The Problem This Solves

You're building a game state machine. Each state has a label (the current game state) and a list of possible next states. You want to annotate every reachable state with derived information — say, the minimum number of moves to win from there. Without the right abstraction, this requires either mutable global maps or deeply nested explicit recursion.

The Cofree comonad solves this: it's a tree structure where every node has a value *and* a collection of children, and `extend` lets you replace every node's value with a function of its entire subtree. Want every node annotated with "sum of values in my subtree"? One `extend` call. Want every node annotated with "depth of my subtree"? Another one.

More concretely: if you have a file system tree (each directory has a name and a list of child directories), `extend` on the Cofree comonad lets you annotate every directory with its total size — in a single, structurally recursive pass.

## The Intuition

The **Cofree comonad over a functor F** is the "most general" comonad you can build from F. For `F = Vec` (list of children), it's a **rose tree** — a tree where each node has any number of children.

Think of it this way: regular `fmap` gives each node a new label based on its *own* value. `extend` gives each node a new label based on its *entire subtree* — the node, its children, their children, all the way down.

**`duplicate`** is the most powerful operation: it replaces every node's label with *the subtree rooted at that node*. After `duplicate`, the root's label is the whole original tree. The root's first child's label is the subtree rooted at that child. And so on. You now have a "tree of trees" — every possible substructure is labeled and accessible.

The relationship between `extend` and `duplicate`:
- `extend f = fmap f . duplicate`
- To compute a property of every subtree: `duplicate` first (now each node holds its own subtree as a label), then `fmap` with your property function.

**Why "cofree"?** The Cofree comonad is the right adjoint of the forgetful functor from comonads to functors. Every comonad for functor F embeds into `Cofree F`. It's the "freest" possible comonad — the one with the least extra structure.

## How It Works in Rust

The rose tree as Cofree over Vec:
```rust
pub struct Rose<A> {
    pub value: A,
    pub children: Vec<Rose<A>>,
}
```

Comonad operations:
```rust
impl<A: Clone> Rose<A> {
    pub fn extract(&self) -> A { self.value.clone() }

    // fmap: transform each node's label independently
    pub fn fmap<B>(&self, f: &impl Fn(A) -> B) -> Rose<B> {
        Rose {
            value: f(self.value.clone()),
            children: self.children.iter().map(|c| c.fmap(f)).collect(),
        }
    }

    // extend: replace each node's label with f applied to the *subtree there*
    // f receives the whole subtree rooted at each node
    pub fn extend<B>(&self, f: &impl Fn(&Rose<A>) -> B) -> Rose<B> {
        Rose {
            value: f(self),            // f sees the entire subtree at this point
            children: self.children.iter().map(|c| c.extend(f)).collect(),
        }
    }

    // duplicate: label each node with its own subtree
    pub fn duplicate(&self) -> Rose<Rose<A>> {
        Rose {
            value: self.clone(),       // root's label = the whole tree
            children: self.children.iter().map(|c| c.duplicate()).collect(),
        }
    }
}
```

Annotating every node with its subtree sum:
```rust
let tree = Rose::node(1, vec![
    Rose::node(2, vec![Rose::leaf(4), Rose::leaf(5)]),
    Rose::node(3, vec![Rose::leaf(6), Rose::node(7, vec![Rose::leaf(8)])]),
]);

// Annotate each node with the sum of values in its subtree
let annotated: Rose<i32> = tree.extend(&|subtree| subtree.sum());
// annotated.extract() == 36 (sum of entire tree)
// annotated.children[0].extract() == 11 (sum of subtree 2→[4,5])
```

## What This Unlocks

- **Annotation propagation**: any bottom-up tree computation (subtree sums, depths, counts) is a single `extend` call — no explicit recursion needed.
- **Game trees / decision trees**: `duplicate` exposes every subgame; `extend` annotates each position with its value under minimax or alpha-beta.
- **Document processing**: annotate every node in a syntax/document tree with derived attributes (line counts, word counts, validation results).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Cofree/Rose type | `type 'a rose = Rose of 'a * 'a rose list` | `struct Rose<A> { value: A, children: Vec<Rose<A>> }` |
| fmap | `let rec fmap f (Rose(v, cs)) = Rose(f v, List.map (fmap f) cs)` | Method with `&impl Fn(A) -> B` |
| extend | Recursive, same pattern as fmap but `f` receives whole subtree | Method; `f` receives `&Rose<A>` |
| Sharing subtrees | GC allows aliasing freely | `duplicate` requires `Clone` — full copy |
| Functor abstraction | Can parameterize over functor F via modules | `Rose<A>` is concrete; parameterizing over F needs GATs or trait objects |
