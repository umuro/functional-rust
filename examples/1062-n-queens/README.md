# 1062: N-Queens

**Difficulty:** Advanced
**Category:** Backtracking
**Concept:** Place N queens on an N×N chessboard so no two attack each other
**Key Insight:** Three constraint arrays (columns, main diagonals, anti-diagonals) enable O(1) conflict checking. The bitmask variant packs all three constraints into integers for the fastest solution.
