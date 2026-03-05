# 608: Catamorphism (Fold) Generalized

**Difficulty:** 5  **Level:** Master

Every recursive function that consumes a tree is secretly a fold — `cata` makes this explicit so you can write the traversal once and plug in any logic.

## The Problem This Solves

You've written `tree_sum`. Then `tree_max`. Then `tree_depth`. Then `tree_to_list`. Each one looks like this:

```rust
fn tree_sum(t: &Tree<i64>) -> i64 {
    match t {
        Tree::Leaf(n)      => *n,
        Tree::Branch(l, r) => tree_sum(l) + tree_sum(r),  // recurse left, recurse right
    }
}

fn tree_depth(t: &Tree<i64>) -> usize {
    match t {
        Tree::Leaf(_)      => 0,
        Tree::Branch(l, r) => 1 + tree_depth(l).max(tree_depth(r)),  // same structure!
    }
}
```

The recursion pattern — visit both children, combine — is identical in every function. Only the combination logic differs. You are copying and pasting the structure of recursion while changing only the payload.

A `cata` (catamorphism) extracts the recursion into one function parameterized by the combination logic. Everything else — how to traverse, which subtrees to visit, how deep to go — is handled automatically. You write only "what to do with one node once its children are already processed".

This isn't academic. You see this pattern in real codebases whenever someone builds a query processor, AST evaluator, configuration parser, or JSON transformer. The fold is always there — just usually unnamed and duplicated.

## The Intuition

Think about how `Vec::fold` works:

```rust
let sum = vec![1, 2, 3, 4, 5].iter().fold(0, |acc, x| acc + x);
```

You supply:
1. A starting value for the empty case
2. A function for "given accumulated result so far and one element, produce the next accumulated result"

`fold` handles the looping. You provide the logic.

`cata` is the same contract for trees. You supply:
1. A function for leaves: "given the leaf value, produce a result"
2. A function for branches: "given the *already computed* results from left and right subtrees, combine them"

By the time your branch function is called, both subtrees are fully processed. You never recurse manually — you just combine.

```
tree_sum with cata:

       Branch
      /       \
  Branch      Branch        ← cata visits bottom-up
  /    \      /    \
Leaf(1) Leaf(2) Leaf(3) Leaf(4)

Step 1: Leaf(1) → leaf_alg(1) → 1
Step 2: Leaf(2) → leaf_alg(2) → 2
Step 3: Branch(1, 2) → branch_alg(1, 2) → 3
Step 4: Leaf(3) → leaf_alg(3) → 3
Step 5: Leaf(4) → leaf_alg(4) → 4
Step 6: Branch(3, 4) → branch_alg(3, 4) → 7
Step 7: Branch(3, 7) → branch_alg(3, 7) → 10
```

## How It Works in Rust

**The recursive type:**

```rust
#[derive(Debug, Clone)]
enum Tree<A> {
    Leaf(A),
    Branch(Box<Tree<A>>, Box<Tree<A>>),
}
```

**`cata_tree` — the generic fold:**

```rust
fn cata_tree<A: Clone, R>(
    tree:       &Tree<A>,
    leaf_alg:   impl Fn(A) -> R + Copy,         // what to do with a leaf value
    branch_alg: impl Fn(R, R) -> R + Copy,      // how to combine two child results
) -> R {
    match tree {
        Tree::Leaf(a)        => leaf_alg(a.clone()),
        Tree::Branch(l, r)   => branch_alg(
            cata_tree(l, leaf_alg, branch_alg),   // recurse left → get R
            cata_tree(r, leaf_alg, branch_alg),   // recurse right → get R
        ),  // combine two Rs into one R — that's your only job
    }
}
```

**Multiple operations, one traversal engine:**

```rust
// Sum all leaf values
fn tree_sum(t: &Tree<i64>) -> i64 {
    cata_tree(t, |x| x, |l, r| l + r)
}

// Maximum leaf value
fn tree_max(t: &Tree<i64>) -> i64 {
    cata_tree(t, |x| x, i64::max)
}

// Depth (longest path from root to leaf)
fn tree_depth(t: &Tree<i64>) -> usize {
    cata_tree(t, |_| 0, |l, r| 1 + l.max(r))
}

// Flatten to a Vec (in-order)
fn tree_to_list(t: &Tree<i64>) -> Vec<i64> {
    cata_tree(t, |x| vec![x], |mut l, r| { l.extend(r); l })
}
```

All of these are one-liners now. The recursion is in `cata_tree`, written once.

**Catamorphisms on other types — natural numbers:**

The concept isn't limited to trees. Any recursive type has a catamorphism. Here it is for natural numbers (represented as recursion depth):

```rust
fn cata_nat<R>(zero: R, succ: impl Fn(R) -> R + Clone, n: u64) -> R {
    if n == 0 { zero }            // base case
    else { succ(cata_nat(zero, succ.clone(), n - 1)) }  // recursive case
}

// "5 + 3" defined as: start with 3, apply "add one" five times
let five_plus_three = cata_nat(3u64, |n| n + 1, 5);
assert_eq!(five_plus_three, 8);
```

**List fold is a catamorphism too:**

```rust
let xs = vec![1i64, 2, 3, 4, 5];
let sum:  i64 = xs.iter().copied().fold(0, |acc, x| acc + x);
let prod: i64 = xs.iter().copied().fold(1, |acc, x| acc * x);
```

`Vec::fold` is a catamorphism on lists. The pattern is universal — lists, trees, natural numbers, expressions — all the same idea.

## What This Unlocks

- **One traversal engine, unlimited operations.** Add `tree_variance`, `tree_serialize`, `tree_pretty_print` — each is two small functions, no recursion. The cata handles all the traversal for every new operation you add.
- **Easy testing.** Your leaf and branch functions operate on plain values, not recursive structures. Unit-test them independently, then know they'll compose correctly under `cata`.
- **Appears in real codebases.** Rust compiler itself folds over HIR/MIR nodes. `serde` traverses value trees. Database query engines evaluate expression trees. The pattern is everywhere — you're just naming it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Cata | `fold` on lists, structural recursion on ADTs | Generic `cata_tree` function with two algebra parameters |
| Leaf algebra | `'a -> 'b` function | `impl Fn(A) -> R + Copy` |
| Branch algebra | `'b -> 'b -> 'b` function | `impl Fn(R, R) -> R + Copy` |
| List cata | `List.fold_right` | `.fold()` on iterators |
| Natural number cata | Church numerals / fold on `nat` | `cata_nat` with zero and succ functions |
