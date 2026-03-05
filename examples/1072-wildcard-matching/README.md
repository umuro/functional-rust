# 1072: Wildcard Matching

**Difficulty:** Advanced
**Category:** Dynamic Programming
**Concept:** Match strings against patterns with `?` (any single char) and `*` (any sequence including empty) using DP
**Key Insight:** Unlike regex `*` (which couples with preceding char), wildcard `*` is standalone and matches any sequence. This makes the DP simpler: `*` means `dp[i-1][j]` (consume one char) or `dp[i][j-1]` (skip `*`). The greedy two-pointer approach is O(m+n) on average.
