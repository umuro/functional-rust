📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1070-hamiltonian-path)**

---

# 1070-hamiltonian-path — Hamiltonian Path
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A Hamiltonian path visits every vertex of a graph exactly once. Unlike Euler paths (which traverse every edge once), Hamiltonian paths are NP-complete — no polynomial algorithm is known. The Traveling Salesman Problem (TSP) is a weighted Hamiltonian path problem and is one of the most famous NP-hard problems in computer science.

The backtracking approach tries to extend the path one vertex at a time, backtracking when a dead end is reached. For small graphs (up to ~20 vertices), this is practical.

## Learning Outcomes

- Implement Hamiltonian path finding via backtracking
- Use a visited array to enforce the "exactly once" constraint
- Understand why Hamiltonian path is NP-complete
- Contrast with Eulerian path (polynomial time via Hierholzer's algorithm)
- Connect to TSP and its approximation algorithms

## Rust Application

`src/lib.rs` uses an adjacency matrix. `hamiltonian_path` fixes vertex 0 as the start, marks it visited, and calls `solve(pos=1)`. At each position, it tries all unvisited vertices adjacent to the current last vertex. If all vertices are placed (`pos == n`), success. The path is returned as a `Vec<usize>` of vertex indices.

To find Hamiltonian circuits (start = end), add a check at `pos == n` that the last vertex is adjacent to vertex 0.

## OCaml Approach

```ocaml
let hamiltonian_path adj =
  let n = Array.length adj in
  let path = Array.make n 0 in
  let visited = Array.make n false in
  visited.(0) <- true;
  let rec solve pos =
    if pos = n then Some (Array.to_list path)
    else
      let result = ref None in
      for v = 0 to n - 1 do
        if !result = None && not visited.(v) && adj.(path.(pos-1)).(v) = 1 then begin
          path.(pos) <- v; visited.(v) <- true;
          result := solve (pos + 1);
          if !result = None then visited.(v) <- false
        end
      done;
      !result
  in
  solve 1
```

Structurally identical. OCaml's `ref result` manages early exit; Rust returns from the inner function.

## Key Differences

1. **Early exit**: Rust's recursive approach returns `true`/`false`, naturally exiting; OCaml uses a `ref` to propagate success upward.
2. **NP-completeness**: Both implementations are exponential in the worst case — no polynomial alternative exists.
3. **Bitmask DP**: For graphs up to ~20 vertices, bitmask DP gives O(2^n × n^2) time and O(2^n × n) space — better than backtracking's O(n!).
4. **TSP connection**: Hamiltonian path + edge weights = TSP; the Held-Karp bitmask DP algorithm solves TSP in O(2^n × n^2).

## Exercises

1. Implement `hamiltonian_circuit` that additionally requires the last vertex to be adjacent to the first (completing the cycle).
2. Add the bitmask DP optimization for graphs up to 20 vertices: `dp[mask][v]` = true if the subset encoded by `mask` has a Hamiltonian path ending at vertex `v`.
3. Write `tsp_exact(dist: &Vec<Vec<f64>>) -> (f64, Vec<usize>)` using Held-Karp DP for the minimum-cost Hamiltonian circuit.
