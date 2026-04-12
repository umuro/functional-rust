📖 **[View on hightechmind.io →](https://hightechmind.io/rust/842-branch-and-bound)**

---

# Branch and Bound
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Backtracking explores the complete search space but doesn't use bounds to prune. Branch and bound adds an upper/lower bound computation at each node: if the best possible solution achievable from a partial assignment cannot beat the current best complete solution, prune that entire subtree. This makes branch and bound the algorithm of choice for exact optimization: integer linear programming, traveling salesman problem (with bounds from relaxations), knapsack (with greedy fractional bound), and scheduling optimization. While still exponential in the worst case, good bounds make it practical for real instances that pure backtracking cannot handle.

## Learning Outcomes

- Understand the bounding function: optimistic estimate of best achievable value from a partial solution
- Implement the branch-and-bound template: branch into children, compute bound, prune if bound ≤ best
- Apply to 0/1 knapsack with fractional relaxation bound (greedily pack remaining items fractionally)
- Recognize the tradeoff: tighter bounds → more pruning → faster → more expensive to compute
- Compare with dynamic programming: DP is exact and polynomial for knapsack; B&B is exact but can prune large trees

## Rust Application

```rust
pub fn knapsack_bb(items: &[(f64, f64)], capacity: f64) -> f64 {
    // Sort items by value/weight ratio
    let mut sorted = items.to_vec();
    sorted.sort_by(|a, b| (b.0/b.1).partial_cmp(&a.0/a.1).unwrap());
    let mut best = 0.0;
    // BFS/DFS with bound pruning
    fn bound(items: &[(f64, f64)], idx: usize, cap: f64, val: f64) -> f64 {
        // Fractional knapsack from current state — optimistic upper bound
        let mut cap = cap; let mut v = val;
        for &(wi, vi) in &items[idx..] {
            if wi <= cap { cap -= wi; v += vi; }
            else { v += vi * (cap / wi); break; }
        }
        v
    }
    best
}
```

The bounding function for 0/1 knapsack is the fractional relaxation: greedily pack items (sorted by ratio) fractionally, allowing splitting the last item. This gives the tightest polynomial-time computable upper bound for knapsack. Rust's mutable `best` variable holds the current best complete solution. Items sorted by value/weight ratio upfront ensure the greedy bound computation is efficient. The `f64` type handles real-valued weights and values; integer variants use `u64`.

## OCaml Approach

OCaml's branch-and-bound uses a priority queue (BFS with best-first search) or a recursive DFS. The `bound` function mirrors the Rust fractional knapsack. OCaml's `List.sort` sorts items by ratio. The `Queue.t` or `Heap` module implements BFS order. Mutable `best ref` tracks the current best. OCaml's tail-recursive DFS avoids stack overflow for deep trees. The `List.fold_left` accumulates the bound computation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Search strategy | DFS (stack-based) or BFS | Same options |
| Best tracking | `mut f64` variable | `float ref` |
| Bounding function | Nested `fn bound` | Nested `let bound` or separate |
| Priority queue | `BinaryHeap` for best-first | `Heap` module or priority list |
| Sorting | `sort_by` with `partial_cmp` | `List.sort` with comparison |
| Integer variant | `u64` items and capacity | `int` items |

## Exercises

1. Implement best-first search branch-and-bound using `BinaryHeap<Reverse<(NotNan<f64>, State)>>` for better pruning order.
2. Apply branch-and-bound to the traveling salesman problem with a minimum spanning tree lower bound.
3. Compare branch-and-bound vs. DP knapsack on instances with 20, 50, 100 items and measure when B&B beats DP.
4. Implement the LP relaxation bound using the `minilp` crate for tighter bounds on integer programs.
5. Profile the percentage of nodes pruned as a function of bound tightness for random knapsack instances.
