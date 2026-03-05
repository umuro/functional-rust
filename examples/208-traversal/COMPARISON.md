# OCaml vs Rust: Traversal

## Side-by-Side Code

### OCaml

```ocaml
(* A traversal focuses on 0-to-many values inside a structure *)
type ('s, 'a) traversal = {
  over    : ('a -> 'a) -> 's -> 's;
  to_list : 's -> 'a list;
}

(* Vec equivalent: traverse every list element *)
let each_traversal : ('a list, 'a) traversal = {
  over    = List.map;
  to_list = Fun.id;
}

(* Tree leaf traversal *)
type 'a tree = Leaf of 'a | Branch of 'a tree * 'a tree

let rec tree_over f = function
  | Leaf x        -> Leaf (f x)
  | Branch (l, r) -> Branch (tree_over f l, tree_over f r)

let rec tree_to_list = function
  | Leaf x        -> [x]
  | Branch (l, r) -> tree_to_list l @ tree_to_list r

let each_leaf : ('a tree, 'a) traversal = {
  over    = tree_over;
  to_list = tree_to_list;
}

(* Derived operations — work with any traversal *)
let length_of trav s = List.length (trav.to_list s)
let sum_of    trav s = List.fold_left (+) 0 (trav.to_list s)
let all_of    trav s pred = List.for_all pred (trav.to_list s)
```

### Rust (idiomatic — struct with boxed closures)

```rust
type OverFn<S, A>    = Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>;
type ToListFn<S, A>  = Box<dyn Fn(&S) -> Vec<A>>;

pub struct Traversal<S, A> {
    over_fn:    OverFn<S, A>,
    to_list_fn: ToListFn<S, A>,
}

impl<S: 'static, A: 'static> Traversal<S, A> {
    pub fn over(&self, f: impl Fn(&A) -> A, s: &S) -> S {
        (self.over_fn)(&f, s)
    }
    pub fn collect_all(&self, s: &S) -> Vec<A> { (self.to_list_fn)(s) }
    pub fn length_of(&self, s: &S) -> usize    { self.collect_all(s).len() }
    pub fn all_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect_all(s).iter().all(pred)
    }
}

pub fn each_traversal<T: Clone + 'static>() -> Traversal<Vec<T>, T> {
    Traversal::new(
        |f, v: &Vec<T>| v.iter().map(f).collect(),
        |v: &Vec<T>| v.clone(),
    )
}
```

### Rust (functional/recursive — trait-based, zero allocation)

```rust
pub trait TraversableExt<A> {
    fn over_all(self, f: &impl Fn(A) -> A) -> Self;
    fn collect_targets(&self) -> Vec<A> where A: Clone;
}

impl<A> TraversableExt<A> for Vec<A> {
    fn over_all(self, f: &impl Fn(A) -> A) -> Self {
        self.into_iter().map(f).collect()
    }
    fn collect_targets(&self) -> Vec<A> where A: Clone { self.clone() }
}

fn tree_over_owned<A>(tree: Tree<A>, f: &impl Fn(A) -> A) -> Tree<A> {
    match tree {
        Tree::Leaf(x)        => Tree::Leaf(f(x)),
        Tree::Branch(l, r)   => Tree::Branch(
            Box::new(tree_over_owned(*l, f)),
            Box::new(tree_over_owned(*r, f)),
        ),
    }
}

impl<A: Clone> TraversableExt<A> for Tree<A> {
    fn over_all(self, f: &impl Fn(A) -> A) -> Self { tree_over_owned(self, f) }
    fn collect_targets(&self) -> Vec<A>             { tree_to_list(self) }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Traversal type | `('s, 'a) traversal` (record) | `Traversal<S, A>` (struct) |
| `over` | `('a -> 'a) -> 's -> 's` | `Fn(&A) -> A, &S) -> S` |
| `to_list` | `'s -> 'a list` | `Fn(&S) -> Vec<A>` |
| Polymorphic traversal | Type variable `'a` | Generic `<T>` with `'static` bound |
| Recursive tree walk | Pattern-matching `function` | `match` with `Box::new` for heap nodes |
| Trait-based approach | N/A (module functors) | `TraversableExt<A>` trait |

## Key Insights

1. **Records vs structs**: OCaml's record `{ over; to_list }` maps naturally to a Rust struct. Rust needs boxed closures (`Box<dyn Fn(...)>`) where OCaml uses higher-order functions directly, because Rust's closures are unnameable types.

2. **Lifetime and `'static` bounds**: Rust requires `S: 'static` and `A: 'static` when storing closures in a `Box<dyn Fn(...)>`, since the trait object must outlive the references it captures. OCaml's GC handles object lifetimes transparently.

3. **Two dispatch strategies**: The struct-based approach uses runtime dispatch (`dyn Fn`) — flexible, heap-allocated. The trait-based approach (`TraversableExt`) uses compile-time dispatch — zero overhead, but each container type must implement the trait manually. OCaml's functor system offers a third alternative unavailable in Rust.

4. **Recursive trees and ownership**: OCaml's `Branch of 'a tree * 'a tree` stores sub-trees inline. Rust must use `Box<Tree<A>>` to give recursive types a known size. Modifying an owned tree recursively requires consuming it (the `tree_over_owned` helper takes `Tree<A>` by value and returns a new one), matching the immutable-update style of OCaml exactly.

5. **Derived operations are universal**: `length_of`, `all_of`, `any_of` are defined once in terms of `collect_all` and work for *any* `Traversal<S, A>` or any `TraversableExt` instance — the same composability that makes OCaml's traversal abstraction powerful applies equally in Rust.

## When to Use Each Style

**Use struct-based `Traversal<S, A>` when:** you want to pass traversals as first-class values, store them in collections, or build combinators (e.g., `compose`, `filtered`) at runtime — matching the OCaml idiom most closely.

**Use trait-based `TraversableExt<A>` when:** performance matters, the set of traversable types is known at compile time, and you want zero-cost abstraction without heap allocation or runtime dispatch.
