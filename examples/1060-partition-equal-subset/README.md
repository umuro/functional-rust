📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1060-partition-equal-subset)**

---

# 1060: Partition Equal Subset Sum

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Determine if an array can be partitioned into two subsets with equal sum — a boolean 0/1 knapsack variant
**Key Insight:** If total sum is odd, partition is impossible. Otherwise, find if a subset sums to `total/2` using a 1D boolean DP array with reverse iteration (same trick as 0/1 knapsack).
