📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1074-egg-drop)**

---

# 1074-egg-drop — Egg Drop
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Given k eggs and n floors, find the minimum number of trials needed to determine the critical floor above which eggs break. This is a classic DP/binary-search problem that appears in reliability testing, threshold determination, and A/B test stopping rules.

The naive O(kn²) DP can be improved to O(kn log n) using binary search, and further to O(kn) with a rolling-minimum approach.

## Learning Outcomes

- Model the egg drop problem as a DP with state (eggs, floors)
- Implement the O(kn²) DP and understand the recurrence
- Optimize to O(kn log n) with binary search on the drop floor
- Understand the "worst case" minimax nature: minimize(maximum(break, survive))
- Connect to binary search theory and reliability testing

## Rust Application

`src/lib.rs` implements `egg_drop_basic` with `dp[i][j]` = minimum trials with `i` eggs and `j` floors. For one egg, you must try every floor linearly (`dp[1][j] = j`). For more eggs, try all drop floors and take the worst case. `egg_drop_bs` uses binary search to find the optimal drop floor: the transition point where `dp[i-1][x-1] < dp[i][j-x]` changes, enabling O(log n) inner loop. A third approach using the "trials to floors" formulation converts the problem to: with `k` eggs and `t` trials, how many floors can we check?

## OCaml Approach

```ocaml
let egg_drop eggs floors =
  let dp = Array.make_matrix (eggs+1) (floors+1) 0 in
  for i = 1 to eggs do
    for j = 1 to floors do
      if i = 1 then dp.(i).(j) <- j
      else begin
        dp.(i).(j) <- max_int;
        for x = 1 to j do
          let worst = 1 + max dp.(i-1).(x-1) dp.(i).(j-x) in
          if worst < dp.(i).(j) then dp.(i).(j) <- worst
        done
      end
    done
  done;
  dp.(eggs).(floors)
```

Identical structure. The binary search optimization has the same mathematical basis in both languages.

## Key Differences

1. **`max_int` / `usize::MAX`**: Both use a large sentinel as infinity for minimization; Rust's `usize::MAX` saturates on addition — use explicit comparison to avoid overflow.
2. **Binary search optimization**: The transition point derivation is purely mathematical; both languages implement `lo`/`hi` bisection identically.
3. **Alternative formulation**: The "k eggs, t trials → how many floors" DP (`dp[t][k] += dp[t-1][k-1] + dp[t-1][k]`) is cleaner — only requires O(kn) instead of O(kn²) DP.
4. **Practical applications**: Reliability testing (how many tests to find a failure threshold), canary deployments (how many % rollouts to find a regression floor).

## Exercises

1. Implement the `dp[t][k]` formulation: minimum trials `t` such that `dp[t][k] >= n`, which avoids the O(n²) inner loop.
2. Add path reconstruction: return not just the minimum trials but also the sequence of floors to try.
3. Generalize to "weighted egg drop" where breaking an egg on floor `f` costs `cost[f]` — find the minimum expected cost.
