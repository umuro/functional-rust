# 1074: Egg Drop

**Difficulty:** Advanced
**Category:** Dynamic Programming / Binary Search
**Concept:** Find minimum number of trials to determine the critical floor with k eggs and n floors
**Key Insight:** The basic O(kn²) DP can be optimized with binary search on the drop floor (reducing to O(kn log n)). The optimal O(kn) approach inverts the question: "given t trials and k eggs, how many floors can we check?"
