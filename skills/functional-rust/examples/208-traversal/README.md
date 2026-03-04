# 208: Traversal

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

A Lens that focuses on multiple targets at once — apply a function to every element of a `Vec`, every leaf of a tree, or every matching value, collecting all results.

## The Problem This Solves

You have a data structure — a list, a tree, a config object with many optional fields — and you want to transform all of a certain kind of value inside it. With a Lens you can reach *one* field. But what about *all* elements? What about all the leaf nodes in a tree? What about every even number in a list?

The naive approach is to write a custom function for every case:

```rust
fn double_all(xs: Vec<i32>) -> Vec<i32> { xs.into_iter().map(|x| x * 2).collect() }
fn double_evens(xs: Vec<i32>) -> Vec<i32> { xs.into_iter().map(|x| if x % 2 == 0 { x * 2 } else { x }).collect() }
fn double_leaves(t: Tree<i32>) -> Tree<i32> { /* recursive case ... */ }
```

Each of these is bespoke. You can't compose them. You can't write a generic "count the targets" or "collect all targets into a list" without reimplementing it for each structure. You can't make a filtered version that targets only some elements.

A Traversal packages the "how to walk and modify this structure" logic into a reusable value, letting you write `over`, `collect`, `length_of`, `sum_of`, `all_of`, `any_of` once and use them with any Traversal. This exists to solve exactly that pain.

## The Intuition

A **Lens** focuses on **exactly one** value inside a structure (e.g., the `x` field of a `Point`).

A **Traversal** is a Lens that can focus on **zero or more** values at once.

Think of a Traversal as a reusable "visiting pattern." It encodes *where to look* inside a structure, and gives you two fundamental operations:

1. **`over`** (modify all) — apply a function to every focused value and rebuild the structure.
2. **`collect`** (list all) — extract every focused value into a `Vec`.

Everything else is derived: count them (`length_of`), sum them (`sum_of`), check all (`all_of`), check any (`any_of`).

**Analogy:** A Traversal is like a CSS selector. The selector `.foo` says "target all elements with class foo." You can then `.map()` over them, `.filter()` them, count them — but the *selector itself* is reusable and composable, separate from what you do with it.

```rust
// A Traversal carries two pieces of logic:
struct Traversal<S, A> {
    over:    Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>,  // modify all focused A's in S
    to_list: Box<dyn Fn(&S) -> Vec<A>>,                // collect all focused A's
}
//
// S = the structure (e.g., Vec<i32>, Tree<String>)
// A = the focused element type (e.g., i32, String)
```

The key insight: `traverse` applies an *effectful* function to each target and collects results. This is how `Option<Vec<T>>` ↔ `Vec<Option<T>>` flipping works — you traverse a `Vec` with a function that returns `Option`, and the traversal collects into `Option<Vec<T>>`, short-circuiting on the first `None`.

## How It Works in Rust

```rust
// Step 1: The Traversal type — two operations bundled together
struct Traversal<S, A> {
    over:    Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>,
    to_list: Box<dyn Fn(&S) -> Vec<A>>,
}

impl<S: 'static, A: 'static> Traversal<S, A> {
    fn modify(&self, f: &dyn Fn(&A) -> A, s: &S) -> S { (self.over)(f, s) }
    fn collect(&self, s: &S) -> Vec<A>               { (self.to_list)(s) }

    // Derived combinators — built once, work for any Traversal
    fn length_of(&self, s: &S) -> usize { self.collect(s).len() }
}

// Step 2: A simple Traversal — target every element of a Vec
fn each_traversal<T: Clone + 'static>() -> Traversal<Vec<T>, T> {
    Traversal {
        over:    Box::new(|f, xs| xs.iter().map(|x| f(x)).collect()),
        to_list: Box::new(|xs| xs.clone()),
    }
}

let t = each_traversal::<i32>();
let xs = vec![1, 2, 3, 4, 5];
t.collect(&xs);               // [1, 2, 3, 4, 5]
t.modify(&|x| x * 2, &xs);   // [2, 4, 6, 8, 10]
t.length_of(&xs);             // 5

// Step 3: A Traversal over a tree — visits every Leaf
fn each_leaf<T: Clone + 'static>() -> Traversal<Tree<T>, T> {
    fn walk<T: Clone>(f: &dyn Fn(&T) -> T, t: &Tree<T>) -> Tree<T> {
        match t {
            Tree::Leaf(x)       => Tree::Leaf(f(x)),          // apply f at leaves
            Tree::Branch(l, r)  => Tree::Branch(             // recurse into branches
                Box::new(walk(f, l)),
                Box::new(walk(f, r)),
            ),
        }
    }
    // ... (to_list recursively collects leaves)
}

let tree = Tree::Branch(
    Box::new(Tree::Leaf(1)),
    Box::new(Tree::Leaf(2)),
);
each_leaf::<i32>().collect(&tree);             // [1, 2]
each_leaf::<i32>().modify(&|x| x + 10, &tree); // Branch(Leaf(11), Leaf(12))

// Step 4: Filtered Traversal — target only elements matching a predicate
fn filtered<T: Clone + 'static>(
    pred: impl Fn(&T) -> bool + Clone + 'static,
) -> Traversal<Vec<T>, T> {
    let p1 = pred.clone();
    let p2 = pred;
    Traversal {
        // over: apply f only to matching elements, pass others through unchanged
        over: Box::new(move |f, xs| {
            xs.iter().map(|x| if p1(x) { f(x) } else { x.clone() }).collect()
        }),
        // to_list: only include matching elements
        to_list: Box::new(move |xs| xs.iter().filter(|x| p2(x)).cloned().collect()),
    }
}

let evens = filtered(|x: &i32| x % 2 == 0);
evens.collect(&xs);              // [2, 4]
evens.modify(&|x| x * 10, &xs); // [1, 20, 3, 40, 5]  — odds unchanged

// Step 5: Generic combinators that work with any Traversal
fn sum_of(t: &Traversal<Vec<i32>, i32>, s: &Vec<i32>) -> i32 { t.collect(s).iter().sum() }
fn all_of<S, A>(t: &Traversal<S, A>, pred: impl Fn(&A) -> bool, s: &S) -> bool {
    t.collect(s).iter().all(|a| pred(a))
}
```

## What This Unlocks

- **Optics composition** — a Traversal is the "read-write" generalisation of both Lens and Prism. Any Lens or Prism can be treated as a degenerate Traversal. Understanding Traversal is the key to understanding the full optics hierarchy (example 211).
- **Sequence/traverse for `Option` and `Result`** — `traverse_opt(xs, f)` applies `f: A → Option<B>` to every element and either returns `Some(Vec<B>)` or `None` (short-circuits on first failure). This pattern appears in every data validation pipeline.
- **Tree/graph walks** — any recursive structure (AST nodes, directory trees, JSON values) can have a Traversal defined for it, enabling generic "apply transformation to all nodes of type X" without bespoke recursive functions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Traversal value | Record `{ over; to_list }` with polymorphic function types | `struct Traversal<S, A>` with `Box<dyn Fn>` trait objects |
| `over` function type | `('a -> 'a) -> 's -> 's` (polymorphic, curried) | `&dyn Fn(&A) -> A` — reference to trait object |
| Vec mapping | `List.map f xs` | `xs.iter().map(f).collect()` |
| Tree recursion | Pattern match, no boxing needed | Same pattern, but recursive types need `Box<Tree<T>>` |
| Filtered traversal | Predicate closure, no clone needed (GC) | Predicate must be `Clone` — stored in both `over` and `to_list` closures |
