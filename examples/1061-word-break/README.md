# 1061: Word Break

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Determine if a string can be segmented into dictionary words using boolean DP with HashSet lookup
**Key Insight:** `dp[i] = true` if there exists some `j < i` where `dp[j]` is true and `s[j..i]` is in the dictionary — each position checks all possible last-word boundaries.
