# 216: Fix Point — How Recursive Types Work Under the Hood

**Difficulty:** ⭐⭐⭐  **Category:** Recursion Schemes

A single generic wrapper that turns any non-recursive shape into a fully recursive data structure.

## The Problem This Solves

When you define a recursive type in Rust, the recursion and the shape are bundled together:

```rust
enum List {
    Nil,
    Cons(i64, Box<List>),  // shape AND recursion in one definition
}
```

This works fine until you want to write *generic* code over *any* recursive structure. Say you want one fold function that works for lists, trees, expressions, and anything else you might define. You can't — the recursion is baked into each type separately.

What you really want to do is separate the two concerns:
1. **The shape** — what a single node looks like (its variant names and payload types)
2. **The recursion** — how nodes contain other nodes

If you can separate these, you can write one `cata` (fold) function that works for any structure by parameterizing over the shape. You'd also be able to swap in different types for the "child" slot — put `i64` there and children become computed values; put `String` and they become rendered text; put the structure itself and you get full recursion.

The fix point is the bridge: it's the generic wrapper that takes a *non-recursive shape* and makes it recursive. Every recursive type you've ever written can be seen as `Fix<SomeShape>`.

## The Intuition

The word "fixed point" comes from math (don't panic): a fixed point of a function `f` is a value `x` where `f(x) = x`. That is, applying `f` doesn't change it.

In types: the fix point of a type constructor `F` is a type `T` where `T = F<T>`. A type that equals itself when you plug it into `F`.

Concretely — a linked list is a type `T` where `T = either Nil or (i64, T)`. That IS the definition of `List`. So `List` is the fixed point of `ListF`, where `ListF<A>` is "either Nil or (i64, A)".

In Rust we write this literally:

```rust
// The shape — one node, children are generic A
enum ListF<A> {
    NilF,
    ConsF(i64, A),
}

// The fixed point — wraps itself: Fix contains ListF<Fix>
struct FixList(Box<ListF<FixList>>);
//                        ^^^^^
//                        children ARE FixList — full recursion restored
```

Think of `FixList` as a Russian doll: the outermost doll IS the structure, and when you open it (`unfix`), you find `ListF<FixList>` — the same type of doll inside. Dolls all the way down, until you hit `NilF` (no child doll).

## How It Works in Rust

**Step 1: Define the shape (non-recursive functor)**

Take any recursive type and replace all recursive children with `A`:

```rust
// ListF: shape of a list node. A = "what goes in the child position"
enum ListF<A> {
    NilF,           // the end — no child
    ConsF(i64, A),  // a value plus one child
}
```

```rust
// TreeF: shape of a binary tree node
enum TreeF<A> {
    LeafF(i64),    // leaf — no children
    BranchF(A, A), // two children
}
```

**Step 2: Add `map` — transforms children**

This is required for `cata` to work. It applies a function to every child:

```rust
impl<A> ListF<A> {
    // Apply f to every child — NilF has none, ConsF has one
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ListF<B> {
        match self {
            ListF::NilF        => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(*x, f(a)),
        }
    }
}
```

**Step 3: Create the fixed point wrapper**

```rust
// FixList IS a ListF<FixList> — the type equation T = ListF<T> solved
struct FixList(Box<ListF<FixList>>);

// Convenience constructors
fn nil() -> FixList { FixList(Box::new(ListF::NilF)) }
fn cons(x: i64, xs: FixList) -> FixList { FixList(Box::new(ListF::ConsF(x, xs))) }

// Usage: same as a normal linked list
let xs = cons(1, cons(2, cons(3, nil())));
```

**Step 4: Write `cata` — the universal fold**

Once you have a fix point, `cata` is always the same pattern:

```rust
fn cata_list<A>(alg: &dyn Fn(ListF<A>) -> A, fix: &FixList) -> A {
    // 1. Get the node: fix.0 is a ListF<FixList>
    // 2. map_ref recursively evaluates all children first
    // 3. Now we have a ListF<A> — children already computed
    // 4. Pass it to the algebra to produce the final A
    alg(fix.0.map_ref(|child| cata_list(alg, child)))
}
```

**Step 5: Write algebras — pure logic, no recursion**

```rust
// Sum all elements
let sum_alg = |l: ListF<i64>| match l {
    ListF::NilF        => 0,         // empty: 0
    ListF::ConsF(x, a) => x + a,     // x + (already-summed tail)
};
assert_eq!(cata_list(&sum_alg, &xs), 6);

// Count elements
let len_alg = |l: ListF<usize>| match l {
    ListF::NilF        => 0,
    ListF::ConsF(_, a) => 1 + a,
};
assert_eq!(cata_list(&len_alg, &xs), 3);
```

The exact same pattern works for trees:

```rust
struct FixTree(Box<TreeF<FixTree>>);

fn leaf(n: i64) -> FixTree { FixTree(Box::new(TreeF::LeafF(n))) }
fn branch(l: FixTree, r: FixTree) -> FixTree { FixTree(Box::new(TreeF::BranchF(l, r))) }

fn cata_tree<A>(alg: &dyn Fn(TreeF<A>) -> A, fix: &FixTree) -> A {
    alg(fix.0.map_ref(|child| cata_tree(alg, child)))
}

let tree = branch(branch(leaf(1), leaf(2)), leaf(3));
let sum_tree = |t: TreeF<i64>| match t {
    TreeF::LeafF(n)      => n,
    TreeF::BranchF(l, r) => l + r,
};
assert_eq!(cata_tree(&sum_tree, &tree), 6);
```

Notice: the `cata` functions for lists and trees are *structurally identical*. The fix point is what makes this uniformity possible.

## What This Unlocks

- **Generic recursion-scheme machinery.** Once you have `Fix<F>` and `map` for your functor `F`, you get `cata` (fold), `ana` (unfold), and `hylo` (build-then-fold) for free — no per-type implementation needed.
- **Separate shape from recursion.** You can test your node shape with simple `A` values before worrying about the recursive structure.
- **Foundation for compilers and interpreters.** Real-world tools like expression evaluators, query optimizers, and AST transformers use this pattern — often without calling it by name.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fix type | `type 'a fix = In of 'a fix` or a module functor | `struct FixList(Box<ListF<FixList>>)` — one per shape |
| Box requirement | Not needed (GC manages heap) | Required — Rust needs explicit heap allocation for recursive types |
| Generic fix | Single `Fix` type via module functors | Separate `FixList`, `FixTree`, etc. (GATs can generalize this) |
| Functor map | Standalone function, usually polymorphic | Method on the concrete enum |
| Unfix | `let (In f) = x` pattern | `fix.0` or custom `unfix()` method |
