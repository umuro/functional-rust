# OCaml vs Rust: Sum of Multiples

## Side-by-Side Code

### OCaml

```ocaml
module IS = Set.Make(Int)

let sum_of_multiples factors limit =
  List.fold_left (fun s factor ->
    if factor = 0 then s
    else
      let multiples = List.init ((limit - 1) / factor) (fun i -> factor * (i + 1)) in
      List.fold_left (fun s m -> IS.add m s) s multiples
  ) IS.empty factors
  |> IS.fold (+) |> fun f -> f 0
```

### Rust (idiomatic)

```rust
use std::collections::HashSet;

pub fn sum_of_multiples(factors: &[u64], limit: u64) -> u64 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}
```

### Rust (functional/fold — mirrors OCaml structure)

```rust
pub fn sum_of_multiples_fold(factors: &[u64], limit: u64) -> u64 {
    let set: HashSet<u64> =
        factors
            .iter()
            .filter(|&&f| f != 0)
            .fold(HashSet::new(), |mut acc, &f| {
                (f..limit).step_by(f as usize).for_each(|m| {
                    acc.insert(m);
                });
                acc
            });
    set.into_iter().sum()
}
```

### Rust (mathematical — inclusion-exclusion, no set needed)

```rust
pub fn sum_of_multiples_math(factors: &[u64], limit: u64) -> u64 {
    fn sum_divisible(k: u64, limit: u64) -> u64 {
        let n = (limit - 1) / k;
        k * n * (n + 1) / 2
    }
    // ... inclusion-exclusion over all 2^k non-empty subsets via LCM
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val sum_of_multiples : int list -> int -> int` | `fn sum_of_multiples(factors: &[u64], limit: u64) -> u64` |
| Factor list | `int list` | `&[u64]` (borrowed slice) |
| Set type | `IS.t` (balanced BST via `Set.Make(Int)`) | `HashSet<u64>` (hash table) |
| Empty set | `IS.empty` | `HashSet::new()` |
| Insert | `IS.add m s` | `acc.insert(m)` |
| Fold / reduce | `IS.fold (+)` | `.into_iter().sum()` |
| Sequence generation | `List.init n (fun i -> f*(i+1))` | `(f..limit).step_by(f)` |

## Key Insights

1. **Set deduplication is the core idea.** Both OCaml and Rust use a set to ensure each multiple is counted only once, regardless of how many factors produce it. This is the direct translation: `Set.Make(Int)` → `HashSet<u64>`.

2. **Lazy vs eager sequences.** OCaml's `List.init` eagerly allocates a list of multiples. Rust's range + `step_by` is a lazy iterator — it produces values on demand, with no intermediate allocation until `collect`.

3. **Mutable accumulator in fold.** OCaml's functional sets are immutable: each `IS.add` returns a new set. The Rust fold variant uses `|mut acc, ...| { acc.insert(...); acc }` — the `mut acc` binding allows in-place mutation of the `HashSet` while still following the fold signature, blending functional form with imperative efficiency.

4. **`step_by` replaces `List.init` arithmetic.** The OCaml expression `List.init ((limit-1)/factor) (fun i -> factor * (i+1))` computes multiples manually. In Rust, `(factor..limit).step_by(factor)` expresses the same arithmetic progression directly using the iterator's built-in stepping mechanism.

5. **Mathematical alternative avoids O(limit) space.** The inclusion-exclusion solution uses `sum(k below N) = k * n*(n+1)/2` over LCM of every subset — O(2^k) time, O(k) space. This showcases how understanding the mathematical structure can eliminate data structures entirely, a pattern common in competitive programming.

## When to Use Each Style

**Use idiomatic Rust (flat_map + HashSet) when:** you want clarity and correctness with moderate-sized inputs; the code reads naturally as "collect all multiples, deduplicate, sum".

**Use fold-based Rust when:** you want to mirror the OCaml structure closely for pedagogical comparison, or when you need to interleave set construction with other per-factor logic.

**Use mathematical inclusion-exclusion when:** the limit is very large (billions) making O(limit) space impractical, or when factors are few (k ≤ 20) and you need O(1) space with a fast closed-form answer.
