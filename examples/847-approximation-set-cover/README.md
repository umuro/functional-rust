📖 **[View on hightechmind.io →](https://hightechmind.io/rust/847-approximation-set-cover)**

---

# Approximation Algorithm — Set Cover

## Problem Statement

Set cover is NP-hard: given a universe U of elements and a collection S of subsets, find the minimum number of subsets whose union equals U. The greedy approximation picks the subset covering the most uncovered elements at each step, achieving an O(log n) approximation — proven optimal assuming P ≠ NP. This approximation is used in: network monitoring (minimum sensor placement to cover all traffic), advertising (minimum campaigns to reach all demographics), feature selection in machine learning, and compiler register allocation. Understanding approximation algorithms — which sacrifice optimality for polynomial time — is essential for NP-hard optimization.

## Learning Outcomes

- Implement the greedy set cover approximation: at each step, pick the set covering most uncovered elements
- Understand the O(ln n) approximation ratio: greedy covers at least 1/OPT fraction at each step
- Recognize when approximation is appropriate: NP-hard problems in practice
- Compare greedy approximation ratio against brute-force optimal on small instances
- Apply the technique to vertex cover, dominating set, and other NP-hard problems with similar guarantees

## Rust Application

```rust
pub fn greedy_set_cover(universe: &HashSet<u32>, sets: &[HashSet<u32>]) -> Vec<usize> {
    let mut uncovered = universe.clone();
    let mut chosen = vec![];
    while !uncovered.is_empty() {
        // Find set covering most uncovered elements
        let best = sets.iter().enumerate()
            .max_by_key(|(_, s)| s.intersection(&uncovered).count())
            .map(|(i, _)| i);
        if let Some(idx) = best {
            uncovered.retain(|e| !sets[idx].contains(e));
            chosen.push(idx);
        } else { break; } // no progress possible
    }
    chosen
}
```

The `intersection(&uncovered).count()` computes the coverage score for each set. `max_by_key` selects the best set in O(n * |S|) per iteration. `uncovered.retain` removes covered elements efficiently. Rust's `HashSet::intersection` returns an iterator, making the count lazy and allocation-free. The `break` handles the case where universe cannot be fully covered. The result is the list of chosen set indices in selection order.

## OCaml Approach

OCaml implements greedy set cover with a `Hashtbl` for the uncovered set or a `Set.Make(Int)` balanced BST. `List.fold_left` finds the maximum-coverage set. `Set.inter uncovered s |> Set.cardinal` counts intersections. OCaml's `Set.diff uncovered chosen_set` removes covered elements immutably. The `while` loop or tail recursion drives iteration. `List.rev` restores selection order if building the list by prepending.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Universe set | `HashSet<u32>` | `Set.Make(Int).t` or `Hashtbl` |
| Intersection count | `.intersection().count()` | `Set.inter \|> Set.cardinal` |
| Best set selection | `max_by_key` | `List.fold_left` with max |
| Uncovered removal | `retain(|e| !contains)` | `Set.diff uncovered chosen` |
| Approximation ratio | O(ln n) provable | Same |
| Brute force comparison | Exponential search | Same |

## Exercises

1. Implement brute-force optimal set cover (exponential) for small instances and compare with greedy approximation ratio.
2. Implement the LP relaxation of set cover and show its integrality gap matches the greedy bound.
3. Apply greedy set cover to a sensor placement problem: cover all n points in a plane with minimum unit-radius circles.
4. Implement a weighted set cover: each subset has a cost, minimize total cost using the cost-effectiveness greedy.
5. Measure approximation quality empirically: for random instances, how close does greedy come to optimal?
