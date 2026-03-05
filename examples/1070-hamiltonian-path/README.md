# 1070: Hamiltonian Path

**Difficulty:** Advanced
**Category:** Backtracking / Bitmask DP
**Concept:** Find a path that visits every vertex exactly once using backtracking, with bitmask DP for existence checking
**Key Insight:** Backtracking explores O(n!) paths in the worst case. Bitmask DP reduces this to O(2^n × n^2) by tracking which subsets of vertices can be reached ending at each vertex — exponential but much faster than factorial.
