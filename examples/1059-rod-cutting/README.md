📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1059-rod-cutting)**

---

# 1059: Rod Cutting

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Maximize revenue from cutting a rod into pieces with given prices per length
**Key Insight:** Rod cutting is an unbounded knapsack variant — each length can be used multiple times. The recurrence `dp[i] = max(price[j] + dp[i-j])` for all valid j tries every first-cut position.
