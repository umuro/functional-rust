📖 **[View on hightechmind.io →](https://hightechmind.io/rust/844-greedy-algorithm-patterns)**

---

# Greedy Algorithm Patterns

## Problem Statement

Greedy algorithms make locally optimal choices at each step, hoping for a global optimum. Unlike dynamic programming which considers all subproblems, greedy algorithms commit to a choice and never reconsider. When a greedy choice property holds — meaning there always exists an optimal solution that includes the greedy choice — greedy algorithms are correct and typically O(n log n) (dominated by sorting). Classic greedy problems: interval scheduling (schedule maximum activities), fractional knapsack, Huffman coding, Prim's/Kruskal's MST, and coin change (for standard denominations). Recognizing when greedy works (and when it fails) is a key algorithm design skill.

## Learning Outcomes

- Identify the greedy choice property: locally optimal → globally optimal
- Implement interval scheduling maximization: sort by end time, greedily pick non-overlapping intervals
- Implement fractional knapsack: sort by value/weight ratio, greedily fill
- Understand exchange argument proofs: show swapping greedy choice for any other cannot improve
- Recognize when greedy fails: 0/1 knapsack, coin change with non-standard denominations

## Rust Application

```rust
pub fn interval_scheduling(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // Sort by end time — the greedy choice
    intervals.sort_by_key(|&(_, end)| end);
    let mut result = vec![];
    let mut last_end = i32::MIN;
    for interval in intervals {
        if interval.0 >= last_end {
            last_end = interval.1;
            result.push(interval);
        }
    }
    result
}
```

Sorting by end time is the greedy choice for interval scheduling — the exchange argument proves that replacing any selected interval with the earliest-ending compatible interval cannot decrease the count. `sort_by_key` with a key function is idiomatic Rust. The `last_end` variable tracks the latest end time of selected intervals. `i32::MIN` initializes it to allow any first interval. The result collects selected intervals; returning just the count is also valid for pure optimization queries.

## OCaml Approach

OCaml's interval scheduling uses `List.sort (fun (_, e1) (_, e2) -> compare e1 e2)` then `List.fold_left` accumulating selected intervals. The greedy decision is `if start >= last_end`. OCaml's `Int.min_int` initializes `last_end`. The fractional knapsack sorts by ratio using `List.sort` and fills greedily with `List.fold_left`. OCaml's pattern destructuring `let (start, end_) = interval` reads naturally. The `Printf` module formats activity schedules for verification.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Sort key | `sort_by_key(|&(_, end)| end)` | `List.sort` with comparison fn |
| Initialization | `i32::MIN` | `Int.min_int` |
| Accumulation | `mut last_end` + push | `fold_left` with acc tuple |
| Fractional knapsack | Sort by `v/w` with `f64` | Same with `float` |
| Correctness proof | Exchange argument | Same mathematical argument |
| Failure cases | 0/1 knapsack example | Same |

## Exercises

1. Implement a greedy coin change algorithm and show a denomination set where it fails to find the optimal solution.
2. Implement Huffman coding: build a frequency tree greedily using a min-heap, then assign binary codes.
3. Verify the interval scheduling result: write a checker that confirms no two selected intervals overlap.
4. Implement the activity selection with weighted jobs (maximize total weight, not count) and show greedy fails — requires DP.
5. Implement the job scheduling to minimize lateness: sort by deadline, prove exchange argument for correctness.
