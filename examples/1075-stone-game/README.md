📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1075-stone-game)**

---

# 1075: Stone Game

**Difficulty:** Advanced
**Category:** Dynamic Programming (Minimax / Game Theory)
**Concept:** Two players take turns picking from ends of a pile array; determine if first player wins with optimal play
**Key Insight:** `dp[i][j]` stores the score *difference* (current player minus opponent) for the subarray `piles[i..j]`. The mathematical insight: with even piles, first player can always force a win by choosing all odd-indexed or all even-indexed piles.
