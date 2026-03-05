# 1058: Longest Increasing Subsequence

**Difficulty:** Intermediate-Advanced
**Category:** Dynamic Programming / Binary Search
**Concept:** Find the length of the longest strictly increasing subsequence using patience sorting
**Key Insight:** Maintain a `tails` array where `tails[i]` is the smallest tail element of all increasing subsequences of length `i+1` — binary search determines where each new element fits, achieving O(n log n).
