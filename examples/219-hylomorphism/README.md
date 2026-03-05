📖 **[View on hightechmind.io →](https://hightechmind.io/rust/219-hylomorphism)**

---

# 219: Hylomorphism — Build Then Fold, in One Pass

**Difficulty:** ⭐⭐⭐  **Category:** Recursion Schemes

Compose an unfold (building a structure) with a fold (consuming it) — and fuse them so no intermediate structure is ever created.

## The Problem This Solves

Many algorithms have two phases:
1. **Expand** a problem into subproblems (build a structure)
2. **Combine** the subproblem results (fold the structure)

Merge sort is the classic example:
- Phase 1 (anamorphism): split the list recursively until you have lists of size 1
- Phase 2 (catamorphism): merge pairs of sorted lists back up

Factorial is another:
- Phase 1: unfold `n` into `n, n-1, n-2, ..., 1` (a countdown list)
- Phase 2: fold by multiplying all elements together

If you implement these as separate steps — `ana` then `cata` — you build a tree or list in memory, then immediately traverse it again to consume it. That intermediate structure exists only to be destroyed.

A **hylomorphism** fuses these two passes into one. The intermediate structure is never built. You get the same result with half the allocations.

Even better: by separating the "split" logic (coalgebra) from the "combine" logic (algebra), each piece is easy to reason about independently. You can prove merge sort correct by:
1. Proving the coalgebra always terminates and splits correctly
2. Proving the algebra merges correctly
3. Knowing `hylo` handles the composition

## The Intuition

Think about how a recursive function like `factorial` actually works:

```
factorial(5)
  = 5 * factorial(4)
         = 4 * factorial(3)
                = 3 * factorial(2)
                       = 2 * factorial(1)
                              = 1
```

On the way *down*, you're building up a call stack — essentially unfolding `5` into `[5, 4, 3, 2, 1]`.
On the way *back up*, you're multiplying — folding those numbers into a product.

The call stack IS the intermediate structure. A hylomorphism makes this explicit:

```rust
// Unfold phase: 5 → 5, 4, 3, 2, 1, stop
let coalg = |n| if n <= 0 { ListF::NilF } else { ListF::ConsF(n, n - 1) };

// Fold phase: multiply everything together
let alg = |l| match l { ListF::NilF => 1, ListF::ConsF(n, acc) => n * acc };

hylo(&alg, &coalg, 5)  // = 120, without building a list in memory
```

`hylo` runs both simultaneously. For each node, it:
1. Applies the coalgebra (expand this seed into a node + child seeds)
2. Recursively `hylo`s each child seed (get child results)
3. Applies the algebra (combine child results into the node result)

The intermediate structure exists only as the *call stack* — never as heap-allocated data.

## How It Works in Rust

**`hylo` — the fused unfold-then-fold:**

```rust
enum ListF<A> { NilF, ConsF(i64, A) }

impl<A> ListF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF        => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(x, f(a)),
        }
    }
}

fn hylo<S, A>(
    alg:   &dyn Fn(ListF<A>) -> A,   // what to do at each node (fold logic)
    coalg: &dyn Fn(S) -> ListF<S>,   // how to expand a seed (unfold logic)
    seed:  S,
) -> A {
    // coalg expands the seed → ListF<S> (a node with child seeds)
    // .map() recursively hylos each child seed → ListF<A> (a node with child results)
    // alg collapses the node with results → A
    alg(coalg(seed).map(|s| hylo(alg, coalg, s)))
}
// Notice: this is identical in structure to cata, but coalg replaces the Fix unwrapping.
// No Fix type needed at all!
```

**Factorial — countdown then multiply:**

```rust
fn factorial(n: i64) -> i64 {
    hylo(
        &|l| match l { ListF::NilF => 1, ListF::ConsF(n, acc) => n * acc },
        &|n| if n <= 0 { ListF::NilF } else { ListF::ConsF(n, n - 1) },
        n,
    )
}

assert_eq!(factorial(5), 120);
assert_eq!(factorial(10), 3628800);
```

**Sum of range — same coalgebra, different algebra:**

```rust
fn sum_range(n: i64) -> i64 {
    hylo(
        &|l| match l { ListF::NilF => 0, ListF::ConsF(x, acc) => x + acc },
        &|n| if n <= 0 { ListF::NilF } else { ListF::ConsF(n, n - 1) },
        n,
    )
}

assert_eq!(sum_range(100), 5050);
```

**Merge sort — split then merge via tree hylo:**

```rust
enum TreeF<A> { LeafF(i64), BranchF(A, A) }

fn hylo_tree<S, A>(
    alg:   &dyn Fn(TreeF<A>) -> A,
    coalg: &dyn Fn(S) -> TreeF<S>,
    seed:  S,
) -> A {
    alg(coalg(seed).map(|s| hylo_tree(alg, coalg, s)))
}

fn merge_sort(xs: Vec<i64>) -> Vec<i64> {
    if xs.is_empty() { return vec![]; }
    hylo_tree(
        // Algebra: merge two sorted lists
        &|t| match t {
            TreeF::LeafF(n)      => vec![n],
            TreeF::BranchF(l, r) => merge(&l, &r),  // merge helper
        },
        // Coalgebra: split a list into two halves
        &|xs: Vec<i64>| {
            if xs.len() <= 1 { TreeF::LeafF(xs[0]) }
            else {
                let mid = xs.len() / 2;
                TreeF::BranchF(xs[..mid].to_vec(), xs[mid..].to_vec())
            }
        },
        xs,
    )
}

assert_eq!(merge_sort(vec![3, 1, 4, 1, 5, 9, 2, 6]), vec![1, 1, 2, 3, 4, 5, 6, 9]);
```

The split tree is never stored in memory — just in the call stack.

## What This Unlocks

- **Fused build-and-consume algorithms.** Merge sort, quicksort, FFT, grammar expansion — any divide-and-conquer algorithm is a hylomorphism. The pattern names the structure, making it easier to reason about.
- **Separate correctness concerns.** Prove the coalgebra terminates and produces valid splits. Prove the algebra correctly combines results. Hylo wires them together correctly by construction.
- **Compilers use this constantly.** Parsing (unfold source → AST) then code generation (fold AST → bytecode) is a hylomorphism. Type checking, optimization passes — the pattern appears everywhere once you recognize it.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hylo definition | `hylo alg coalg seed = alg (fmap (hylo alg coalg) (coalg seed))` | Same — `alg(coalg(seed).map(\|s\| hylo(alg, coalg, s)))` |
| Intermediate structure | None (fused in call stack) | None — same behavior |
| List splitting | `List.take` / `List.drop` | Slice syntax `xs[..mid].to_vec()` |
| Merge implementation | Recursive pattern match | While loop with index variables |
| Empty list guard | Pattern match | Early `return vec![]` guard |
