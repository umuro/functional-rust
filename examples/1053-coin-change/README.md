📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1053-coin-change)**

---

# 1053: Coin Change

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Find minimum number of coins to make a given amount — classic unbounded knapsack variant
**Key Insight:** The DP recurrence `dp[i] = min(dp[i], dp[i-coin] + 1)` builds optimality bottom-up; BFS offers an alternative "shortest path" interpretation where each coin denomination is an edge.
