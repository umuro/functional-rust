**Difficulty:** ⭐  
**Category:** Functional Programming  

[sum-of-multiples on hightechmind.io](https://hightechmind.io/posts/functional-rust/sum-of-multiples)

---

## Problem Statement

Find the sum of all unique multiples of any factor in a given set, below a limit. For example, the sum of all multiples of 3 or 5 below 1000 is the classic Project Euler problem 1. Implement three approaches: iterator-based deduplication via `HashSet`, a fold-based functional accumulation, and an O(1) mathematical formula using inclusion-exclusion with GCD/LCM.

## Learning Outcomes

- Generate multiples using `(f..limit).step_by(f)` for each factor
- Deduplicate across factors with `HashSet<u64>` — collect all multiples then sum
- Express the same logic as a `fold` that inserts into a `HashSet` incrementally
- Implement the closed-form formula: `sum_divisible(k, limit) = k * n*(n+1)/2` where `n = (limit-1)/k`
- Apply inclusion-exclusion via GCD/LCM to avoid double-counting in the math version

## Rust Application

```rust
pub fn sum_of_multiples(factors: &[u64], limit: u64) -> u64 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}

// Fold-based variant
pub fn sum_of_multiples_fold(factors: &[u64], limit: u64) -> u64 {
    let set: HashSet<u64> = factors
        .iter()
        .filter(|&&f| f != 0)
        .fold(HashSet::new(), |mut acc, &f| {
            (f..limit).step_by(f as usize).for_each(|m| { acc.insert(m); });
            acc
        });
    set.into_iter().sum()
}

// O(1) per factor using arithmetic series formula
fn sum_divisible(k: u64, limit: u64) -> u64 {
    if k == 0 { return 0; }
    let n = (limit - 1) / k;
    k * n * (n + 1) / 2
}
```

The `flat_map` version generates all multiples lazily then deduplicates with a `HashSet`. Collecting into a `HashSet` before summing handles the case where multiple factors share multiples (e.g., 15 is a multiple of both 3 and 5).

The closed-form approach: `sum_divisible(k, n)` computes `k + 2k + ... + floor((n-1)/k)*k = k * T(floor((n-1)/k))` where `T(m)` is the `m`-th triangular number. Inclusion-exclusion then adds/subtracts pairwise LCM terms.

## OCaml Approach

```ocaml
module IntSet = Set.Make(Int)

let multiples_below f limit =
  let rec go acc m =
    if m >= limit then acc
    else go (IntSet.add m acc) (m + f)
  in
  if f = 0 then IntSet.empty else go IntSet.empty f

let sum_of_multiples factors limit =
  let all_multiples =
    List.fold_left
      (fun acc f -> IntSet.union acc (multiples_below f limit))
      IntSet.empty factors
  in
  IntSet.fold ( + ) all_multiples 0
```

OCaml's `Set.Make(Int)` provides an ordered set backed by a balanced BST. `IntSet.union` combines the multiples from each factor, naturally deduplicating. The `fold` over `IntSet` sums the elements.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Deduplication | `HashSet` — O(1) insert/lookup | `Set.Make(Int)` — O(log n) insert |
| Range generation | `(f..limit).step_by(f)` — iterator | Recursive `go` function |
| Union | Implicit via `HashSet` collect | `IntSet.union` |
| Math formula | Feasible with explicit GCD/LCM | Same algorithm |

`step_by` is the idiomatic way to generate arithmetic sequences in Rust. The HashSet approach is simpler for moderate inputs; the mathematical formula is O(factors²) and works for arbitrarily large limits.

## Exercises

1. Implement the inclusion-exclusion formula for exactly two factors.
2. Generalize inclusion-exclusion for `n` factors using the `2^n` subsets approach.
3. Compute the sum of multiples of all primes below 20, under the limit 1,000,000, and compare the `HashSet` and formula approaches for speed.
4. Modify the function to accept a list of ranges `(start, step)` rather than single factors.
5. Write a property test verifying that all three implementations (iterator, fold, math) agree for random inputs.
