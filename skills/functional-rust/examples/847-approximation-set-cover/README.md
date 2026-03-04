# 847: Approximation Algorithms — Greedy Set Cover

**Difficulty:** 4  **Level:** Advanced

Solve an NP-hard problem within a provable factor of optimal using a greedy strategy — the foundation of approximation algorithm design.

## The Problem This Solves

Set cover is NP-hard: given a universe U and a collection of sets, find the smallest sub-collection that covers every element. Optimal solutions require exponential search. But many real-world instances are huge — covering all customers with minimum sales territories, covering all network nodes with minimum monitoring points, covering all genes with minimum probes.

The greedy algorithm gives a practical escape: always pick the set that covers the most currently uncovered elements. This runs in polynomial time and achieves a ln(|U|)+1 approximation ratio — meaning it uses at most ln(|U|) times as many sets as the optimal solution. Crucially, this is essentially optimal: no polynomial algorithm can do better (unless P=NP).

The weighted variant extends naturally: instead of maximizing elements covered per set, minimize cost per newly covered element. This is greedy in a different direction but carries the same approximation guarantee.

## The Intuition

Greedy is locally optimal at each step. The insight is that "locally optimal" compounds nicely: at each round, the uncovered universe shrinks by at least a factor of (1 - 1/OPT). After k rounds the uncovered fraction is at most (1 - 1/OPT)^k ≤ e^(-k/OPT). Setting this below 1 gives k < OPT·ln(|U|). This harmonic series argument is the core of the approximation proof.

For the weighted case, the greedy rule "minimum cost per element covered" is the fractional relaxation of the LP, which gives the same ln approximation.

## How It Works in Rust

1. **`greedy_set_cover`** — maintains `uncovered: HashSet<usize>`. Each round uses `.max_by_key(|(_, s)| s.intersection(&uncovered).count())` to find the best set.
2. **Mark and remove** — add chosen set's index to `chosen`, mark it `used`, remove its elements from `uncovered`.
3. **Terminate** when `uncovered.is_empty()` — guaranteed if the input sets cover the universe.
4. **Weighted variant** — `filter_map` computes `cost / new_covered` for each unused set, then `.min_by` picks the cheapest per-element option.
5. **`verify_cover`** — confirms correctness: union of chosen sets must be a superset of the universe.

```rust
let best = sets.iter()
    .enumerate()
    .filter(|(i, _)| !used[*i])
    .max_by_key(|(_, s)| s.intersection(&uncovered).count());
```

## What This Unlocks

- **Provable approximation** — ln(|U|)+1 ratio is a mathematical guarantee, not empirical hope.
- **Weighted generalization** — the same framework handles heterogeneous set costs with minimal code change.
- **Blueprint for approximation design** — greedy + local-optimality argument works for vertex cover, facility location, and scheduling variants.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Set operations | `Set.S` functor | `HashSet<T>` |
| Intersection count | `Set.cardinal (Set.inter a b)` | `a.intersection(&b).count()` |
| Greedy loop | Recursive with list fold | `while !uncovered.is_empty()` |
| Max by key | `List.fold_left` comparison | `.max_by_key(...)` iterator adapter |
| Approximation ratio | ln(n)+1 — math identical | Same guarantee, same algorithm |
