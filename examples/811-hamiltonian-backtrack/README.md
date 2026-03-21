📖 **[View on hightechmind.io →](https://hightechmind.io/rust/811-hamiltonian-backtrack)**

---

# 811-hamiltonian-backtrack — Hamiltonian Path (Backtracking)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A Hamiltonian path visits every vertex exactly once. Unlike Eulerian path (efficient, O(V+E)), Hamiltonian path is NP-complete — no polynomial algorithm is known. The backtracking approach prunes the search tree by abandoning partial paths that cannot possibly complete. It is the basis for TSP (traveling salesman) solvers and appears in puzzle solving (knight's tour, Sudoku) and genome sequencing (alternative to Eulerian for short reads).

## Learning Outcomes

- Implement backtracking for Hamiltonian path: try each unvisited neighbor, recurse, undo on failure
- Use a `visited: Vec<bool>` array to track which vertices are in the current path
- Understand why backtracking is still exponential worst-case but practical for small graphs
- Apply pruning heuristics: Warnsdorff's rule for knight's tour
- See why Hamiltonian is NP-complete while Eulerian is polynomial

## Rust Application

`hamiltonian_path(n, edges)` builds an adjacency matrix. Starts at vertex 0 with `visited[0] = true`. `backtrack` tries each unvisited neighbor `next`: if `adj[last][next]`, set `visited[next]`, push to path, recurse; if `path.len() == n`, return true. Backtrack by popping and clearing visited. Returns the first complete path found, or `None`.

## OCaml Approach

OCaml implements backtracking with `let rec backtrack path vis = ...`. OCaml's recursive style makes backtracking natural: `if success then Some path else List.fold_left try_next None neighbors`. The `exception Found of int list` pattern enables early termination when a path is found. Heuristics like Warnsdorff's rule (choose vertex with fewest onward moves first) dramatically speed up knight's tour solutions.

## Key Differences

1. **Backtracking style**: Rust's mutable `path` and `visited` with explicit push/pop is imperative; OCaml's recursive approach with immutable lists is more idiomatic but allocates more.
2. **Early termination**: Rust returns `true` immediately on finding a path; OCaml uses exceptions or `Option` for early return through the recursion stack.
3. **NP-completeness**: Both languages face the same exponential worst case; pruning heuristics matter more than language choice.
4. **Knight's tour**: The knight's tour (finding a Hamiltonian path on a chessboard for a knight) is solvable in O(n²) using Warnsdorff's heuristic — a practical exception to the NP-hardness.

## Exercises

1. Implement Warnsdorff's heuristic for the knight's tour: always move to the square with the fewest onward moves. Verify it solves 8×8 chessboard instantly.
2. Add pruning: if any unvisited vertex has 0 remaining unvisited neighbors before the path is complete, immediately backtrack.
3. Implement the Hamiltonian cycle variant (returns to start) by adding a check at `path.len() == n` that `adj[last][start]` is true.
