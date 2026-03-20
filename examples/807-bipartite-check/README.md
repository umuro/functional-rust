📖 **[View on hightechmind.io →](https://hightechmind.io/rust/807-bipartite-check)**

---

# 807-bipartite-check — Bipartite Check
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A bipartite graph can be 2-colored: vertices split into two sets where all edges go between sets (no edges within a set). BFS-based 2-coloring checks bipartiteness in O(V+E). Applications: job matching (workers and jobs are two sets), recommendation systems (users and items), scheduling (tasks and time slots), and the fundamental theorem that a graph is bipartite if and only if it contains no odd-length cycles.

## Learning Outcomes

- Implement BFS 2-coloring: assign alternating colors to adjacent vertices
- Detect non-bipartiteness: same color on adjacent vertices during BFS
- Handle disconnected graphs: run BFS from every uncolored vertex
- Understand the bipartite-iff-no-odd-cycles theorem
- Apply to matching problems (König's theorem: max matching = min vertex cover in bipartite graphs)

## Rust Application

`is_bipartite(n, edges)` builds an undirected adjacency list. BFS from each unvisited vertex assigns `color[start] = true`. For each neighbor `v` of `u`: if uncolored, assign opposite color and enqueue; if same color as `u`, return false (odd cycle). Tests: even cycle (4 nodes, bipartite) and triangle (3 nodes, not bipartite).

## OCaml Approach

OCaml implements BFS with `Queue.t` and a `color: bool option array`. The `Queue.add` / `Queue.pop` pattern drives the BFS. OCaml's `Option.map` applies color flipping functionally. The `Ocamlgraph` library provides `Coloring.check_bipartite`. OCaml's `for v in adj.(u) do ...` is idiomatic for adjacency list traversal.

## Key Differences

1. **BFS queue**: Rust uses `VecDeque<usize>` (FIFO); OCaml uses `Queue.t` — equivalent data structures.
2. **Color representation**: Rust uses `Option<bool>` (uncolored / colored); OCaml uses `int option` or `bool option` similarly.
3. **Disconnected graphs**: Both check all vertices to handle disconnected graphs — the `for v in 0..n` outer loop is identical.
4. **Matching connection**: Bipartite check is a prerequisite for maximum bipartite matching (Hopcroft-Karp); both languages use the same foundation.

## Exercises

1. Implement `bipartite_partition(n, edges) -> Option<(Vec<usize>, Vec<usize>)>` that returns the two vertex sets if bipartite, or `None` if not.
2. Find the shortest odd cycle in a non-bipartite graph using BFS level tracking — the cycle length is the graph's "odd girth."
3. Implement maximum bipartite matching using the augmenting path algorithm (Hungarian / Hopcroft-Karp), building on the bipartite check.
