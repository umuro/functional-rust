📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1073-burst-balloons)**

---

# 1073-burst-balloons — Burst Balloons
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The burst balloons problem is an interval DP gem: given a row of balloons with numbers, bursting balloon `k` earns `nums[k-1] * nums[k] * nums[k+1]` coins. Find the maximum coins from bursting all balloons in the best order. The key insight is reasoning about the LAST balloon to burst in an interval, not the first.

This "think backwards" trick converts an exponential search into a polynomial DP, and the same pattern appears in matrix chain multiplication, optimal binary search trees, and stone merging problems.

## Learning Outcomes

- Understand interval DP and the "last element" thinking strategy
- Implement burst balloons with `dp[i][j]` = max coins for interval `(i, j)` exclusive
- Add sentinel values (1, ...nums..., 1) to handle edge balloons cleanly
- Compare top-down memoization to bottom-up interval DP
- Apply the "think backwards" strategy to other interval DP problems

## Rust Application

`src/lib.rs` adds sentinel 1s at both ends of the array (`balloons[0] = balloons[n+1] = 1`). `dp[i][j]` = max coins from bursting all balloons strictly between positions `i` and `j`. The recurrence: for each `k` in `(i, j)`, treat `k` as the last balloon to burst: `dp[i][k] + dp[k][j] + balloons[i] * balloons[k] * balloons[j]`. Fill by increasing interval length.

The "last balloon" insight: when balloon `k` is last in interval `(i, j)`, its neighbors at burst time are `balloons[i]` and `balloons[j]` (the sentinels or already-burst boundaries), making the coin calculation clean.

## OCaml Approach

```ocaml
let max_coins nums =
  let n = Array.length nums in
  let b = Array.make (n+2) 1 in
  Array.blit nums 0 b 1 n;
  let len = n + 2 in
  let dp = Array.make_matrix len len 0 in
  for gap = 2 to len - 1 do
    for i = 0 to len - gap - 1 do
      let j = i + gap in
      for k = i + 1 to j - 1 do
        let coins = dp.(i).(k) + dp.(k).(j) + b.(i) * b.(k) * b.(j) in
        if coins > dp.(i).(j) then dp.(i).(j) <- coins
      done
    done
  done;
  dp.(0).(len-1)
```

Structurally identical. The sentinel-insertion and interval-filling logic is purely mathematical.

## Key Differences

1. **Sentinel insertion**: Both prepend and append 1 to handle boundary conditions; Rust uses `vec![1; n+2]` with array blit, OCaml uses `Array.make`.
2. **Interval filling order**: Both fill by increasing gap size — a fundamental constraint of interval DP ensuring sub-intervals are computed before larger ones.
3. **`usize::MAX` risk**: Rust must avoid `usize::MAX` initialization for max-problems; using 0 and taking max works here since coins are non-negative.
4. **"Think backwards" applicability**: Same pattern applies to matrix chain (1057), stone merge, and optimal BST — recognizing the pattern is the key skill.

## Exercises

1. Reconstruct the optimal burst order using a separate `order` table that records the chosen `k` at each `dp[i][j]`.
2. Solve the stone merging problem using the same interval DP pattern: merge adjacent stones, cost = sum of merged stones.
3. Implement the "stone game" variant where two players alternate bursting balloons and both want to maximize their own coins.
