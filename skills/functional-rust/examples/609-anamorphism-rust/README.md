# 609: Anamorphism (Unfold) Generalized

**Difficulty:** 5  **Level:** Master

Generate recursive structures from a seed value — the exact dual of `cata`, working in reverse.

## The Problem This Solves

Some algorithms run in reverse: instead of consuming a tree, they *produce* one. Building a range, generating Fibonacci numbers, constructing a BST from a sorted list, expanding a grammar — each starts from a simple seed and grows into a full recursive structure.

The code you'd write by hand:

```rust
fn build_range(lo: i64, hi: i64) -> Vec<i64> {
    let mut result = vec![];
    let mut i = lo;
    while i <= hi {
        result.push(i);
        i += 1;
    }
    result
}
```

This works, but it conflates two things: *the expansion rule* (what element to produce and how to advance the state) and *the accumulation machinery* (the loop, the push, the vector). Change the structure from `Vec` to a tree and you rewrite everything.

An anamorphism (also called **unfold**) separates these. You write a **coalgebra** — a function from seed to "one step of the structure" — and the `ana` machinery handles the recursion and accumulation. Swap the output type and only `ana` changes; your coalgebra stays the same.

Rust already ships a specialization of this: `std::iter::from_fn` and `std::iter::successors` are anamorphisms over the `Option`-terminated sequence type. This example generalizes to trees and any other shaped output.

## The Intuition

A coalgebra is just a function that answers: *"given my current state, what's the next step?"*

For Fibonacci numbers up to 100:
- State: `(current, next)` pair
- Step: produce `current`, new state is `(next, current + next)`
- Stop: when `current > 100`

```rust
let fibs = unfold_list((0u64, 1u64), |(a, b)| {
    if a > 100 { None }             // stop here
    else { Some((a, (b, a + b))) }  // produce a, next state is (b, a+b)
});
```

Compare to `std::iter::successors` — same concept:

```rust
let fibs: Vec<u64> = std::iter::successors(Some((0u64, 1u64)), |&(a, b)| {
    if b > 100 { None } else { Some((b, a + b)) }
}).map(|(a, _)| a).collect();
```

The key difference: `unfold_list` works for any output structure, not just iterators. The same coalgebra pattern works for trees:

```rust
// Build a BST from a sorted range
let bst = unfold_tree(1..=7, |range| {
    let v: Vec<i32> = range.collect();
    if v.is_empty() { None }
    else {
        let mid = v.len() / 2;
        Some((v[mid], v[..mid].to_vec(), v[mid+1..].to_vec()))
        //   ^value    ^left subtree      ^right subtree
    }
});
```

## How It Works in Rust

**List unfold:**

```rust
// unfold_list: seed S, coalgebra S → Option<(A, S)>
// Returns None → stop. Returns Some((value, next_seed)) → produce value, continue.
fn unfold_list<S: Clone, A>(
    seed:  S,
    coalg: impl Fn(S) -> Option<(A, S)>,
) -> Vec<A> {
    let mut result = Vec::new();
    let mut state  = seed;
    while let Some((item, next)) = coalg(state.clone()) {
        result.push(item);
        state = next;
    }
    result
}

// Range [1, 5]
let range = unfold_list(1, |i| {
    if i > 5 { None } else { Some((i, i + 1)) }
});
assert_eq!(range, vec![1, 2, 3, 4, 5]);

// Fibonacci numbers up to 100
let fibs = unfold_list((0u64, 1u64), |(a, b)| {
    if a > 100 { None } else { Some((a, (b, a + b))) }
});
// → [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]

// Digits of a number (least significant first)
let digits = unfold_list(1234u32, |n| {
    if n == 0 { None } else { Some((n % 10, n / 10)) }
});
// → [4, 3, 2, 1]
```

**Tree unfold — same idea, two children instead of one:**

```rust
#[derive(Debug)]
enum Tree<A> {
    Leaf,
    Node { val: A, left: Box<Tree<A>>, right: Box<Tree<A>> },
}

// coalg: seed → None (leaf) or Some((value, left_seed, right_seed))
fn unfold_tree<S: Clone, A>(
    seed:  S,
    coalg: impl Fn(S) -> Option<(A, S, S)> + Copy,
) -> Tree<A> {
    match coalg(seed) {
        None => Tree::Leaf,
        Some((val, l_seed, r_seed)) => Tree::Node {
            val,
            left:  Box::new(unfold_tree(l_seed, coalg)),
            right: Box::new(unfold_tree(r_seed, coalg)),
        }
    }
}

// BST from a sorted range
let bst: Tree<i32> = unfold_tree(vec![1, 2, 3, 4, 5, 6, 7], |v| {
    if v.is_empty() { return None; }
    let mid = v.len() / 2;
    Some((v[mid], v[..mid].to_vec(), v[mid+1..].to_vec()))
});
```

**Iterator as anamorphism — Rust's built-in version:**

```rust
// std::iter::successors IS an anamorphism
let powers_of_2: Vec<u64> = std::iter::successors(Some(1u64), |&n| {
    n.checked_mul(2)  // None = stop (overflow)
}).take(10).collect();
// → [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
```

## What This Unlocks

- **Generate any recursive structure with a simple rule.** Ranges, BSTs, Fibonacci trees, Collatz sequences, grammar derivation trees — write the rule once as a coalgebra; `ana` handles the structure.
- **Pair naturally with `cata`.** `ana` builds; `cata` consumes. Stack them together (fusing into `hylo`) to express algorithms as explicit build-then-consume pipelines.
- **Infinite structures safely.** Coalgebras that never return `None` produce infinite sequences. In Rust this requires lazy evaluation (iterators) or a depth limit, but the pattern maps directly to Haskell's corecursive coinductive types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Anamorphism | Dual of fold; generates potentially infinite structures | Same concept; `unfold_list` / `unfold_tree` |
| Coalgebra type | `'seed -> 'seed list_f` | `impl Fn(S) -> Option<(A, S)>` |
| Rust standard library | No direct equivalent | `std::iter::from_fn`, `std::iter::successors` are list anamorphisms |
| Termination | Not guaranteed — coinductive types are valid | Must terminate (finite `Vec`) or use `Iterator` (lazy) |
| Infinite streams | Coinductive, lazy by default | Use `Iterator` — lazy and safe |
