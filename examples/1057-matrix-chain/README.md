📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1057-matrix-chain)**

---

# 1057: Matrix Chain Multiplication

**Difficulty:** Advanced
**Category:** Dynamic Programming
**Concept:** Find optimal parenthesization of matrix chain to minimize scalar multiplications
**Key Insight:** This is interval DP — `dp[i][j]` represents the minimum cost to multiply matrices i through j, trying every possible split point k between them.
