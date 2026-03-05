📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1065-combinations-sum)**

---

# 1065: Combination Sum

**Difficulty:** Intermediate
**Category:** Backtracking
**Concept:** Find all unique combinations of candidates that sum to a target, where each number can be reused
**Key Insight:** Starting each recursion at index `i` (not `i+1`) allows reuse; sorting enables early pruning when `candidates[i] > remaining`. The variant with `i+1` and duplicate skipping solves Combination Sum II.
