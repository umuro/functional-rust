# 1063: Sudoku Solver

**Difficulty:** Advanced
**Category:** Backtracking
**Concept:** Solve a 9×9 Sudoku puzzle using backtracking with constraint propagation
**Key Insight:** Pre-computing constraint sets (which numbers are used in each row, column, and 3×3 box) reduces the validation from O(27) per check to O(1), dramatically speeding up the backtracking.
