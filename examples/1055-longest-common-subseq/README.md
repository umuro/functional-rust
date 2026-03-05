📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1055-longest-common-subseq)**

---

# 1055: Longest Common Subsequence

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Find the longest subsequence common to two strings using 2D DP table with backtracking for reconstruction
**Key Insight:** The DP table stores lengths, but the actual subsequence is recovered by backtracking from `dp[m][n]` — following diagonal moves (match) vs horizontal/vertical moves (skip).
