📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1056-edit-distance)**

---

# 1056: Edit Distance (Levenshtein)

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Minimum insertions, deletions, and substitutions to transform one string into another
**Key Insight:** Three operations (insert, delete, replace) map to three neighboring cells in the DP table — `dp[i-1][j]`, `dp[i][j-1]`, and `dp[i-1][j-1]` — making the recurrence a min of three choices plus a diagonal match shortcut.
