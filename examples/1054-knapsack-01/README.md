# 1054: 0/1 Knapsack

**Difficulty:** Intermediate
**Category:** Dynamic Programming
**Concept:** Maximize value selection with weight constraints using 2D DP table, with 1D space optimization
**Key Insight:** The reverse iteration trick in the 1D optimization ensures each item is used at most once — iterating `w` from `capacity` down to `weight[i]` prevents double-counting.
