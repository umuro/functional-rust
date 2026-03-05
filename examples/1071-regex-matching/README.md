# 1071: Regex Matching

**Difficulty:** Advanced
**Category:** Dynamic Programming
**Concept:** Match strings against patterns with `.` (any char) and `*` (zero or more of preceding) using DP
**Key Insight:** The `*` operator is the tricky part — it couples with the preceding character. `dp[i][j]` depends on: (1) zero matches of `x*` → `dp[i][j-2]`, or (2) one+ matches if `x` matches current char → `dp[i-1][j]`.
