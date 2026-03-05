📖 **[View on hightechmind.io →](https://hightechmind.io/rust/610-hylomorphism-rust)**

---

# 610: Hylomorphism — Unfold Then Fold

**Difficulty:** 5  **Level:** Master

Many algorithms secretly build a tree and immediately destroy it — hylomorphism names this pattern and lets you prove it correct by reasoning about each half separately.

## The Problem This Solves

Look at merge sort. Here's the algorithm:
1. If the list has 0 or 1 elements, it's already sorted
2. Split it in half
3. Sort the left half recursively
4. Sort the right half recursively
5. Merge the two sorted halves

Steps 2-4 are building a binary tree of sublists. Step 5 is collapsing that tree by merging. The tree is never stored anywhere — it's implicit in the call stack. But it's there.

The same structure appears in:
- **Factorial**: build a countdown `[n, n-1, ..., 1]`, then multiply
- **Sum of range**: build `[n, n-1, ..., 1]`, then add
- **FFT**: build a butterfly tree of sub-problems, then combine
- **Compilers**: unfold source into a parse tree, fold into bytecode

These all follow the same two-phase shape: *expand* (anamorphism), then *collapse* (catamorphism). A **hylomorphism** is the composition of these two phases — but crucially, it's a *fused* composition. The intermediate tree is never heap-allocated; it lives only in the call stack.

Beyond efficiency, the pattern has a correctness benefit: you can prove the algorithm correct by proving the two halves correct independently. Is the split always valid? Does the merge always produce a sorted output? Prove each one separately, then know the composition is correct.

## The Intuition

The name comes from Aristotle's "hylomorphism" — matter (hyle) + form (morphe). The coalgebra provides the *matter* (raw structure, split into parts); the algebra provides the *form* (the recombination logic).

Here's the mental model for merge sort:

```
Input: [3, 1, 4, 1, 5]

Coalgebra (split):         Algebra (merge):
[3, 1, 4, 1, 5]            —
├── [3, 1]                  —
│   ├── [3]     → [3]       ↑ merge([3], [1]) = [1, 3]
│   └── [1]     → [1]       ↑
└── [4, 1, 5]               —
    ├── [4]     → [4]       ↑ merge([4], [1, 5]) = [1, 4, 5]
    └── [1, 5]              —
        ├── [1] → [1]       ↑ merge([1], [5]) = [1, 5]
        └── [5] → [5]       ↑
                            merge([1,3], [1,4,5]) = [1,1,3,4,5] ✓
```

The tree in the middle is never stored as a `Vec<Vec<i64>>` in memory. It's just the recursion structure, handled automatically by `hylo`.

## How It Works in Rust

**The `hylo` function — fused unfold + fold:**

```rust
fn hylo<S: Clone, A, R>(
    seed:  S,
    coalg: impl Fn(S) -> Option<(A, S)> + Copy,   // expand: seed → (value, next_seed) or stop
    alg:   impl Fn(A, R) -> R,                     // combine: (value, child_result) → result
    base:  R,                                       // result for the base case (stop)
) -> R {
    match coalg(seed) {
        None             => base,
        Some((a, next)) => alg(a, hylo(next, coalg, alg, base)),
        //                      ^   ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                      |   recurse into next seed first → get R
        //                      combine this level's value with child result
    }
}
```

**Factorial — unfold countdown, fold by multiplying:**

```rust
fn factorial(n: u64) -> u64 {
    hylo(
        n,
        |k| if k <= 1 { None } else { Some((k, k - 1)) },  // coalg: count down
        |k, acc| k * acc,                                     // alg: multiply
        1,                                                     // base: empty product
    )
}

assert_eq!(factorial(5), 120);
assert_eq!(factorial(10), 3628800);
```

**Sum of 1..=n:**

```rust
fn sum_to(n: u64) -> u64 {
    hylo(
        n,
        |k| if k == 0 { None } else { Some((k, k - 1)) },
        |k, acc| k + acc,
        0,
    )
}

assert_eq!(sum_to(10), 55);   // 1+2+...+10
```

**Merge sort — hylomorphism over a binary tree:**

```rust
// The intermediate structure: a tree of sublists (never heap-allocated as a whole)
#[derive(Debug)]
enum SortTree<A> {
    Leaf(A),
    Branch(Box<SortTree<A>>, Box<SortTree<A>>),
}

// Coalgebra: split a slice into a tree
fn split_to_tree<A: Clone>(xs: &[A]) -> Option<SortTree<A>> {
    match xs {
        []  => None,         // empty → no tree
        [x] => Some(SortTree::Leaf(x.clone())),  // singleton → leaf
        _   => {
            let mid = xs.len() / 2;
            let l = split_to_tree(&xs[..mid]);
            let r = split_to_tree(&xs[mid..]);
            match (l, r) {
                (Some(l), Some(r)) => Some(SortTree::Branch(Box::new(l), Box::new(r))),
                (Some(l), None) | (None, Some(l)) => Some(l),
                _ => None,
            }
        }
    }
}

// Algebra: merge two sorted lists
fn merge_sorted<A: Ord + Clone>(a: Vec<A>, b: Vec<A>) -> Vec<A> {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { result.push(a[i].clone()); i += 1; }
        else             { result.push(b[j].clone()); j += 1; }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

// Fold over the split tree
fn fold_sort<A: Ord + Clone>(t: SortTree<A>) -> Vec<A> {
    match t {
        SortTree::Leaf(x)       => vec![x],
        SortTree::Branch(l, r)  => merge_sorted(fold_sort(*l), fold_sort(*r)),
    }
}

// Merge sort = split (ana) then merge (cata)
fn merge_sort<A: Ord + Clone>(xs: &[A]) -> Vec<A> {
    match split_to_tree(xs) {
        None       => vec![],
        Some(tree) => fold_sort(tree),
    }
}

assert_eq!(merge_sort(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3]), vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
assert_eq!(merge_sort(&["banana", "apple", "cherry"]), vec!["apple", "banana", "cherry"]);
```

**Proving correctness — the real payoff:**

Because the coalgebra and algebra are separate functions, you can test them independently:
- Is `split_to_tree` always valid? Test it on its own — does it cover all elements? Does it terminate?
- Is `merge_sorted` correct? Test it on its own with known sorted inputs.
- If both pass, `merge_sort` is correct by construction.

## What This Unlocks

- **Name and isolate divide-and-conquer algorithms.** Merge sort, FFT, quicksort, and similar all fit this shape. Recognizing it makes the structure explicit and testable.
- **Prove correctness in parts.** Verify the coalgebra (correct splitting) and algebra (correct merging) independently. Hylo composition is automatically correct if the parts are.
- **Compiler pipeline insight.** Source → tokens (unfold) → AST → bytecode (fold) is a hylomorphism chain. Understanding this helps when designing language tools and interpreters.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hylo definition | `cata alg ∘ ana coalg` (composition) | Explicit function or separated `split_to_tree` + `fold_sort` |
| Stream fusion | GHC RULES pragma fuses automatically | Rust iterators fuse via adapters; manual for trees |
| Divide & conquer | Recursion scheme directly | Hylo pattern — same semantics |
| Generic sort | Polymorphic with type classes | `A: Ord + Clone` bounds |
| Intermediate tree | Implicit in OCaml's lazy evaluation | Explicit `SortTree` or call-stack-only with full fusion |
