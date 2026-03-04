# 220: Paramorphism

**Difficulty:** ⭐⭐⭐  **Level:** Recursion Schemes

Catamorphism upgraded: fold a recursive structure while keeping the original subterms in view at each step.

## The Problem This Solves

A catamorphism (plain fold) gives you only the *computed result* at each recursive position — you can see that the tail of `[1,2,3]` folded to `6`, but you've lost the actual list `[2,3]`. Sometimes that's not enough. Computing `tails` of a list requires you to see the original tail alongside its fold result. Computing a sliding window requires the same. A plain `fold` loses the structure too early.

Paramorphism solves this by giving the algebra both pieces of information at once: the *computed result* from the recursive call **and** the *original subtree* it came from. You can think of it as "fold with a memory" — at each step you get `(result_so_far, original_substructure)`.

The name comes from Greek *para* (beside) — the original subterm sits *beside* the recursive result, always available.

## The Intuition

Imagine you're folding a list from right to left. With a plain fold, when you're at element `x`, you only see what the rest computed to. With a paramorphism, when you're at element `x`, you see *two* things: (1) what the rest computed to, and (2) the actual list that came after `x`.

Real example: computing all tails of `[1, 2, 3]` gives `[[1,2,3], [2,3], [3], []]`. To produce the sublist `[2,3]` when processing element `1`, you need access to the original remainder `[2,3]` — not just whatever it computed to. A fold would have thrown that structure away.

Factorial with access to `n-1` is the canonical numeric example: `n! = n * (n-1)!` where you need both `(n-1)!` (the fold result) and `n-1` (the original subterm, as a `FixNat`).

**Para algebra type:** `ListF<(A, FixList)> -> A` — every child becomes a pair of `(result, original)`.

## How It Works in Rust

```rust
// para: algebra gets (result, original_subtree) for each child
fn para<A: Clone>(alg: &dyn Fn(ListF<(A, FixList)>) -> A, fl: &FixList) -> A {
    // For every child node, recursively compute (result, clone of original)
    let paired = fl.0.map_ref(|child| (para(alg, child), child.clone()));
    alg(paired)
}
```

Step by step:
1. For each child in the current layer, recurse to get result `A`
2. Clone the original child `FixList` to keep it alongside
3. Pass `(A, FixList)` pairs to the algebra — algebra sees both
4. Algebra decides what to do: use the result, the original, or both

Computing `tails` with paramorphism:
```rust
fn tails(fl: &FixList) -> Vec<Vec<i64>> {
    para(&|l: ListF<(Vec<Vec<i64>>, FixList)>| match l {
        ListF::NilF => vec![vec![]],                          // base: just empty tail
        ListF::ConsF(_, (rest_tails, original_tail)) => {
            // original_tail is the actual FixList after this element
            let mut v = vec![to_vec(&original_tail)];        // this suffix as a vec
            v.extend(rest_tails);                             // all smaller suffixes
            v
        }
    }, fl)
}
```

The key line is `to_vec(&original_tail)` — this is only possible because paramorphism preserved `original_tail` for us.

## What This Unlocks

- **Tails / inits / sliding windows** — any operation that needs sublist views during the fold, not just the accumulated result.
- **Factorial and similar** — numeric recursions where you need `n-1` as a value, not just `(n-1)!` as a result.
- **Tree diffing** — compare subtrees structurally during a fold, using the original fragments for context-sensitive decisions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Algebra signature | `ListF ('a * fix) -> 'a` | `ListF<(A, FixList)> -> A` |
| Keeping the original | Shared reference (GC) | `.clone()` required |
| `tails` implementation | Natural, no overhead | Same logic, clone cost per node |
| vs catamorphism | Only result visible | Result **and** original both visible |
| Performance | GC handles sharing | Clone copies subtrees — trade-off |
