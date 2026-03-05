# 1069: Graph Coloring

**Difficulty:** Advanced
**Category:** Backtracking
**Concept:** Assign colors to graph vertices so no two adjacent vertices share a color, using minimum colors
**Key Insight:** Backtracking tries each color for each vertex, pruning when a conflict is detected. The chromatic number (minimum colors needed) is NP-hard to find, but backtracking with pruning works for small graphs.
