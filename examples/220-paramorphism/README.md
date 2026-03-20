📖 **[View on hightechmind.io →](https://hightechmind.io/rust/220-paramorphism)**

---

# Paramorphism — Cata with Access to Original Subtree
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A catamorphism's algebra sees only the folded results of children, not the original sub-structure. But some algorithms need both — the original subtree and its folded result simultaneously. Factorial needs "the predecessor" and its result; a tree algorithm may need the original child count and the accumulated value. Paramorphisms extend catamorphisms: the algebra receives pairs of `(original_subtree, accumulated_result)` for each child.

## Learning Outcomes

- Understand paramorphisms as catamorphisms with access to original sub-structure
- Learn the `para` function: algebra receives `F<(Fix<F>, A)>` instead of `F<A>`
- See factorial as the canonical example: uses both the number and its factorial
- Understand when `para` is needed vs. `cata`: when the original structure matters, not just the result

## Rust Application

`para<A>(alg: impl Fn(ListF<(FixList, A)>) -> A) -> impl Fn(FixList) -> A`. The algebra for `(Fix(ConsF(x, (tail, tail_result))))` receives both `tail` (the original remaining list) and `tail_result` (the accumulated fold result). The "sliding window" example: `para` over a list of `i64` computes each element's ratio to the previous — needing both the previous element's value and the accumulated results. Factorial: `alg(ConsF(n, (n_pred_node, factorial_n_pred))) = n * factorial_n_pred`.

## OCaml Approach

OCaml's paramorphism:
```ocaml
let rec para alg (Fix lf) =
  alg (map_list_f (fun child -> (child, para alg child)) lf)
```
Each child is paired with its recursed result. OCaml's `let rec` makes the mutual structure clear. The list tail in a `para` computation is available as the original `FixList` value, not just the folded integer result.

## Key Differences

1. **Original access**: `para` preserves the original structure at each step; `cata` discards it after folding — this is the essential difference.
2. **Expressive power**: Any `cata` can be expressed as a `para` (ignore the original structure parameter); `para` is strictly more expressive.
3. **Use cases**: Paramorphisms are needed for algorithms that compare adjacent elements, access previous values, or need the structural context.
4. **Tuple overhead**: The `(Fix<F>, A)` pairs require allocating tuples at each node; this is the cost of the extra information.

## Exercises

1. Implement `list_suffixes` using `para`: for `[1, 2, 3]` return `[[1, 2, 3], [2, 3], [3], []]` — each suffix is the original tail.
2. Write a `sliding_sum(n)` that sums every window of `n` consecutive elements using `para`.
3. Implement `indexed_fold` that pairs each element with its index using `para`.
